use std::error::Error;
use std::fs::File;
use std::io::Write;

use clap::Parser;

mod cli;
use cli::{Cli, Shell};
use nuacomp_core::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // println!(
    //     "{}",
    //     serde_yaml::to_string(&Command {
    //         description: "Test description".into(),
    //         keyword_arguments: vec![Argument {
    //             name: "--testing".into(),
    //             description: "a test argument".into(),
    //             shorthand: Some("-t".to_string()),
    //             arg_type: ArgumentType::Enumeration(vec![
    //                 "foo".to_string(),
    //                 "bar".to_string(),
    //                 "baz".to_string()
    //             ]),
    //             incompatible_with: vec![]
    //         }],
    //         positional_arguments: vec![],
    //         arguments_valid_anywhere: true
    //     })
    //     .unwrap()
    // );

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
