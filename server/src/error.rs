use thiserror::Error;

pub type PTResult<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid password")]
    InvalidPassword,
    #[error("user does not exist")]
    UserDoesNotExist,
    #[error("pupil does not exist")]
    PupilDoesNotExist,
    // std errors
    #[error("environment error: {0}")]
    VarError(#[from] std::env::VarError),
    #[error("ip address parse error: {0}")]
    AddrParseError(#[from] std::net::AddrParseError),
    // External crate errors
    #[error("database error: {0}")]
    DbErr(#[from] sea_orm::DbErr),
    #[error("hyper error: {0}")]
    HyperError(#[from] hyper::Error),
    #[error("")]
    AuthError(#[from] eyre::ErrReport),
    #[error("")]
    IoError(#[from] std::io::Error),
    #[error("session database error: {0}")]
    SessionDatabaseError(#[from] redis::RedisError),
}
