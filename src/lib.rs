use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SchnitzelError {
    CompileError(String),
    RouteError(String),
    IOError(std::io::Error),
}

type Result<T> = std::result::Result<T, SchnitzelError>;

pub mod compiler;
pub mod markdown;
pub mod route;
pub mod rule;

pub mod context;

impl From<std::io::Error> for SchnitzelError {
    fn from(value: std::io::Error) -> Self {
        SchnitzelError::IOError(value)
    }
}

impl Display for SchnitzelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchnitzelError::CompileError(e) => write!(f, "Compile error: {e}"),
            SchnitzelError::RouteError(e) => write!(f, "Invalid route: {e}"),
            SchnitzelError::IOError(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl Error for SchnitzelError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SchnitzelError::IOError(e) => Some(e),
            _ => None
        }
    }
}
