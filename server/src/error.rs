use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use macros::KindError;
use serde::{Deserialize, Serialize};
use shared_utils::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, KindError)]
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
