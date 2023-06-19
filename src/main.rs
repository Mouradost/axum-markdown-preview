use args::Cli;
use clap::Parser;
use axum_markdown_preview::run_server;
mod args;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let args = Cli::parse();
    run_server(args.file).await?;
    Ok(())
}
