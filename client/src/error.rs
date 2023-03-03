use gloo_storage::errors::StorageError;

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
    DecodeError,
    StorageError,
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

from_error!(gloo_net::Error > ServerError);
from_error!(std::num::ParseIntError > ValueError: "failed to parse to int");
from_error!(chrono::ParseError > ValueError: "failed to parse date or time");
from_error!(StorageError > StorageError);
from_error!(base64::DecodeError > DecodeError);
from_error!(std::string::FromUtf8Error > DecodeError);
from_error!(serde_json::Error > JsonError);

error_macro!{
    ResponseParseError,
    DecodeError,
    StorageError,
    ServerError,
    Unauthorized
}
