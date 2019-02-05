use std::io;

#[derive(Debug, Fail)]
pub enum BenchError {
    #[fail(display = "Internal error: {}", _0)]
    InternalError(&'static str),
    #[fail(display = "ABI error: {}", _0)]
    ABIError(&'static str),
    #[fail(display = "Parse error: {}", _0)]
    ParseError(String),
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "Unsupported")]
    Unsupported,
}

impl From<io::Error> for BenchError {
    fn from(e: io::Error) -> BenchError {
        BenchError::Io(e)
    }
}

impl From<&'static str> for BenchError {
    fn from(e: &'static str) -> BenchError {
        BenchError::InternalError(e)
    }
}
