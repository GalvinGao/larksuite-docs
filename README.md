# Open LarkSuite docs

This Rust CLI mirrors the public Lark developer documentation as a directory tree of Markdown
files. It discovers document leaves from Lark's directory API, fetches each document's detail
record, and writes only durable metadata into YAML frontmatter.

Each document is stored under a human-readable breadcrumb derived from the directory tree. For
example, `/home/intro` becomes
`docs/developer-guides/platform-introduction/lark-open-capabilities-introduction.md`, while the
opaque API path remains in frontmatter. Duplicate sibling labels are disambiguated with meaningful
API resource names. The detail API canonicalizes aliases, so `/client-docs/intro` resolves to the
same breadcrumb path.

## Fetch

Fetch every document with bounded request concurrency, batched result draining, and retries:

```sh
cargo run --release
```

Fetch one document or choose another destination:

```sh
cargo run --release -- --path /client-docs/intro
cargo run --release -- --output ./markdown --concurrency 16 --batch-size 200
```

The detail endpoint accepts one `fullPath` per request, so `--concurrency` controls the continuously
full pool of in-flight requests. `--batch-size` independently controls how many completed outcomes
are processed together for progress and error reporting; the CLI never retains all corpus results
in memory.

Generated frontmatter contains `document_id`, `directory_id`, `title`, the original `full_path`, a
human-readable `breadcrumb`, `document_type`, a UTC RFC 3339 `updated_at`, and `source_url`.
Volatile rendering and access fields—including `schema`, `bizScope`, `allVisible`, `visibleTag`,
and empty template fields—are intentionally omitted.

Legacy internal link destinations such as `/ssl:ttdoc/home/intro` are rewritten to
`/document/home/intro`. Markdown remains Markdown, inline and fenced code examples remain
unchanged, and Lark HTML-component `href` attributes receive the same path normalization.
Lark `<md-table>` components are converted to native Markdown tables; multiline cell content is
kept on explicit line breaks, and fenced examples inside cells remain formatted as code.

## Quality checks

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Automated updates

The `Publish fetcher release` GitHub Actions workflow tests and compiles the project whenever `main`
changes. Each successful build publishes the Linux x86_64 binary and its SHA-256 checksum in a
commit-specific GitHub Release and marks that release as latest.

The `Update documentation mirror` workflow refreshes `docs/` every day at 03:23 UTC and can also be
started manually from the Actions tab. It downloads and verifies the binary from the latest release,
rebuilds the complete documentation tree without compiling Rust, and stages only `docs/`. When the
generated tree is unchanged, the workflow exits without creating a commit. Otherwise, it commits as
`github-actions[bot]` and pushes directly to the repository's default branch. The resulting
`GITHUB_TOKEN` push does not recursively start the release workflow.

Both workflows use the repository `GITHUB_TOKEN` with only `contents: write` permission. Repository
settings and default-branch rules must allow GitHub Actions to create releases and write to the
branch.
