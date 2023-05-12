use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandParseError {
    #[error("The cursor is at position {0} which is out of range for the input.")]
    CursorOutOfRange(usize),

    #[error("The command input is invalid")]
    InvalidCommandInput,

    #[error("The argument {0} is missing a value")]
    ArgumentMissingValue(String),

    #[error("Unexpected Token: {0}")]
    UnexpectedToken(String),

    #[error("IOError: {0}")]
    IOError(#[from] glob::GlobError)
}
