use std::fmt::Display;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde_json::json;

#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) message: String,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.kind.as_string(), self.message)
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
            | ErrorKind::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
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
                    message: value.to_string(),
                }
            }
        }
    };
}

impl_from_error! {sea_orm::DbErr,           DatabaseError}
impl_from_error! {std::env::VarError,       MissingEnvVariable}
impl_from_error! {std::net::AddrParseError, AddrParseError}
impl_from_error! {hyper::Error,             ServerError}

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
}

impl ErrorKind {
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
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
