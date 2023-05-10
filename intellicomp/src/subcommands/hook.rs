use crate::hooks::generate_bash_completions;
use crate::{IntellicompError, generate_fish_completions};

use directories::ProjectDirs;

use crate::cli::HookArgs;

pub fn run_hook(args: HookArgs) -> Result<(), IntellicompError> {
    let dirs = ProjectDirs::from("com", "dob9601", "nuacomp").unwrap();


    let schema_dir = dirs.data_dir();
    std::fs::create_dir_all(schema_dir)?;

    let binary_location = std::env::current_exe()?;

    match args.shell {
        crate::cli::Shell::Bash => generate_bash_completions(schema_dir, &binary_location),
        crate::cli::Shell::Fish => generate_fish_completions(schema_dir),
        crate::cli::Shell::Zsh => todo!(),
        crate::cli::Shell::Csh => todo!(),
    }?;

    println!("echo 'Intellicomp configured!'");

    Ok(())
}
