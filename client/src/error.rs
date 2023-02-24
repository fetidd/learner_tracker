#[derive(Clone, PartialEq, Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub details: Option<String>
}

#[derive(Clone, PartialEq, Debug)]
pub enum ErrorKind {
    Unauthorized,
    ValueError,
    ResponseParseError,
    ServerError,
    CastError,
    JsonError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.details {
            Some(msg) => write!(f, "{:?}: {}", self.kind, msg),
            None => write!(f, "{:?}", self.kind),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

from_error!(() > ServerError);
from_error!(gloo_net::Error > ServerError);
from_error!(std::num::ParseIntError > ValueError: "failed to parse to int");
from_error!(chrono::ParseError > ValueError: "failed to parse date or time");
error_macro!{
    Unauthorized,
    ResponseParseError,
    ServerError,
    ValueError,
    CastError,
    JsonError
}
