use crate::{app::state::AppState, core::error::*, user::model::User};
use axum::{extract::Json, http::StatusCode};
use serde_json::json;
use uuid::Uuid;

use super::model::Metric;

pub async fn handle_create_metric(
    state: AppState,
    metric: Metric,
    user: User,
) -> Result<StatusCode> {
    Ok(StatusCode::CREATED)
}

pub async fn handle_get_metrics(state: AppState, user: User) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Vec::<Metric>::new())))
}

pub async fn handle_get_metric(
    state: AppState,
    id: Uuid,
    user: User,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Option::<Metric>::None)))
}

pub async fn handle_update_metric(
    state: AppState,
    id: Uuid,
    user: User,
    metric: Metric,
) -> Result<Json<serde_json::Value>> {
    Ok(Json(json!(Option::<Metric>::None)))
}

pub async fn handle_delete_metric(state: AppState, id: Uuid, user: User) -> Result<()> {
    Ok(())
}
