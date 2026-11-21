pub enum SchnitzelError {
    CompileError(String),
    RouteError(String)
}

type Result<T> = std::result::Result<T, SchnitzelError>;

pub mod compiler;
pub mod route;
pub mod rule;
