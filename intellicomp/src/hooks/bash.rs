use std::path::Path;

use crate::{CompletableShell, IntellicompError};

pub struct Bash;

impl CompletableShell for Bash {
    fn generate_completions_from_schema(
        schema_file: &Path,
    ) -> Result<Vec<String>, IntellicompError> {
        let command_name = schema_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches(".yaml");

        Ok(vec![format!(
            "complete -C \"{} complete bash {}\" {command_name}",
            std::env::current_exe()?
                .to_str()
                .ok_or(IntellicompError::InvalidUnicodeInPath)?,
            schema_file.to_string_lossy()
        )])
    }
}
