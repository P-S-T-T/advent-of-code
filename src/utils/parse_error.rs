use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParseError {
    Validation,
    None,
    ParseInt(ParseIntError),
}
impl std::error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Validation => write!(f, "Input Sting is not valid"),
            ParseError::None => write!(f, "Input Sting is ill formatted"),
            ParseError::ParseInt(err) => write!(
                f,
                "Input Sting is ill formatted - expected an integer {}",
                err
            ),
        }
    }
}
impl std::convert::From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseError::ParseInt(err)
    }
}
