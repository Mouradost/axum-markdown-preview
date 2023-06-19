use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Markdown file.
    pub file: std::path::PathBuf,
}
