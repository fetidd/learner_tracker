use crate::{app::state::AppState, core::error::*};
use axum::{extract::Json, http::StatusCode};
use serde_json::json;
use uuid::Uuid;
use entity::*;

pub async fn handle_create_record(
    _state: AppState,
    _metric: record::Model,
    _user: user::Model,
) -> Result<StatusCode> {
    Ok(StatusCode::CREATED)
}

pub async fn handle_get_records(_state: AppState, _user: user::Model) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Vec::<record::Model>::new())))
}

pub async fn handle_get_record(
    _state: AppState,
    _id: Uuid,
    _user: user::Model,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Option::<record::Model>::None)))
}

pub async fn handle_update_record(
    _state: AppState,
    _id: Uuid,
    _user: user::Model,
    _metric: record::Model,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Option::<record::Model>::None)))
}

pub async fn handle_delete_record(_state: AppState, _id: Uuid, _user: user::Model) -> Result<()> {
    Ok(())
}