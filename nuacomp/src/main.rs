use std::error::Error;

use clap::Parser;

mod cli;
use cli::Cli;

mod subcommands;
use subcommands::*;

fn main() -> Result<(), Box<dyn Error>> {
    let subcommand = Cli::parse();

    match subcommand {
        Cli::Complete(args) => run_complete(args),
        Cli::Hook(args) => run_hook(args),
    }
}
