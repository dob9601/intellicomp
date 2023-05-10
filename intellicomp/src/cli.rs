use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub enum Cli {
    Complete(CompleteArgs),
    Hook(HookArgs)
}

#[derive(Debug, Parser)]
#[clap(trailing_var_arg=true)]
pub struct CompleteArgs {
    pub shell: Shell,
    pub schema: PathBuf,

    _extra: Vec<String>
}


#[derive(Debug, Parser)]
pub struct HookArgs {
    pub shell: Shell,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
    Csh,
}
