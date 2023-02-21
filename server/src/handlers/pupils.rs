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
    match pupil.save(state.database().as_ref()).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn get_pupils(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Json<PupilsResponse>> {
    tracing::info!("requested all pupils");
    match Pupil::all_from_db(&user, state.database().as_ref()).await {
        Ok(pupils) => Ok(Json(PupilsResponse { pupils })),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn get_pupil_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Extension(user): Extension<User>,
) -> Result<Json<PupilsResponse>> {
    match Pupil::one_from_db(&user, id, state.database().as_ref()).await {
        Ok(pupil) => Ok(Json(PupilsResponse {
            pupils: vec![pupil],
        })),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!(error.to_string())),
            ErrorKind::PupilDoesNotExist => Err(PupilDoesNotExist!()),
            _ => Err(UnknownError!()),
        },
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PupilsResponse {
    pupils: Vec<Pupil>,
}
