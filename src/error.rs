use thiserror::Error;

#[derive(Error, Debug)]
pub enum SidlError {
    #[error("Unexpected character '{0}'")]
    UnexpectedChar(char),

    #[error("Unexpected token: expected {expected}, found {found}")]
    UnexpectedToken {
        expected: String,
        found: String,
    },

    #[error("Unexpected token at top level: {0}")]
    UnexpectedTopLevelToken(String),

    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}
