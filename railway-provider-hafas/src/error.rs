use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum ParseError {
    #[error("{info}")]
    InvalidData { info: String },
    #[error("source")]
    Chrono {
        #[from]
        source: chrono::ParseError,
    },
    #[error("source")]
    Int {
        #[from]
        source: std::num::ParseIntError,
    },
}

impl From<String> for ParseError {
    fn from(info: String) -> ParseError {
        ParseError::InvalidData { info }
    }
}

impl From<&str> for ParseError {
    fn from(info: &str) -> ParseError {
        ParseError::InvalidData {
            info: info.to_string(),
        }
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{source}")]
    Json {
        #[from]
        source: serde_json::Error,
    },
    #[error("{source}")]
    Parse {
        #[from]
        source: ParseError,
    },
    #[error("{text}")]
    Hafas { code: String, text: String },
    #[error("{0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, Error>;
pub type ParseResult<T> = std::result::Result<T, ParseError>;
