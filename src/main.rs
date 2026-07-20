use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use open_larksuite_docs::{FetchOptions, fetch};

#[derive(Debug, Parser)]
#[command(about, version)]
struct Cli {
    /// Directory in which the Markdown tree is stored.
    #[arg(short, long, default_value = "docs")]
    output: PathBuf,

    /// Fetch only one document. Aliases such as /client-docs/intro are accepted.
    #[arg(long)]
    path: Option<String>,

    /// Maximum number of detail requests in flight.
    #[arg(short, long, default_value_t = 8, value_parser = clap::value_parser!(u8).range(1..=64))]
    concurrency: u8,

    /// Completed fetches to drain and report together without stopping request parallelism.
    #[arg(long, default_value_t = 100, value_parser = clap::value_parser!(u16).range(1..=1000))]
    batch_size: u16,

    /// Retries after transient network errors, HTTP 429, and HTTP 5xx responses.
    #[arg(long, default_value_t = 4)]
    retries: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    fetch(FetchOptions {
        output: cli.output,
        path: cli.path,
        concurrency: usize::from(cli.concurrency),
        batch_size: usize::from(cli.batch_size),
        retries: cli.retries,
    })
    .await
}
