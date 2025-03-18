use std::fmt;

#[derive(Debug)]
pub enum DbError {
    ConnectionFailed(String),
    QueryFailed(String),
    UnexpectedError(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DbError::ConnectionFailed(ref msg) => write!(f, "Connection Failed: {}", msg),
            DbError::QueryFailed(ref msg) => write!(f, "Query Failed: {}", msg),
            DbError::UnexpectedError(ref msg) => write!(f, "Unexpected Error: {}", msg),
        }
    }
}

impl std::error::Error for DbError {}
