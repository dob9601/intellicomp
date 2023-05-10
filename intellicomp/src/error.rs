use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntellicompError {
    #[error("Invalid unicode characters were encountered in a path")]
    InvalidUnicodeInPath,

    #[error("IO Error: {0}")]
    FailedToSchemaDir(#[from] std::io::Error),

    #[error("IO Error: {0}")]
    FailedToReadSchema(#[from] serde_yaml::Error)
}
