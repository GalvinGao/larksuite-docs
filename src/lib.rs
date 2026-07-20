#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result, bail, ensure};
use chrono::{DateTime, SecondsFormat, Utc};
use futures::{StreamExt, stream};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const DIRECTORY_URL: &str = "https://open.larksuite.com/api/tools/docment/directory_list";
const DETAIL_URL: &str = "https://open.larksuite.com/document_portal/v1/document/get_detail";
const PUBLIC_DOCUMENT_BASE_URL: &str = "https://open.larksuite.com/document";
const LEGACY_INTERNAL_PATH: &str = "/ssl:ttdoc/";
const DOCUMENT_INTERNAL_PATH: &str = "/document/";

#[derive(Clone, Copy)]
struct Fence {
    marker: u8,
    length: usize,
}

#[derive(Debug)]
pub struct FetchOptions {
    pub output: PathBuf,
    pub path: Option<String>,
    pub concurrency: usize,
    pub batch_size: usize,
    pub retries: u8,
}

#[derive(Debug, Deserialize)]
struct ApiEnvelope {
    code: i64,
    #[serde(default)]
    msg: String,
    data: Value,
}

#[derive(Debug, Deserialize)]
struct DirectoryData {
    items: Vec<TreeNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TreeNode {
    full_path: String,
    name: String,
    #[serde(rename = "type")]
    node_type: String,
    #[serde(default)]
    items: Vec<Self>,
}

#[derive(Debug)]
struct DocumentJob {
    full_path: String,
    breadcrumb: Vec<String>,
    relative_path: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentDetail {
    id: String,
    directory_id: String,
    full_path: String,
    #[serde(rename = "type")]
    document_type: String,
    #[serde(default)]
    title: String,
    name: String,
    update_time: i64,
    content: String,
}

#[derive(Debug, Serialize)]
struct FrontMatter<'a> {
    document_id: &'a str,
    directory_id: &'a str,
    title: &'a str,
    full_path: &'a str,
    breadcrumb: &'a [String],
    document_type: &'a str,
    updated_at: String,
    source_url: String,
}

/// Fetches either the complete public document tree or one requested path.
///
/// # Errors
///
/// Returns an error when an API request fails, a response has an unexpected shape, a document
/// path is unsafe, metadata cannot be serialized, or a generated file cannot be written.
pub async fn fetch(options: FetchOptions) -> Result<()> {
    ensure!(
        options.concurrency > 0,
        "concurrency must be greater than zero"
    );
    ensure!(
        options.batch_size > 0,
        "batch size must be greater than zero"
    );

    let client = Client::builder()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .timeout(Duration::from_secs(30))
        .build()
        .context("failed to build the HTTP client")?;

    let mut jobs = fetch_document_jobs(&client, options.retries).await?;
    ensure!(!jobs.is_empty(), "directory list contained no documents");
    ensure_unique_output_paths(&jobs)?;

    if let Some(path) = options.path {
        let detail = fetch_document_detail(&client, &path, options.retries).await?;
        let job = jobs
            .iter()
            .find(|job| job.full_path == detail.full_path)
            .with_context(|| {
                format!(
                    "the canonical document path {} is absent from the directory tree",
                    detail.full_path
                )
            })?;
        let saved_path = store_document(&detail, job, &options.output).await?;
        eprintln!("saved {}", saved_path.display());
        return Ok(());
    }

    jobs.sort_unstable_by(|left, right| left.full_path.cmp(&right.full_path));
    let total = jobs.len();
    eprintln!(
        "fetching {total} documents with concurrency {} in batches of {}",
        options.concurrency, options.batch_size
    );

    let output = &options.output;
    let mut batches = stream::iter(jobs)
        .map(|job| {
            let client = client.clone();
            async move {
                let full_path = job.full_path.clone();
                let result = fetch_and_store_document(&client, &job, output, options.retries).await;
                (full_path, result)
            }
        })
        .buffer_unordered(options.concurrency)
        .chunks(options.batch_size);
    let mut completed = 0_usize;
    let mut failure_count = 0_usize;
    let mut failure_samples = Vec::new();

    while let Some(batch) = batches.next().await {
        completed += batch.len();
        for (path, result) in batch {
            if let Err(error) = result {
                failure_count += 1;
                if failure_samples.len() < 20 {
                    failure_samples.push((path, error));
                }
            }
        }
        eprintln!("fetched {completed}/{total}");
    }

    if failure_count > 0 {
        for (path, error) in &failure_samples {
            eprintln!("failed {path}: {error:#}");
        }
        if failure_count > failure_samples.len() {
            eprintln!(
                "... and {} more failures",
                failure_count - failure_samples.len()
            );
        }
        bail!("{failure_count} of {total} documents failed");
    }

    eprintln!(
        "saved all {total} documents under {}",
        options.output.display()
    );
    Ok(())
}

async fn fetch_document_jobs(client: &Client, retries: u8) -> Result<Vec<DocumentJob>> {
    let envelope = fetch_envelope(client, DIRECTORY_URL, None, retries).await?;
    ensure_api_success(&envelope, "directory list")?;
    let directory: DirectoryData = serde_json::from_value(envelope.data)
        .context("directory list returned an unexpected data shape")?;
    let mut jobs = Vec::new();
    collect_document_jobs(directory.items, &mut Vec::new(), &mut Vec::new(), &mut jobs);
    Ok(jobs)
}

async fn fetch_and_store_document(
    client: &Client,
    job: &DocumentJob,
    output: &Path,
    retries: u8,
) -> Result<PathBuf> {
    let detail = fetch_document_detail(client, &job.full_path, retries).await?;
    store_document(&detail, job, output).await
}

async fn fetch_document_detail(
    client: &Client,
    requested_path: &str,
    retries: u8,
) -> Result<DocumentDetail> {
    let envelope = fetch_envelope(client, DETAIL_URL, Some(requested_path), retries).await?;
    ensure_api_success(&envelope, requested_path)?;
    let detail: DocumentDetail = serde_json::from_value(envelope.data)
        .with_context(|| format!("{requested_path} returned an unexpected data shape"))?;
    Ok(detail)
}

async fn store_document(
    detail: &DocumentDetail,
    job: &DocumentJob,
    output: &Path,
) -> Result<PathBuf> {
    let destination = output.join(&job.relative_path);
    let markdown = render_markdown(detail, &job.breadcrumb)?;
    write_atomically(&destination, markdown.as_bytes()).await?;
    Ok(destination)
}

async fn fetch_envelope(
    client: &Client,
    url: &str,
    full_path: Option<&str>,
    retries: u8,
) -> Result<ApiEnvelope> {
    let mut last_error = String::new();

    for attempt in 0..=retries {
        let mut request = client.get(url);
        if let Some(full_path) = full_path {
            request = request.query(&[("fullPath", full_path)]);
        }

        match request.send().await {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    match response.json::<ApiEnvelope>().await {
                        Ok(envelope) => return Ok(envelope),
                        Err(error) => last_error = format!("invalid JSON response: {error}"),
                    }
                } else {
                    let retryable = is_retryable_status(status);
                    let body = response.text().await.unwrap_or_default();
                    last_error = format!("HTTP {status}: {}", body.trim());
                    if !retryable {
                        bail!("request to {url} failed: {last_error}");
                    }
                }
            }
            Err(error) => last_error = error.to_string(),
        }

