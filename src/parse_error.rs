use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParseError {
  NoneError,
  ParseIntError(ParseIntError),
}
impl std::error::Error for ParseError {}
impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ParseError::NoneError => write!(f, "Input Sting is ill formated"),
      ParseError::ParseIntError(err) => write!(
        f,
        "Input Sting is ill formated - expected an integer {}",
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
