use crate::{
    app_state::AppState,
    error::{Error, ErrorKind, Result},
    models::User,
};
use axum::{extract::State, Json};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

pub async fn login_handler(
    State(state): State<AppState>,
    Json(login_req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let user = get_and_validate_user(
        login_req.email_address,
        login_req.hashed_password,
        state.database().as_ref(),
    )
    .await?;
    Ok(Json(LoginResponse { user: Some(user) }))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email_address: String,
    hashed_password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    user: Option<User>,
}

async fn get_and_validate_user(
    email: String,
    pass: String,
    db: &DatabaseConnection,
) -> Result<User> {
    let user = User::one_from_db(&email, db).await;
    if let Ok(user) = user {
        if pass == user.hashed_password {
            Ok(user)
        } else {
            tracing::error!("passwords did not match");
            Err(Error {
                kind: ErrorKind::InvalidUserPassword,
                message: "passwords did not match".into(),
            })
        }
    } else {
        tracing::error!("user with email {} does not exist", &email);
        Err(Error {
            kind: ErrorKind::UserDoesNotExist,
            message: format!("user with email {} does not exist", &email),
        })
    }
}

pub async fn logout_handler() {
    // auth.logout().await;
}
