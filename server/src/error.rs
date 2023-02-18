use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use macros::*;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) message: Option<String>,
}

error_macro! { // creates an Error for each with optional message, logs it, then returns it
    InvalidApiRequest,
    InvalidUserPassword,
    UserDoesNotExist,
    PupilDoesNotExist,
    InvalidJwt, // jsonwebtoken::errors::Error
    Unauthorised
}

pub type Result<T> = std::result::Result<T, Error>;

from_error! {sea_orm::DbErr > DatabaseError}
from_error! {std::env::VarError > MissingEnvVariable}
from_error! {std::net::AddrParseError > AddrParseError}
from_error! {hyper::Error > ServerError}
from_error! {jsonwebtoken::errors::Error > InvalidJwt}
from_error! {serde_json::Error > SerializeError}
from_error! {std::string::FromUtf8Error > ParseError}
from_error! {base64::DecodeError > DecodeError: "error decoding"}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "[{}]::> {}", self.kind.as_string(), msg),
            None => write!(f, "[{}]", self.kind.as_string()),
        }
    }
}

#[derive(Serialize, PartialEq, Clone, Debug)]
struct ErrorResponse {
    error: ErrorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let code = match self.kind {
            ErrorKind::InvalidApiRequest
            | ErrorKind::InvalidUserPassword
            | ErrorKind::UserDoesNotExist
            | ErrorKind::PupilDoesNotExist => StatusCode::BAD_REQUEST,
            ErrorKind::MissingEnvVariable
            | ErrorKind::AddrParseError
            | ErrorKind::IoError
            | ErrorKind::ParseIntError
            | ErrorKind::DatabaseError
            | ErrorKind::JWTTokenCreationError
            | ErrorKind::SerializeError
            | ErrorKind::DeserializeError
            | ErrorKind::EncodeError
            | ErrorKind::DecodeError
            | ErrorKind::ParseError
            | ErrorKind::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::Unauthorised | ErrorKind::InvalidJwt => StatusCode::UNAUTHORIZED,
        };
        (
            code,
            Json(ErrorResponse {
                error: self.kind,
                details: self.message,
            }),
        )
            .into_response()
    }
}

//=====================================================================================================
/// Different kinds of error that can be thrown in the system.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum ErrorKind {
    InvalidApiRequest,
    InvalidUserPassword,
    UserDoesNotExist,
    PupilDoesNotExist,
    MissingEnvVariable, // std::var::VarError
    AddrParseError,     // std::net::AddrParseError
    IoError,            // std::io::Error
    ParseIntError,      // std::num::ParseIntError
    DatabaseError,      // sea_orm
    ServerError,        // hyper
    JWTTokenCreationError,
    InvalidJwt, // jsonwebtoken::errors::Error
    SerializeError,
    DeserializeError,
    DecodeError,
    EncodeError,
    ParseError,
    Unauthorised,
}

impl ErrorKind {
    // TODO turn this into a proc_macro_derive to use on the enum: https://developerlife.com/2022/03/30/rust-proc-macro/
    fn as_string(&self) -> String {
        match self {
            ErrorKind::InvalidApiRequest => String::from("InvalidApiRequest"),
            ErrorKind::InvalidUserPassword => String::from("InvalidUserPassword"),
            ErrorKind::UserDoesNotExist => String::from("UserDoesNotExist"),
            ErrorKind::PupilDoesNotExist => String::from("PupilDoesNotExist"),
            ErrorKind::MissingEnvVariable => String::from("MissingEnvVariable"),
            ErrorKind::AddrParseError => String::from("AddrParseError"),
            ErrorKind::IoError => String::from("IoError"),
            ErrorKind::ParseIntError => String::from("ParseIntError"),
            ErrorKind::DatabaseError => String::from("DatabaseError"),
            ErrorKind::ServerError => String::from("ServerError"),
            ErrorKind::JWTTokenCreationError => String::from("JWTTokenCreationError"),
            ErrorKind::InvalidJwt => String::from("InvalidJwt"),
            ErrorKind::SerializeError => String::from("SerializeError"),
            ErrorKind::DeserializeError => String::from("DeserializeError"),
            ErrorKind::DecodeError => String::from("DecodeError"),
            ErrorKind::EncodeError => String::from("EncodeError"),
            ErrorKind::ParseError => String::from("ParseError"),
            ErrorKind::Unauthorised => String::from("Unauthorised"),
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
