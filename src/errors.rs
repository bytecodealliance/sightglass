use std::io;

#[derive(Debug, Error)]
pub enum BenchError {
    #[error("Internal error: {0}")]
    InternalError(&'static str),
    #[error("ABI error: {0}")]
    ABIError(&'static str),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("Unsupported")]
    Unsupported,
}

impl From<&'static str> for BenchError {
    fn from(e: &'static str) -> BenchError {
        BenchError::InternalError(e)
    }
}
