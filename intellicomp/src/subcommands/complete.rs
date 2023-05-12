use std::error::Error;
use std::fs::File;
use std::io::Write;

use intellicomp_core::Command;

use crate::cli::{CompleteArgs, Shell};

pub fn run_complete(args: CompleteArgs) -> Result<(), Box<dyn Error>> {
    let schema: Command = serde_yaml::from_reader(File::open(args.schema)?)?;

    match args.shell {
        Shell::Bash => {
            let cursor_position = std::env::var("COMP_POINT")?.parse().unwrap();

            let command: String = std::env::var("COMP_LINE")?;

            let completions = schema
                .generate_completions(&command, cursor_position)
                .unwrap();

            print!("{}", completions.join("\n"));
            std::io::stdout().flush()?;
        }
        _ => unimplemented!(),
    };
    Ok(())
}
