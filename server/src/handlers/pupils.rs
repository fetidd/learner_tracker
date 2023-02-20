use crate::{
    error::*,
    models::{Pupil, User},
    state::AppState,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    Extension,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn create_pupil(
    State(state): State<AppState>,
    Json(pupil): Json<Pupil>,
) -> Result<StatusCode> {
    pupil.save(state.database().as_ref()).await?;
    Ok(StatusCode::CREATED)
}

pub async fn get_pupils(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Json<PupilsResponse>> {
    tracing::info!("requested all pupils");
    let pupils = Pupil::all_from_db(&user, state.database().as_ref()).await?;
    Ok(Json(PupilsResponse { pupils }))
}

pub async fn get_pupil_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Extension(user): Extension<User>,
) -> Result<Json<PupilsResponse>> {
    let pupil = Pupil::one_from_db(&user, id, state.database().as_ref()).await?;
    Ok(Json(PupilsResponse {
        pupils: vec![pupil],
    }))
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PupilsResponse {
    pupils: Vec<Pupil>,
}
