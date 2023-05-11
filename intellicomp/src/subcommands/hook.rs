use crate::{Bash, CompletableShell, Fish, IntellicompError};

use directories::ProjectDirs;
use git2::build::RepoBuilder;

use crate::cli::HookArgs;

pub fn run_hook(args: HookArgs) -> Result<(), IntellicompError> {
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

    let completions = match args.shell {
        crate::cli::Shell::Bash => Bash::generate_completion_commands(),
        crate::cli::Shell::Fish => Fish::generate_completion_commands(),
        crate::cli::Shell::Zsh => todo!(),
        crate::cli::Shell::Csh => todo!(),
    }?;

    println!("{}", completions.join("\n"));

    println!("echo 'Intellicomp configured!'");

    Ok(())
}
