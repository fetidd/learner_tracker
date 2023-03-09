use crate::{app::state::AppState, core::error::*, record::model::*};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    Extension,
};
use serde_json::json;
use uuid::Uuid;

use super::model::Metric;

pub async fn create_metric(
    State(app_state): State<AppState>,
    Json(metric_data): Json<Metric>,
) -> Result<StatusCode> {
    Ok(handle_create_metric(app_state, metric_data).await?)
}

async fn handle_create_metric(app_state: AppState, metric_data: Metric) -> Result<StatusCode> {
    Ok(StatusCode::CREATED)
}




