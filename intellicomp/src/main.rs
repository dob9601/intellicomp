use std::error::Error;

use clap::Parser;
use intellicomp::subcommands::{run_autogenerate, run_complete, run_hook};
use intellicomp::Cli;

fn main() -> Result<(), Box<dyn Error>> {
    //let foo = FishCompleteParser::parse();
    let subcommand = Cli::parse();

    match subcommand {
        Cli::Complete(args) => run_complete(args)?,
        Cli::Hook(args) => run_hook(args)?,
        Cli::Autogenerate(args) => run_autogenerate(args)?,
    };
    Ok(())
}
