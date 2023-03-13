use crate::{
    app::state::AppState,
    core::error::{ErrorKind, Result, Error},
    utils,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use chrono::Utc;
use sea_orm::{IntoActiveModel, ActiveModelTrait, EntityTrait};
use serde::{Deserialize, Serialize};
use entity::*;

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<RequestUser>,
) -> Result<StatusCode> {
    req.validate()?;
    let user = user::Model {
        first_names: req.first_names,
        last_name: req.last_name,
        email_address: req.email_address,
        hashed_password: req.hashed_password,
        years: req.years,
        last_refresh: Utc::now().naive_local(),
        secret: crate::utils::functions::generate_secret().to_vec(),
    };
    match user.into_active_model().save(state.database().as_ref()).await.map_err(|e| Error::from(e)) {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn get_users(State(state): State<AppState>) -> Result<Json<UsersResponse>> {
    match user::Entity::find().all(state.database().as_ref()).await.map_err(|e| Error::from(e)) {
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
    years: Vec<i32>,
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
    years: Vec<i32>,
}

impl From<user::Model> for ResponseUser {
    fn from(value: user::Model) -> Self {
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
    use crate::core::error::Error;
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
        #[case] years: Vec<i32>,
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
