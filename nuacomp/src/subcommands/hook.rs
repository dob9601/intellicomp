use std::error::Error;
use std::fs;

use directories::ProjectDirs;

use crate::cli::HookArgs;

pub fn run_hook(args: HookArgs) -> Result<(), Box<dyn Error>> {
    let dirs = ProjectDirs::from("com", "dob9601", "nuacomp").unwrap();

    println!("echo 'Intellicomp configured!'");

    let schema_dir = dirs.data_dir();
    std::fs::create_dir_all(schema_dir)?;

    let bin_location = std::env::current_exe()?.to_string_lossy().to_string();

    match args.shell {
        crate::cli::Shell::Bash => {
            for schema_file in fs::read_dir(schema_dir)?.filter_map(|f| f.ok()) {
                let file_name = schema_file.file_name().to_string_lossy().to_string();

                let file_path = schema_file.path().to_string_lossy().to_string();

                let command_name = file_name.strip_suffix(".yaml");

                if let Some(command_name) = command_name {
                    println!(
                        "complete -C \"{bin_location} complete bash {file_path}\" {command_name}"
                    );
                }
            }
            Ok(())
        }
        crate::cli::Shell::Fish => todo!(),
        crate::cli::Shell::Zsh => todo!(),
        crate::cli::Shell::Csh => todo!(),
    }
}
