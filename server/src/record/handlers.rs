use crate::{app::state::AppState, core::error::*, user::model::User};
use axum::{extract::Json, http::StatusCode};
use serde_json::json;
use uuid::Uuid;

use super::model::Record;

pub async fn handle_create_record(
    state: AppState,
    metric: Record,
    user: User,
) -> Result<StatusCode> {
    Ok(StatusCode::CREATED)
}

pub async fn handle_get_records(state: AppState, user: User) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Vec::<Record>::new())))
}

pub async fn handle_get_record(
    state: AppState,
    id: Uuid,
    user: User,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Option::<Record>::None)))
}

pub async fn handle_update_record(
    state: AppState,
    id: Uuid,
    user: User,
    metric: Record,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Option::<Record>::None)))
}

pub async fn handle_delete_record(state: AppState, id: Uuid, user: User) -> Result<()> {
    Ok(())
}