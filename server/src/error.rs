use thiserror::Error;

pub type LTResult<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("invalid password")]
    InvalidPassword,
    #[error("user does not exist")]
    UserDoesNotExist,
    #[error("pupil does not exist")]
    PupilDoesNotExist,
    #[error("users must have at least 1 year assigned to them")]
    NoYearsForUser,

    // std errors
    #[error("environment error: {0}")]
    VarError(#[from] std::env::VarError),
    #[error("ip address parse error: {0}")]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("parsing error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    // External crate errors
    #[error("database error: {0}")]
    DbErr(#[from] sea_orm::DbErr),
    #[error("hyper error: {0}")]
    HyperError(#[from] hyper::Error),
    #[error("authorisation error: {0}")] // FIXME is this still needed?
    AuthError(#[from] eyre::ErrReport),
    #[error("session database error: {0}")]  // FIXME is this still needed?
    SessionDatabaseError(#[from] redis::RedisError),
}
