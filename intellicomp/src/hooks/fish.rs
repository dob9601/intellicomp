use std::fs;
use std::path::Path;

use intellicomp_core::Command;

use crate::{CompletableShell, IntellicompError};

pub struct Fish;

impl CompletableShell for Fish {
    fn generate_completions_from_schema(
        schema_file: &Path,
    ) -> Result<Vec<String>, IntellicompError> {
        let schema: Command = serde_yaml::from_reader(fs::File::open(schema_file)?)?;
        let file_name = schema_file.file_name().unwrap().to_string_lossy();

        let command_name = file_name.strip_suffix(".yaml");

        let mut completion_commands = vec![];

        if let Some(command_name) = command_name {
            let base_command = format!("complete -c {command_name}");

            for (name, argument) in schema.keyword_arguments {
                let description = argument.description;

                let mut command = format!("{base_command} -l '{name}' -d \'{description}\'");

                if let Some(shorthand) = argument.shorthand {
                    let shorthand = shorthand.trim_start_matches('-');
                    command = format!("{command} -s '{shorthand}'")
                }

                completion_commands.push(command);
            }
        }

        Ok(completion_commands)
    }
}
