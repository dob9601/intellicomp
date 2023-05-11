use std::{
    fs,
    path::{Path, PathBuf},
};

mod fish;
pub use fish::Fish;

mod bash;
pub use bash::Bash;

use directories::ProjectDirs;
use git2::build::RepoBuilder;

use crate::IntellicompError;

pub trait CompletableShell {
    fn generate_completion_commands() -> Result<Vec<String>, IntellicompError> {
        let mut completion_commands = vec![];

        let schema_directory = get_or_update_schema_dir()?;

        for schema_file in fs::read_dir(schema_directory)?
            .filter_map(|f| f.ok())
            .filter(|f| {
                f.path()
                    .extension()
                    .map(|e| e.to_string_lossy())
                    .unwrap_or("".into())
                    == "yaml"
            })
        {
            completion_commands
                .extend(Self::generate_completions_from_schema(&schema_file.path())?.into_iter());
        }
        Ok(completion_commands)
    }

    fn generate_completions_from_schema(
        schema_file: &Path,
    ) -> Result<Vec<String>, IntellicompError>;
}

pub fn get_or_update_schema_dir() -> Result<PathBuf, IntellicompError> {
    let dirs = ProjectDirs::from("com", "dob9601", "intellicomp").unwrap();
    let schema_dir = dirs.data_dir();
    if !schema_dir.exists() {
        println!("echo 'Pulling schemas from dob9601/intellicomp-schemas'");

        std::fs::create_dir_all(schema_dir)?;
        RepoBuilder::new().clone(
            "https://github.com/dob9601/intellicomp-schemas.git",
            schema_dir,
        )?;
    }
    Ok(schema_dir.to_path_buf())
}
