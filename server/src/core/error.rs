use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use macros::KindError;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, KindError)]
pub enum ErrorKind {
    InvalidApiRequest,
    InvalidCredentials,
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

    UnknownError,
}

#[macro_export]
macro_rules! from_error {
    ($error:ty > $kind:ident) => {
        impl From<$error> for Error {
            fn from(value: $error) -> Self {
                Error {
                    kind: ErrorKind::$kind,
                    message: Some(value.to_string()),
                }
            }
        }
    };
    ($error:ty > $kind:ident: $msg:expr) => {
        impl From<$error> for Error {
            fn from(_value: $error) -> Self {
                Error {
                    kind: ErrorKind::$kind,
                    message: Some($msg.to_string()),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! error_macro {
    ($($kind:ident),+) => {
        $(macro_rules! $kind {
            () => {{
                let e = crate::core::error::Error {
                    kind: crate::core::error::ErrorKind::$kind,
                    message: None,
                };
                tracing::error!("{}", e.to_string());
                e
            }};
            ($msg:expr) => {{
                let e = crate::core::error::Error {
                    kind: crate::core::error::ErrorKind::$kind,
                    message: Some(String::from($msg)),
                };
                tracing::error!("{}", e.to_string());
                e
            }};
        })+
    };
}

pub type Result<T> = std::result::Result<T, Error>;

error_macro! { // creates an Error for each with optional message, logs it, then returns it
    InvalidApiRequest,
    InvalidCredentials,
    UserDoesNotExist,
    PupilDoesNotExist,
    InvalidJwt, // jsonwebtoken::errors::Error
    Unauthorised,
    DatabaseError,
    DecodeError,
    JWTTokenCreationError,

    UnknownError
}

from_error! {sea_orm::DbErr > DatabaseError}
from_error! {std::env::VarError > MissingEnvVariable}
from_error! {std::net::AddrParseError > AddrParseError}
from_error! {hyper::Error > ServerError}
from_error! {jsonwebtoken::errors::Error > InvalidJwt}
from_error! {serde_json::Error > SerializeError}
from_error! {std::string::FromUtf8Error > ParseError}
from_error! {uuid::Error > ParseError}
from_error! {base64::DecodeError > DecodeError: "error decoding"}

impl IntoResponse for Error {
    // TODO integrate this with the KindError macro
    fn into_response(self) -> Response {
        let code = match self.kind {
            ErrorKind::InvalidApiRequest
            | ErrorKind::InvalidCredentials
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
            | ErrorKind::UnknownError
            | ErrorKind::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::Unauthorised | ErrorKind::InvalidJwt => StatusCode::UNAUTHORIZED,
        };
        (
            code,
            Json(ErrorResponse {
                error: self.kind.to_string(),
                details: self.message,
            }),
        )
            .into_response()
    }
}
