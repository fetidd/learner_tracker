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
    pub(crate) message: String
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}| {}", self.kind.as_string(), self.message)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let code = match self.kind {
            ErrorKind::InvalidApiRequest =>     StatusCode::BAD_REQUEST,
            ErrorKind::InvalidUserPassword =>   StatusCode::BAD_REQUEST,
            ErrorKind::UserDoesNotExist =>      StatusCode::BAD_REQUEST,
            ErrorKind::PupilDoesNotExist =>     StatusCode::BAD_REQUEST,
            ErrorKind::MissingEnvVariable =>    StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::AddrParseError =>        StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::IoError =>               StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::ParseIntError =>         StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::DatabaseError =>         StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::ServerError =>           StatusCode::INTERNAL_SERVER_ERROR,
        };
        (code, Json(json!({"error": self.kind.as_string(), "details": self.message}))).into_response()
    }
}

//=====================================================================================================
impl From<sea_orm::DbErr> for Error {
    fn from(value: sea_orm::DbErr) -> Self {
        Error {
            kind: ErrorKind::DatabaseError,
            message: value.to_string()
        }
    }
}

impl From<std::env::VarError> for Error {
    fn from(value: std::env::VarError) -> Self {
        Error {
            kind: ErrorKind::MissingEnvVariable,
            message: value.to_string()
        }
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(value: std::net::AddrParseError) -> Self {
        Error {
            kind: ErrorKind::AddrParseError,
            message: value.to_string()
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(value: hyper::Error) -> Self {
        Error {
            kind: ErrorKind::ServerError,
            message: value.to_string()
        }
    }
}

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
            ErrorKind::InvalidApiRequest =>     String::from("InvalidApiRequest"),
            ErrorKind::InvalidUserPassword =>   String::from("InvalidUserPassword"),
            ErrorKind::UserDoesNotExist =>      String::from("UserDoesNotExist"),
            ErrorKind::PupilDoesNotExist =>     String::from("PupilDoesNotExist"),
            ErrorKind::MissingEnvVariable =>    String::from("MissingEnvVariable"),
            ErrorKind::AddrParseError =>        String::from("AddrParseError"),
            ErrorKind::IoError =>               String::from("IoError"),
            ErrorKind::ParseIntError =>         String::from("ParseIntError"),
            ErrorKind::DatabaseError =>         String::from("DatabaseError"),
            ErrorKind::ServerError =>           String::from("ServerError"),
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}