        if attempt < retries {
            tokio::time::sleep(retry_delay(attempt)).await;
        }
    }

    bail!(
        "request to {url} failed after {} attempts: {last_error}",
        u16::from(retries) + 1
    )
}

fn ensure_api_success(envelope: &ApiEnvelope, context: &str) -> Result<()> {
    ensure!(
        envelope.code == 0,
        "Lark API rejected {context} with code {}: {}",
        envelope.code,
        envelope.msg
    );
    Ok(())
}

fn is_retryable_status(status: StatusCode) -> bool {
    status.as_u16() == 429 || status.is_server_error()
}

fn retry_delay(attempt: u8) -> Duration {
    let exponent = u32::from(attempt.min(5));
    Duration::from_millis(250 * 2_u64.pow(exponent))
}

fn collect_document_jobs(
    siblings: Vec<TreeNode>,
    breadcrumb: &mut Vec<String>,
    slug_breadcrumb: &mut Vec<String>,
    jobs: &mut Vec<DocumentJob>,
) {
    let slugs = sibling_slugs(&siblings);
    for (node, slug) in siblings.into_iter().zip(slugs) {
        let label = clean_breadcrumb_label(&node.name);
        breadcrumb.push(label);
        slug_breadcrumb.push(slug);

        if node.node_type == "DocumentType" {
            let mut relative_path = slug_breadcrumb.iter().collect::<PathBuf>();
            relative_path.as_mut_os_string().push(".md");
            jobs.push(DocumentJob {
                full_path: node.full_path.clone(),
                breadcrumb: breadcrumb.clone(),
                relative_path,
            });
        }

        collect_document_jobs(node.items, breadcrumb, slug_breadcrumb, jobs);
        breadcrumb.pop();
        slug_breadcrumb.pop();
    }
}

