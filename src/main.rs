mod cli;
mod hook;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();

    hook::hook(cli);
}
