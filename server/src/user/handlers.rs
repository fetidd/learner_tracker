use crate::{
    core::error::{Error, ErrorKind, Result},
    app::state::AppState,
    user::model::*,
    utils,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

pub async fn create_user(State(state): State<AppState>, Json(req): Json<RequestUser>) -> Result<StatusCode> {
    req.validate()?;
    let user = User::new(&req.first_names, &req.last_name, &req.email_address, &req.hashed_password, req.years);
    match user.save(state.database().as_ref()).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn get_users(State(state): State<AppState>) -> Result<Json<UsersResponse>> {
    match User::all_from_db(state.database().as_ref()).await {
        Ok(users) => Ok(Json(UsersResponse {
            users: users.into_iter().map(ResponseUser::from).collect(),
        })),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!()),
            ErrorKind::UserDoesNotExist => Err(UserDoesNotExist!()),
            _ => Err(UnknownError!()),
        },
    }
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct RequestUser {
    first_names: String,
    last_name: String,
    email_address: String,
    hashed_password: String,
    years: Vec<u32>,
}

impl RequestUser {
    fn validate(&self) -> Result<()> {
        if self.first_names.is_empty() || self.last_name.is_empty() {
            Err(InvalidApiRequest!("names cannot be empty"))
        } else if self.hashed_password.is_empty() {
            Err(InvalidApiRequest!("password cannot be empty"))
        } else if self.years.is_empty() {
            Err(InvalidApiRequest!("must specify at least 1 year group"))
        } else if !utils::is_valid_email(&self.email_address) {
            Err(InvalidApiRequest!("email address is invalid"))
        } else {
            Ok(())
        }
    }
}

#[derive(Serialize, Clone, PartialEq, Debug, Deserialize)]
pub struct ResponseUser {
    first_names: String,
    last_name: String,
    email_address: String,
    years: Vec<u32>,
}

impl From<User> for ResponseUser {
    fn from(value: User) -> Self {
        Self {
            first_names: value.first_names,
            last_name: value.last_name,
            email_address: value.email_address,
            years: value.years,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UsersResponse {
    users: Vec<ResponseUser>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("test", "user", "test@test.com", "password", vec![2,3],  Ok(()))]
    #[case("", "user", "test@test.com", "password", vec![2,3],      Err(InvalidApiRequest!("names cannot be empty")))]
    #[case("test", "", "test@test.com", "password", vec![2,3],      Err(Error {kind: ErrorKind::InvalidApiRequest, message: Some("names cannot be empty".into())}))]
    #[case("test", "user", "test@test.", "password", vec![2,3],     Err(Error {kind: ErrorKind::InvalidApiRequest, message: Some("email address is invalid".into())}))]
    #[case("test", "user", "testattest.com", "password", vec![2,3], Err(Error {kind: ErrorKind::InvalidApiRequest, message: Some("email address is invalid".into())}))]
    #[case("test", "user", "", "password", vec![2,3],               Err(Error {kind: ErrorKind::InvalidApiRequest, message: Some("email address is invalid".into())}))]
    #[case("test", "user", "test@test.com", "", vec![2,3],          Err(Error {kind: ErrorKind::InvalidApiRequest, message: Some("password cannot be empty".into())}))]
    #[case("test", "user", "test@test.com", "password", vec![],     Err(Error {kind: ErrorKind::InvalidApiRequest, message: Some("must specify at least 1 year group".into())}))]
    fn test_validate_request_user(
        #[case] first_names: String,
        #[case] last_name: String,
        #[case] email_address: String,
        #[case] hashed_password: String,
        #[case] years: Vec<u32>,
        #[case] exp: Result<()>,
    ) {
        let req = RequestUser {
            first_names,
            last_name,
            email_address,
            hashed_password,
            years,
        };
        match exp {
            Ok(_) => assert!(req.validate().is_ok()),
            Err(error) => {
                assert_eq!(error.kind, req.validate().unwrap_err().kind);
                assert_eq!(error.message, req.validate().unwrap_err().message);
            }
        }
    }
}
