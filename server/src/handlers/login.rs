use crate::{
    app_state::AppState,
    error::{Error, PTResult},
    models::User,
};
use axum::{extract::State, response::IntoResponse, Json};
use entity::user::Model;
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

pub async fn login_handler(
    State(state): State<AppState>,
    Json(login_req): Json<LoginRequest>,
) -> impl IntoResponse {
    match get_and_validate_user(
        login_req.email_address,
        login_req.hashed_password,
        state.database().as_ref(),
    )
    .await
    {
        Ok(user) => (
            StatusCode::OK,
            Json(LoginResponse {
                user: Some(user),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(LoginResponse {
                user: None,
                error: Some(err.to_string()),
            }),
        ),
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email_address: String,
    hashed_password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

async fn get_and_validate_user(
    email: String,
    pass: String,
    db: &DatabaseConnection,
) -> PTResult<User> {
    let user = entity::user::Entity::find_by_id(email.clone())
        .one(db)
        .await?;
    if let Some(user) = user {
        let user: Model = user.into();
        if pass == user.hashed_password {
            Ok(user.into())
        } else {
            tracing::error!("passwords did not match");
            Err(Error::InvalidPassword)
        }
    } else {
        tracing::error!("user with email {} does not exist", &email);
        Err(Error::UserDoesNotExist)
    }
}

pub async fn logout_handler() {
    // auth.logout().await;
}
