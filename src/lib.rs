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

struct TableRow {
    cells: Vec<String>,
    is_header: bool,
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
    let content = normalize_special_tables(&content);
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

fn normalize_special_tables(markdown: &str) -> String {
    let mut normalized = String::with_capacity(markdown.len());
    let mut cursor = 0;
    let mut fence = None;

    while cursor < markdown.len() {
        let line_end = markdown[cursor..]
            .find('\n')
            .map_or(markdown.len(), |offset| cursor + offset + 1);
        let line = &markdown[cursor..line_end];

        if let Some(active_fence) = fence {
            normalized.push_str(line);
            if is_closing_fence(line, active_fence) {
                fence = None;
            }
            cursor = line_end;
            continue;
        }

        if let Some(opening_fence) = opening_fence(line) {
            normalized.push_str(line);
            fence = Some(opening_fence);
            cursor = line_end;
            continue;
        }

        let Some(table_offset) = line.find("<md-table") else {
            normalized.push_str(line);
            cursor = line_end;
            continue;
        };
        let table_start = cursor + table_offset;
        let Some((consumed, table)) = parse_special_table(&markdown[table_start..]) else {
            normalized.push_str(line);
            cursor = line_end;
            continue;
        };

        normalized.push_str(&markdown[cursor..table_start]);
        let table_end = table_start + consumed;
        let wrapper_start = html_wrapper_start(&normalized);
        let wrapper_end = html_wrapper_end(markdown, table_end);
        if let (Some(wrapper_start), Some(wrapper_end)) = (wrapper_start, wrapper_end) {
            normalized.truncate(wrapper_start);
            cursor = wrapper_end;
        } else {
            cursor = table_end;
        }

        if !normalized.is_empty() && !normalized.ends_with("\n\n") {
            if !normalized.ends_with('\n') {
                normalized.push('\n');
            }
            normalized.push('\n');
        }
        normalized.push_str(&table);
    }

    normalized
}

fn parse_special_table(source: &str) -> Option<(usize, String)> {
    let opening_end = source.find('>')? + 1;
    let (closing, name) = parse_tag(&source[..opening_end])?;
    if closing || name != "md-table" {
        return None;
    }

    let mut rows = Vec::new();
    let mut table_depth = 1_usize;
    let mut in_header = false;
    let mut row_started = false;
    let mut row_is_header = false;
    let mut cells = Vec::new();
    let mut cell_start = None;
    let mut cursor = opening_end;

    while cursor < source.len() {
        let tag_start = source[cursor..].find('<').map(|offset| cursor + offset);
        let wrapper_end = find_html_wrapper_marker(source, cursor);
        if let Some(wrapper_end) = wrapper_end.filter(|wrapper_end| {
            table_depth == 1 && tag_start.is_none_or(|tag| *wrapper_end < tag)
        }) {
            finish_table_cell(source, wrapper_end, &mut cell_start, &mut cells);
            finish_table_row(&mut rows, &mut row_started, row_is_header, &mut cells);
            return render_markdown_table(rows).map(|table| (wrapper_end, table));
        }

        let Some(tag_start) = tag_start else {
            if table_depth == 1 && source[cursor..].trim().is_empty() {
                finish_table_cell(source, source.len(), &mut cell_start, &mut cells);
                finish_table_row(&mut rows, &mut row_started, row_is_header, &mut cells);
                return render_markdown_table(rows).map(|table| (source.len(), table));
            }
            return None;
        };
        let tag_end = tag_start + source[tag_start..].find('>')? + 1;
        let Some((closing, name)) = parse_tag(&source[tag_start..tag_end]) else {
            cursor = tag_end;
            continue;
        };

        if is_table_tag(name) {
            if closing {
                if table_depth > 1 {
                    table_depth -= 1;
                    cursor = tag_end;
                    continue;
                }

                finish_table_cell(source, tag_start, &mut cell_start, &mut cells);
                finish_table_row(&mut rows, &mut row_started, row_is_header, &mut cells);
                return render_markdown_table(rows).map(|table| (tag_end, table));
            }

            table_depth += 1;
            cursor = tag_end;
            continue;
        }

        if table_depth > 1 {
            cursor = tag_end;
            continue;
        }

        match (closing, name) {
            (false, "md-thead" | "thead") => in_header = true,
            (true, "md-thead" | "thead") => in_header = false,
            (false, "md-tr" | "tr") => {
                finish_table_cell(source, tag_start, &mut cell_start, &mut cells);
                finish_table_row(&mut rows, &mut row_started, row_is_header, &mut cells);
                row_started = true;
                row_is_header = in_header;
            }
            (true, "md-tr" | "tr") => {
                finish_table_cell(source, tag_start, &mut cell_start, &mut cells);
                finish_table_row(&mut rows, &mut row_started, row_is_header, &mut cells);
            }
            (false, "md-th" | "th" | "md-td" | "td") => {
                if !row_started {
                    row_started = true;
                    row_is_header = in_header;
                }
                if cell_start.is_some_and(|start| !source[start..tag_start].trim().is_empty()) {
                    finish_table_cell(source, tag_start, &mut cell_start, &mut cells);
                }
                cell_start = Some(tag_end);
            }
            (true, "md-th" | "th" | "md-td" | "td") => {
                finish_table_cell(source, tag_start, &mut cell_start, &mut cells);
            }
            _ => {}
        }

        cursor = tag_end;
    }

    None
}

fn parse_tag(tag: &str) -> Option<(bool, &str)> {
    let inside = tag.strip_prefix('<')?.strip_suffix('>')?.trim();
    let (closing, inside) = inside
        .strip_prefix('/')
        .map_or((false, inside), |rest| (true, rest.trim_start()));
    let name_end = inside
        .find(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
        .unwrap_or(inside.len());
    (name_end > 0).then_some((closing, &inside[..name_end]))
}

fn is_table_tag(name: &str) -> bool {
    matches!(name, "md-table" | "table")
}

fn finish_table_cell(
    source: &str,
    end: usize,
    cell_start: &mut Option<usize>,
    cells: &mut Vec<String>,
) {
    if let Some(start) = cell_start.take() {
        cells.push(normalize_table_cell(&source[start..end]));
    }
}

fn finish_table_row(
    rows: &mut Vec<TableRow>,
    row_started: &mut bool,
    is_header: bool,
    cells: &mut Vec<String>,
) {
    if *row_started && !cells.is_empty() {
        rows.push(TableRow {
            cells: std::mem::take(cells),
            is_header,
        });
    }
    *row_started = false;
}

fn normalize_table_cell(cell: &str) -> String {
    let mut normalized = String::new();
    let mut fence = None;
    let mut code = Vec::new();

    for line in cell.trim().lines() {
        if let Some(active_fence) = fence {
            if is_closing_fence(line, active_fence) {
                push_table_cell_part(&mut normalized, &format_code_cell(&code));
                code.clear();
                fence = None;
            } else {
                code.push(line.trim_end());
            }
            continue;
        }

        if let Some(opening_fence) = opening_fence(line) {
            fence = Some(opening_fence);
            continue;
        }

        let line = line.trim();
        if !line.is_empty() {
            push_table_cell_part(&mut normalized, line);
        }
    }

    if fence.is_some() {
        push_table_cell_part(&mut normalized, &format_code_cell(&code));
    }

    normalized.replace('|', "&#124;")
}

fn push_table_cell_part(cell: &mut String, part: &str) {
    if !cell.is_empty()
        && !cell.ends_with("<br>")
        && !cell.ends_with("<br/>")
        && !cell.ends_with("<br />")
    {
        cell.push_str("<br>");
    }
    cell.push_str(part);
}

fn format_code_cell(lines: &[&str]) -> String {
    let code = lines.join("\n");
    let code = code
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\n', "<br>");
    format!("<code>{code}</code>")
}

fn render_markdown_table(mut rows: Vec<TableRow>) -> Option<String> {
    let column_count = rows.iter().map(|row| row.cells.len()).max()?;
    if column_count == 0 {
        return None;
    }

    let header_index = rows.iter().position(|row| row.is_header);
    let header = header_index.map_or_else(
        || vec![String::new(); column_count],
        |index| rows.remove(index).cells,
    );
    let mut table = String::new();
    push_markdown_table_row(&mut table, &header, column_count);
    table.push('|');
    for _ in 0..column_count {
        table.push_str(" --- |");
    }
    table.push('\n');
    for row in rows {
        push_markdown_table_row(&mut table, &row.cells, column_count);
    }
    table.push('\n');
    Some(table)
}

fn push_markdown_table_row(table: &mut String, cells: &[String], column_count: usize) {
    table.push('|');
    for index in 0..column_count {
        table.push(' ');
        table.push_str(cells.get(index).map_or("", String::as_str));
        table.push_str(" |");
    }
    table.push('\n');
}

fn html_wrapper_start(markdown: &str) -> Option<usize> {
    let content_end = markdown.trim_end_matches(char::is_whitespace).len();
    let line_start = markdown[..content_end]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    (markdown[line_start..content_end].trim() == ":::html").then_some(line_start)
}

fn html_wrapper_end(markdown: &str, table_end: usize) -> Option<usize> {
    let marker_start =
        table_end + markdown[table_end..].find(|character: char| !character.is_whitespace())?;
    let line_end = markdown[marker_start..]
        .find('\n')
        .map_or(markdown.len(), |offset| marker_start + offset + 1);
    (markdown[marker_start..line_end].trim() == ":::").then_some(line_end)
}

fn find_html_wrapper_marker(markdown: &str, start: usize) -> Option<usize> {
    let mut line_start = start;
    while line_start < markdown.len() {
        let line_end = markdown[line_start..]
            .find('\n')
            .map_or(markdown.len(), |offset| line_start + offset + 1);
        if markdown[line_start..line_end].trim() == ":::" {
            return Some(line_start);
        }
        line_start = line_end;
    }
    None
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

    #[test]
    fn converts_lark_tables_to_native_markdown() {
        let markdown = concat!(
            "Before\n\n",
            ":::html\n",
            "<md-table>\n",
            "<md-thead><md-tr>",
            "<md-th style=\"width: 20%;\">Name</md-th>",
            "<md-th>Details</md-th>",
            "</md-tr></md-thead>\n",
            "<md-tbody><md-tr>",
            "<md-td>alpha</md-td>",
            "<md-td>first line\n- second | value</md-td>",
            "</md-tr></md-tbody>\n",
            "</md-table>\n",
            ":::\n\n",
            "After\n",
        );

        let normalized = normalize_special_tables(markdown);

        assert!(normalized.contains("| Name | Details |\n| --- | --- |"));
        assert!(normalized.contains("| alpha | first line<br>- second &#124; value |"));
        assert!(!normalized.contains("<md-table>"));
        assert!(!normalized.contains(":::html"));
        assert!(normalized.ends_with("After\n"));
    }

    #[test]
    fn converts_headerless_and_mixed_tag_tables_without_dropping_rows() {
        let markdown = concat!(
            "<md-table><md-tbody>",
            "<md-tr><md-th>HTTP URL</md-th><md-td>/open-apis/example</md-td></md-tr>",
            "<md-tr><md-th>HTTP Method</md-th><md-td>GET</md-td></md-tr>",
            "</tbody></table>",
        );

        let normalized = normalize_special_tables(markdown);

        assert!(normalized.starts_with("|  |  |\n| --- | --- |\n"));
        assert!(normalized.contains("| HTTP URL | /open-apis/example |"));
        assert!(normalized.contains("| HTTP Method | GET |"));
    }

    #[test]
    fn converts_tables_terminated_only_by_the_html_directive() {
        let markdown = concat!(
            ":::html\n",
            "<md-table><md-thead><md-tr><md-th>Status</md-th></md-tr></md-thead>\n",
            "<md-tbody><md-tr><md-td>400</md-td></md-tr>\n",
            ":::\n",
            "After\n",
        );

        let normalized = normalize_special_tables(markdown);

        assert!(normalized.starts_with("| Status |\n| --- |\n| 400 |\n"));
        assert!(normalized.ends_with("After\n"));
        assert!(!normalized.contains("<md-table>"));
        assert!(!normalized.contains(":::html"));
    }

    #[test]
    fn converts_unclosed_tables_at_end_of_document() {
        let markdown = concat!(
            "<md-table><md-thead><md-tr>",
            "<md-th><md-td>Name</md-td></md-th>",
            "<md-th>Type</md-th>",
            "</md-tr></md-thead>",
            "<md-tbody><md-tr><md-td>alpha</md-td><md-td>string</md-td></md-tr>\n",
        );

        let normalized = normalize_special_tables(markdown);

        assert_eq!(
            normalized,
            "| Name | Type |\n| --- | --- |\n| alpha | string |\n\n"
        );
    }

    #[test]
    fn keeps_fenced_examples_inside_markdown_table_cells() {
        let markdown = concat!(
            "<md-table><md-thead><md-tr><md-th>Example</md-th></md-tr></md-thead>",
            "<md-tbody><md-tr><md-td>\n",
            "```json\n",
            "{\"pipe\":\"a|b\",\"tag\":\"<x>&\"}\n",
            "```\n",
            "</md-td></md-tr></md-tbody></md-table>",
        );

        let normalized = normalize_special_tables(markdown);

        assert!(
            normalized.contains("<code>{\"pipe\":\"a&#124;b\",\"tag\":\"&lt;x&gt;&amp;\"}</code>")
        );
        assert_eq!(
            normalized
                .lines()
                .filter(|line| line.starts_with('|'))
                .count(),
            3
        );
    }

    #[test]
    fn leaves_special_table_examples_in_fenced_code_unchanged() {
        let markdown = concat!(
            "```html\n",
            "<md-table><md-tr><md-td>example</md-td></md-tr></md-table>\n",
            "```\n",
        );

        assert_eq!(normalize_special_tables(markdown), markdown);
    }
}
