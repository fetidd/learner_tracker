use crate::{
    auth::{authorize_token, decode_token, generate_auth_token},
    error::{Error, ErrorKind, Result},
    models::User,
    state::AppState,
};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    Json, TypedHeader,
};
use http::StatusCode;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

pub async fn login_handler(
    State(state): State<AppState>,
    Json(login_req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let user = get_and_validate_user(
        login_req.email_address,
        login_req.hashed_password,
        state.database(),
    )
    .await?;
    let auth_token = generate_auth_token(&user)?;
    Ok(Json(LoginResponse { token: auth_token }))
}

pub async fn logout_handler(
    State(state): State<AppState>,
    TypedHeader(auth_token): TypedHeader<Authorization<Bearer>>,
) -> Result<StatusCode> {
    let decoded = decode_token(auth_token.token())?;
    // get user
    let user = User::one_from_db(&decoded.email_address, state.database()).await?;
    // authorize token
    if let Ok(_) = authorize_token(auth_token.token(), &user.secret) {
        user.refresh_secret(state.database()).await?;
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
    let user = User::one_from_db(&email, db).await;
    if let Ok(user) = user {
        if pass == user.hashed_password {
            Ok(user)
        } else {
            Err(InvalidUserPassword!())
        }
    } else {
        Err(UserDoesNotExist!())
    }
}
