#[derive(Debug)]
pub enum SchnitzelError {
    CompileError(String),
    RouteError(String),
    IOError(std::io::Error)
}

type Result<T> = std::result::Result<T, SchnitzelError>;

pub mod compiler;
pub mod route;
pub mod rule;
pub mod markdown;

impl From<std::io::Error> for SchnitzelError {
    fn from(value: std::io::Error) -> Self {
        SchnitzelError::IOError(value)
    }
}
