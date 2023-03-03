use crate::{app::state::AppState, auth::token::*, core::error::*, user::model::*};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    Json, TypedHeader,
};
use http::StatusCode;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub async fn login_handler(
    State(state): State<AppState>,
    Json(login_req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    debug!("login request for {}", login_req.email_address);
    let user = get_and_validate_user(
        login_req.email_address,
        login_req.hashed_password,
        state.database(),
    )
    .await?;
    debug!("generating auth token");
    let auth_token = generate_auth_token(&user)?;
    debug!("responding with token for user {}", user.email_address);
    Ok(Json(LoginResponse { token: auth_token }))
}

pub async fn logout_handler(
    State(state): State<AppState>,
    TypedHeader(auth_token): TypedHeader<Authorization<Bearer>>,
) -> Result<StatusCode> {
    let decoded = decode_token(auth_token.token())?;
    debug!("logout request for {}", decoded.email_address);
    let user: User = User::one_from_db(&decoded.email_address, state.database()).await?;
    user.refresh_secret(state.database()).await?;
    debug!("refreshed secret for {}", decoded.email_address);
    if let Ok(_) = authorize_token(auth_token.token(), &user.secret) {
        Ok(StatusCode::OK)
    } else {
        Err(InvalidJwt!())
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email_address: String,
    hashed_password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

async fn get_and_validate_user(
    email: String,
    pass: String,
    db: &DatabaseConnection,
) -> Result<User> {
    let user: Result<User> = User::one_from_db(&email, db).await;
    if let Ok(user) = user {
        if pass == user.hashed_password {
            Ok(user)
        } else {
            Err(InvalidCredentials!())
        }
    } else {
        Err(UserDoesNotExist!())
    }
}
