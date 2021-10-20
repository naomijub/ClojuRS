use num_bigint::ParseBigIntError;

// Issue 12
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Reason(String),
    Thrown(String),
    ArityException(u16, String),
    UnknownSymbol(String),
    CantEval(Option<String>),
    IntParseError,
}

impl From<std::num::ParseIntError> for Error {
    fn from(s: std::num::ParseIntError) -> Self {
        Error::Reason(s.to_string())
    }
}

impl From<ParseBigIntError> for Error {
    fn from(s: ParseBigIntError) -> Self {
        Error::Reason(s.to_string())
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(s: std::num::ParseFloatError) -> Self {
        Error::Reason(s.to_string())
    }
}

impl From<std::str::ParseBoolError> for Error {
    fn from(s: std::str::ParseBoolError) -> Self {
        Error::Reason(s.to_string())
    }
}
