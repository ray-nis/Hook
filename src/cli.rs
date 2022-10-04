use clap::Parser;

#[derive(Parser)]
#[command(name = "Hook")]
#[command(version = "0.1.0")]
#[command(about = "Simple grep clone", long_about = None)]
pub struct Cli {
    pub pattern: String,
    pub source: String,
    /// Amount of lines that match the pattern.
    #[arg(short, long)]
    pub count: bool,
    /// Ignore case on pattern matching.
    #[arg(short, long)]
    pub ignore_case: bool,
    /// Search for whole words.
    #[arg(short, long)]
    pub whole_word: bool,
    /// Show number of line.
    #[arg(short, long)]
    pub number_line: bool,
    /// Highlight patterns found.
    #[arg(short, long)]
    pub pattern_highlighter: bool,
}