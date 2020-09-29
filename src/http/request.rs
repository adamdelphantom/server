use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request {
  path: String,
  query_string: Option<String>,
  method: Method,
}

impl TryFrom<&[u8]> for Request {
  type Error = ParseError;

  // GET /search?name=abs&sort=1 HTTP/1.1
  fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
    let request = str::from_utf8(buf)?;
    unimplemented!();
  }
}

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod,
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      Self::InvalidRequest => "Invalid Request",
      Self::InvalidEncoding => "Invalid Encoding",
      Self::InvalidProtocol => "Invalid Protocol",
      Self::InvalidMethod => "Invalid Method",
    }
  }
}
impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    Self::InvalidEncoding
  }
}
impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message())
  }
}
impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Error for ParseError {}
