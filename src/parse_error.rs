use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParseError {
    ValidationError,
    NoneError,
    ParseIntError(ParseIntError),
}
impl std::error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::ValidationError => write!(f, "Input Sting is not valid"),
            ParseError::NoneError => write!(f, "Input Sting is ill formatted"),
            ParseError::ParseIntError(err) => write!(
                f,
                "Input Sting is ill formatted - expected an integer {}",
                err
            ),
        }
    }
}
impl std::convert::From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseError::ParseIntError(err)
    }
}