fn sibling_slugs(nodes: &[TreeNode]) -> Vec<String> {
    let base_slugs = nodes
        .iter()
        .map(|node| slugify(&node.name))
        .collect::<Vec<_>>();
    let mut resolved = base_slugs.clone();
    let mut groups = HashMap::<(String, String), Vec<usize>>::new();

    for (index, node) in nodes.iter().enumerate() {
        groups
            .entry((node.node_type.clone(), base_slugs[index].clone()))
            .or_default()
            .push(index);
    }

    for ((_, base_slug), indices) in groups {
        if indices.len() == 1 {
            continue;
        }

        let candidates = indices
            .iter()
            .map(|index| source_tail_slug(&nodes[*index].full_path))
            .collect::<Vec<_>>();
        let unique_candidates = candidates.iter().collect::<HashSet<_>>().len() == candidates.len();

        if unique_candidates {
            for (index, candidate) in indices.into_iter().zip(candidates) {
                resolved[index] = candidate;
            }
        } else {
            for (offset, index) in indices.into_iter().enumerate() {
                resolved[index] = if offset == 0 {
                    base_slug.clone()
                } else {
                    format!("{base_slug}-{}", offset + 1)
                };
            }
        }
    }

    resolved
}

fn source_tail_slug(full_path: &str) -> String {
    full_path
        .split('/')
        .rfind(|segment| !segment.is_empty())
        .map_or_else(|| "untitled".to_owned(), slugify)
}

fn clean_breadcrumb_label(name: &str) -> String {
    name.chars()
        .filter(|character| !matches!(character, '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}'))
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn slugify(name: &str) -> String {
    let label = clean_breadcrumb_label(name);
    let mut slug = String::with_capacity(label.len());
    let mut separator_pending = false;

    for character in label.chars() {
        if character.is_alphanumeric() {
            if separator_pending && !slug.is_empty() {
                slug.push('-');
            }
            slug.extend(character.to_lowercase());
            separator_pending = false;
        } else if !slug.is_empty() {
            separator_pending = true;
        }
    }

    if slug.is_empty() {
        "untitled".to_owned()
    } else {
        slug
    }
}

fn ensure_unique_output_paths(jobs: &[DocumentJob]) -> Result<()> {
    let mut destinations = HashSet::with_capacity(jobs.len());
    let mut full_paths = HashSet::with_capacity(jobs.len());
    for job in jobs {
        ensure!(
            destinations.insert(&job.relative_path),
            "multiple documents map to {}",
            job.relative_path.display()
        );
        ensure!(
            full_paths.insert(&job.full_path),
            "duplicate document path in directory tree: {}",
            job.full_path
        );
    }
    Ok(())
}

fn render_markdown(detail: &DocumentDetail, breadcrumb: &[String]) -> Result<String> {
    let updated_at = DateTime::<Utc>::from_timestamp_millis(detail.update_time)
        .with_context(|| format!("invalid updateTime value: {}", detail.update_time))?
        .to_rfc3339_opts(SecondsFormat::Secs, true);
    let title = if detail.title.trim().is_empty() {
        detail.name.as_str()
    } else {
        detail.title.as_str()
    };
    let frontmatter = FrontMatter {
        document_id: &detail.id,
        directory_id: &detail.directory_id,
        title,
        full_path: &detail.full_path,
        breadcrumb,
        document_type: &detail.document_type,
        updated_at,
        source_url: format!("{PUBLIC_DOCUMENT_BASE_URL}{}", detail.full_path),
    };
    let yaml = serde_yaml_ng::to_string(&frontmatter).context("failed to serialize frontmatter")?;

    let content = normalize_internal_links(&detail.content);
    let mut markdown = format!("---\n{yaml}---\n\n{content}");
    if !markdown.ends_with('\n') {
        markdown.push('\n');
    }
    Ok(markdown)
}

fn normalize_internal_links(markdown: &str) -> String {
    let mut normalized = String::with_capacity(markdown.len());
    let mut fence = None;

    for line in markdown.split_inclusive('\n') {
        if let Some(active_fence) = fence {
            normalized.push_str(line);
            if is_closing_fence(line, active_fence) {
                fence = None;
            }
            continue;
        }

        if let Some(opening_fence) = opening_fence(line) {
            normalized.push_str(line);
            fence = Some(opening_fence);
            continue;
        }

        let mut inline_code_delimiter = None;
        normalize_inline_links(line, &mut inline_code_delimiter, &mut normalized);
    }

    normalized
}

fn opening_fence(line: &str) -> Option<Fence> {
    let (marker, length, remainder) = fence_marker(line)?;
    if length < 3 || marker == b'`' && remainder.contains('`') {
        return None;
    }
    Some(Fence { marker, length })
}

fn is_closing_fence(line: &str, fence: Fence) -> bool {
    fence_marker(line).is_some_and(|(marker, length, remainder)| {
        let remainder = remainder.trim();
        marker == fence.marker
            && length >= fence.length
            && (remainder.is_empty() || remainder.starts_with("</"))
    })
}

fn fence_marker(line: &str) -> Option<(u8, usize, &str)> {
    let line = line.trim_end_matches(['\r', '\n']);
    let bytes = line.as_bytes();
    let indentation = bytes
        .iter()
        .take_while(|byte| matches!(**byte, b' ' | b'\t'))
        .count();

    let marker = *bytes.get(indentation)?;
    if marker != b'`' && marker != b'~' {
        return None;
    }
    let length = bytes[indentation..]
        .iter()
        .take_while(|byte| **byte == marker)
        .count();
    Some((marker, length, &line[indentation + length..]))
}

fn normalize_inline_links(
    line: &str,
    inline_code_delimiter: &mut Option<usize>,
    normalized: &mut String,
) {
    let bytes = line.as_bytes();
    let legacy = LEGACY_INTERNAL_PATH.as_bytes();
    let mut copied_until = 0;
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'`' {
            let run_length = bytes[index..]
                .iter()
                .take_while(|byte| **byte == b'`')
                .count();
            match *inline_code_delimiter {
                Some(opening_length) if opening_length == run_length => {
                    *inline_code_delimiter = None;
                }
                None => *inline_code_delimiter = Some(run_length),
                Some(_) => {}
            }
            index += run_length;
        } else if inline_code_delimiter.is_none() && bytes[index..].starts_with(legacy) {
            normalized.push_str(&line[copied_until..index]);
            normalized.push_str(DOCUMENT_INTERNAL_PATH);
            index += legacy.len();
            copied_until = index;
        } else {
            index += 1;
        }
    }

    normalized.push_str(&line[copied_until..]);
}

