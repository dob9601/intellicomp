use std::error::Error;

use clap::Parser;
use intellicomp::Cli;
use intellicomp::subcommands::{run_complete, run_hook};

fn main() -> Result<(), Box<dyn Error>> {
    let subcommand = Cli::parse();

    match subcommand {
        Cli::Complete(args) => run_complete(args)?,
        Cli::Hook(args) => run_hook(args)?,
    };
    Ok(())
}
