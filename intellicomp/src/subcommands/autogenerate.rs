use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use intellicomp_core::{Command, KeywordArgument, ValueType};

use crate::{cli::AutogenerateArgs, IntellicompError};

const APOSTROPHE_MARKER: &str = "<ApO-MaRkEr>";
const ESCAPED_APOSTROPHE: &str = "\\\'";

pub fn run_autogenerate(args: AutogenerateArgs) -> Result<(), IntellicompError> {
    let extension = args
        .file
        .extension()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .ok_or(IntellicompError::InvalidUnicodeInPath)?;

    match extension {
        "fish" => {
            let file = File::open(args.file)?;
            let buffer = BufReader::new(file);
            std::fs::create_dir_all(&args.output_directory)?;

            let mut keyword_arguments: HashMap<String, KeywordArgument> = HashMap::new();
            let mut binary_name = None;

            for line in buffer.lines() {
                let line = line?;
                if !line.starts_with("complete") {
                    continue;
                }
                let mut args = FishCompleteParser::parse_from(
                    // This is a horrible hack. The escaped apostrophes work
                    // in fish-style splitting but not in bash style splitting
                    shlex::split(&line.replace(ESCAPED_APOSTROPHE, APOSTROPHE_MARKER))
                        .unwrap()
                        .into_iter()
                        .map(|fragment| fragment.replace(APOSTROPHE_MARKER, ESCAPED_APOSTROPHE)),
                );

                let name = if let Some(s) = args.long_option {
                    s
                } else if let Some(s) = args.old_option {
                    s
                } else if let Some(s) = args.short_option {
                    args.short_option = None;
                    s
                } else {
                    panic!("Missing a name: {line}")
                };

                binary_name = Some(args.command);

                keyword_arguments.insert(
                    name,
                    KeywordArgument {
                        description: args.description,
                        shorthand: args.short_option,
                        repeatable: false,
                        value_type: ValueType::String, // TODO: Can parse this better
                        incompatible_with: vec![],
                    },
                );
            }

            let command = Command {
                description: "".into(),
                keyword_arguments,
                positional_arguments: vec![],
            };
            let output_file = File::create(
                args.output_directory
                    .join(format!("{}.yaml", binary_name.unwrap())),
            )?;
            serde_yaml::to_writer(output_file, &command)?;
        }
        _ => todo!(),
    };

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(trailing_var_arg = true)]
pub struct FishCompleteParser {
    #[arg(long, short)]
    command: String,

    #[arg(long, short)]
    short_option: Option<String>,

    #[arg(long, short)]
    long_option: Option<String>,

    #[arg(long, short)]
    old_option: Option<String>,

    #[arg(long, short)]
    description: String,

    _extra: Vec<String>,
}
