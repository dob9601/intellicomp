use std::fs;
use std::path::Path;

use crate::IntellicompError;

pub fn generate_bash_completions(
    schema_directory: &Path,
    binary_location: &Path,
) -> Result<(), IntellicompError> {
    for schema_file in fs::read_dir(schema_directory)?.filter_map(|f| f.ok()) {
        let file_name = schema_file.file_name().to_string_lossy().to_string();

        let file_path = schema_file.path().to_string_lossy().to_string();

        let command_name = file_name.strip_suffix(".yaml");

        if let Some(command_name) = command_name {
            println!(
                "complete -C \"{} complete bash {file_path}\" {command_name}",
                binary_location
                    .to_str()
                    .ok_or(IntellicompError::InvalidUnicodeInPath)?
            );
        }
    }
    Ok(())
}
