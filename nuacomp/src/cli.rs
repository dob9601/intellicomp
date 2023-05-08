use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[clap(trailing_var_arg=true)]
pub struct Cli {
    pub shell: Shell,
    pub schema: PathBuf,

    _extra: Vec<String>
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
    Csh,
}
