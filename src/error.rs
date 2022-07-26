pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
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
    #[error("Error: sum would cause overflow")]
    Overflow,
    #[error("Error: subtraction would cause underflow")]
    Underflow,
    #[error("Assertion error: {0}")]
    Assertion(&'static str),
    #[error("Unknown cause of death: {0}")]
    UnknownCauseOfDeath(String),
    #[error("No cause of death is mapped to {0}")]
    CauseOfDeathFromByte(u8),
    #[error("Parsing error")]
    // ParsingError(nom::error::Error<String>),
    ParsingError,
    #[error("Missing file\nUsage: ./q3a-muncher [LOG-FILE]")]
    MissingFile,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
