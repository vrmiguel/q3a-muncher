pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
/// Ensure a condition is true or early return
/// with an `AssertionError`
macro_rules! ensure {
    ($cond:expr, $msg:literal $(,)?) => {
        if !$cond {
            return $crate::Result::Err(
                $crate::Error::Assertion($msg),
            );
        }
    };
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("sum would cause overflow")]
    Overflow,
    #[error("subtraction would cause underflow")]
    Underflow,
    #[error("Assertion error: {0}")]
    Assertion(&'static str),
    #[error("Unknown cause of death: {0}")]
    UnknownCauseOfDeath(String),
    #[error("No cause of death is mapped to {0}")]
    CauseOfDeathFromByte(u8),
    #[error("Parsing error: {0}")]
    ParsingError(#[from] nom::error::Error<String>),
    #[error("Missing file\nUsage: ./q3a-muncher [LOG-FILE]")]
    MissingFile,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
