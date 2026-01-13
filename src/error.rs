use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

#[derive(Error, Debug)]
pub enum SidlError {
    #[error("Unexpected character '{char}' at {loc}")]
    UnexpectedChar { char: char, loc: Location },

    #[error("Unexpected token at {loc}: expected {expected}, found {found}")]
    UnexpectedToken {
        expected: String,
        found: String,
        loc: Location,
    },

    #[error("Unexpected token at top level at {1}: {0}")]
    UnexpectedTopLevelToken(String, Location),

    #[error("Invalid syntax at {loc}: {msg}")]
    #[allow(dead_code)]
    InvalidSyntax { msg: String, loc: Location },

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}
