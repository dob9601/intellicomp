use std::fs;
use std::path::Path;

use nuacomp_core::Command;

use crate::IntellicompError;

pub fn generate_fish_completions(
    schema_directory: &Path,
) -> Result<(), IntellicompError> {
    for schema_file in fs::read_dir(schema_directory)?.filter_map(|f| f.ok()) {
        let schema: Command = serde_yaml::from_reader(fs::File::open(schema_file.path())?)?;
        let file_name = schema_file.file_name().to_string_lossy().to_string();

        let command_name = file_name.strip_suffix(".yaml");

        if let Some(command_name) = command_name {
            let base_command = format!("complete -c {command_name}");

            for argument in schema.keyword_arguments {
                let argument_name =  argument.name.trim_start_matches('-');
                let description = argument.description;

                let mut command = format!("{base_command} -l {argument_name} -d \'{description}\'");

                if let Some(shorthand) = argument.shorthand {
                    let shorthand = shorthand.trim_start_matches('-');
                    command = format!("{command} -s {shorthand}")
                }

                println!("{command}");
            }
        }
    }
    Ok(())
}