async fn write_atomically(destination: &Path, contents: &[u8]) -> Result<()> {
    let parent = destination
        .parent()
        .context("generated document path has no parent")?;
    tokio::fs::create_dir_all(parent)
        .await
        .with_context(|| format!("failed to create {}", parent.display()))?;

    let mut temporary = destination.as_os_str().to_owned();
    temporary.push(".tmp");
    let temporary = PathBuf::from(temporary);
    tokio::fs::write(&temporary, contents)
        .await
        .with_context(|| format!("failed to write {}", temporary.display()))?;
    tokio::fs::rename(&temporary, destination)
        .await
        .with_context(|| format!("failed to move {} into place", destination.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_a_meaningful_breadcrumb_path() {
        let tree = vec![TreeNode {
            full_path: "/guides".into(),
            name: "Developer Guides".into(),
            node_type: "DirectoryType".into(),
            items: vec![TreeNode {
                full_path: "/guides/platform".into(),
                name: "Platform Introduction".into(),
                node_type: "DirectoryType".into(),
                items: vec![TreeNode {
                    full_path: "/home/intro".into(),
                    name: "Lark Open Capabilities Introduction".into(),
                    node_type: "DocumentType".into(),
                    items: Vec::new(),
                }],
            }],
        }];
        let mut jobs = Vec::new();

        collect_document_jobs(tree, &mut Vec::new(), &mut Vec::new(), &mut jobs);

        assert_eq!(jobs.len(), 1);
        assert_eq!(
            jobs[0].relative_path,
            Path::new(
                "developer-guides/platform-introduction/lark-open-capabilities-introduction.md"
            )
        );
        assert_eq!(
            jobs[0].breadcrumb,
            [
                "Developer Guides",
                "Platform Introduction",
                "Lark Open Capabilities Introduction"
            ]
        );
    }

    #[test]
    fn disambiguates_duplicate_sibling_names_with_meaningful_source_tails() {
        let nodes = vec![
            TreeNode {
                full_path: "/reference/app-workflow".into(),
                name: "Workflow".into(),
                node_type: "DirectoryType".into(),
                items: Vec::new(),
            },
            TreeNode {
                full_path: "/reference/app-block_workflow".into(),
                name: "Workflow".into(),
                node_type: "DirectoryType".into(),
                items: Vec::new(),
            },
        ];

        assert_eq!(
            sibling_slugs(&nodes),
            ["app-workflow", "app-block-workflow"]
        );
    }

    #[test]
    fn slugifies_labels_without_opaque_or_unsafe_path_characters() {
        assert_eq!(
            slugify("  Message Field Shortcuts (“+”)  "),
            "message-field-shortcuts"
        );
        assert_eq!(
            slugify("Todos daily reminder\u{200B}"),
            "todos-daily-reminder"
        );
    }

    #[test]
    fn renders_selected_metadata_and_readable_timestamp() {
        let detail = DocumentDetail {
            id: "7226917242922762246".into(),
            directory_id: "7260031291679473669".into(),
            full_path: "/home/intro".into(),
            document_type: "GuideDocumentType".into(),
            title: String::new(),
            name: "Overview".into(),
            update_time: 1_694_597_979_000,
            content: "# Hello".into(),
        };

        let breadcrumb = vec!["Developer Guides".into(), "Overview".into()];
        let markdown = render_markdown(&detail, &breadcrumb).expect("document should render");

        assert!(markdown.starts_with("---\n"));
        assert!(markdown.contains("title: Overview"));
        assert!(markdown.contains("updated_at: 2023-09-13T09:39:39Z"));
        assert!(markdown.contains("breadcrumb:\n- Developer Guides\n- Overview"));
        assert!(markdown.contains("\n---\n\n# Hello\n"));
        assert!(!markdown.contains("bizScope"));
        assert!(!markdown.contains("allVisible"));
    }

    #[test]
    fn converts_legacy_internal_markdown_links() {
        let markdown = "[Lark机器人](/ssl:ttdoc/uAjLw4CM/bot-v3/bot-overview)";

        let normalized = normalize_internal_links(markdown);

        assert_eq!(
            normalized,
            "[Lark机器人](/document/uAjLw4CM/bot-v3/bot-overview)"
        );
    }

    #[test]
    fn converts_internal_paths_in_markdown_and_html_but_not_code() {
        let markdown = concat!(
            "[guide](/ssl:ttdoc/home/guide)\n",
            "<a href=\"/ssl:ttdoc/home/guide\">guide</a>\n",
            "`[code](/ssl:ttdoc/home/code)`\n",
            "```markdown\n",
            "[code](/ssl:ttdoc/home/code)\n",
            "```\n",
            "~~~html\n",
            "<a href=\"/ssl:ttdoc/home/code\">code</a>\n",
            "~~~\n",
        );

        let normalized = normalize_internal_links(markdown);

        assert!(normalized.contains("[guide](/document/home/guide)"));
        assert!(normalized.contains("href=\"/document/home/guide\""));
        assert!(normalized.contains("`[code](/ssl:ttdoc/home/code)`"));
        assert_eq!(normalized.matches("/ssl:ttdoc/").count(), 3);
    }

    #[test]
    fn unmatched_backticks_do_not_hide_links_on_following_lines() {
        let markdown = "`unmatched\n[guide](/ssl:ttdoc/home/guide)\n";

        let normalized = normalize_internal_links(markdown);

        assert!(normalized.contains("[guide](/document/home/guide)"));
    }

    #[test]
    fn triple_backtick_inline_code_does_not_open_a_fenced_block() {
        let markdown = "```inline value```\n[guide](/ssl:ttdoc/home/guide)\n";

        let normalized = normalize_internal_links(markdown);

        assert!(normalized.contains("[guide](/document/home/guide)"));
    }

    #[test]
    fn handles_indented_fences_and_lark_html_closing_tags() {
        let markdown = concat!(
            "    ```json\n",
            "    {\"link\": \"/ssl:ttdoc/home/code\"}\n",
            "    ```</md-td>\n",
            "[guide](/ssl:ttdoc/home/guide)\n",
        );

        let normalized = normalize_internal_links(markdown);

        assert!(normalized.contains("\"/ssl:ttdoc/home/code\""));
        assert!(normalized.contains("[guide](/document/home/guide)"));
        assert_eq!(normalized.matches("/ssl:ttdoc/").count(), 1);
    }

    #[test]
    fn handles_tab_indented_fences() {
        let markdown = concat!(
            "\t```javascript\n",
            "\tconst url = '/ssl:ttdoc/home/code';\n",
            "    ```\n",
            "[guide](/ssl:ttdoc/home/guide)\n",
        );

        let normalized = normalize_internal_links(markdown);

        assert!(normalized.contains("'/ssl:ttdoc/home/code'"));
        assert!(normalized.contains("[guide](/document/home/guide)"));
        assert_eq!(normalized.matches("/ssl:ttdoc/").count(), 1);
    }
}
