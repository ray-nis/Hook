use clap::Parser;

#[derive(Parser)]
#[command(name = "Hook")]
#[command(version = "0.1.0")]
#[command(about = "Simple grep clone", long_about = None)]
pub struct Cli {
    pub pattern: String,
    pub source: String,
}