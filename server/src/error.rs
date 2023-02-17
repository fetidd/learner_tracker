use std::{fmt::Display, string::FromUtf8Error};

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde_json::json;

#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) message: Option<String>,
}
// TODO make a derive macro to create assoc. funcs to create each kind of error, with optional message arg, as below
impl Error {
    pub fn user_does_not_exist(msg: Option<&str>) -> Self {
        Error {
            kind: ErrorKind::UserDoesNotExist,
            message: msg.map(|msg| msg.to_owned())
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "[{}]::> {}", self.kind.as_string(), msg),
            None =>  write!(f, "[{}]", self.kind.as_string()),
        }
    }
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
            ErrorKind::InvalidJwt => StatusCode::UNAUTHORIZED,
        };
        tracing::error!("{}", self.to_string()); // ??? is logging the error here correct?
        (
            code,
            Json(json!({"error": self.kind.as_string(), "details": self.message})),
        )
            .into_response()
    }
}

//=====================================================================================================
macro_rules! impl_from_error {
    ($error:ty, $kind:ident) => {
        impl From<$error> for Error {
            fn from(value: $error) -> Self {
                Error {
                    kind: ErrorKind::$kind,
                    message: Some(value.to_string()),
                }
            }
        }
    };
    ($error:ty, $kind:ident, $msg:expr) => {
        impl From<$error> for Error {
            fn from(value: $error) -> Self {
                Error {
                    kind: ErrorKind::$kind,
                    message: Some($msg.to_string()),
                }
            }
        }
    };
}

impl_from_error! {sea_orm::DbErr, DatabaseError}
impl_from_error! {std::env::VarError, MissingEnvVariable}
impl_from_error! {std::net::AddrParseError, AddrParseError}
impl_from_error! {hyper::Error, ServerError}
impl_from_error! {jsonwebtoken::errors::Error, InvalidJwt}
impl_from_error! {serde_json::Error, SerializeError}
impl_from_error! {FromUtf8Error, ParseError}
impl_from_error! {base64::DecodeError, DecodeError}

//=====================================================================================================
/// Different kinds of error that can be thrown in the system.
#[derive(Debug, PartialEq, Clone)]
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
            ErrorKind::DeserializeError => todo!(),
            ErrorKind::DecodeError => todo!(),
            ErrorKind::EncodeError => todo!(),
            ErrorKind::ParseError => todo!(),
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
