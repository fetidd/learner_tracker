use crate::{app::state::AppState, core::error::*, pupil::model::*, user::model::*};
use axum::{
    extract::Json,
    http::StatusCode,
};
use serde_json::json;
use uuid::Uuid;

pub async fn handle_create_pupil(state: AppState, pupil: Pupil) -> Result<StatusCode> {
    tracing::debug!("creating pupil");
    match pupil.insert(state.database()).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn handle_get_pupils(state: AppState, user: User) -> Result<Json<serde_json::Value>> {
    tracing::debug!("requested all pupils for years {:?}", user.years);
    match Pupil::all(state.database()).await {
        Ok(pupils) => Ok(Json(json!(pupils.into_iter().filter(|p| user.years.contains(&p.year)).collect::<Vec<Pupil>>()))),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn handle_get_pupil(
    state: AppState,
    id: Uuid,
    user: User,
) -> Result<Json<serde_json::Value>> {
    tracing::debug!("requested pupil {id}");
    match Pupil::by_id(id, state.database()).await {
        Ok(pupil) if user.years.contains(&pupil.year) => Ok(Json(json!(pupil))),
        Ok(pupil) => Err(Unauthorised!(format!("you don't have permission to view year {}", pupil.year))),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!(error.to_string())),
            ErrorKind::PupilDoesNotExist => Err(PupilDoesNotExist!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn handle_update_pupil(
    state: AppState,
    id: Uuid,
    user: User,
    update: PupilUpdate,
) -> Result<Json<serde_json::Value>> {
    tracing::debug!("updating pupil {id}");
    let mut pupil = Pupil::by_id(id, state.database()).await?;
    if !user.years.contains(&pupil.year) {
        return Err(Unauthorised!(format!("you don't have permission to modify year {}", pupil.year)));
    }
    pupil.set_from_update(update);
    match pupil.update(state.database().as_ref()).await {
        Ok(pupil) => Ok(Json(json!(pupil))),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!(error.to_string())),
            ErrorKind::PupilDoesNotExist => Err(PupilDoesNotExist!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn handle_delete_pupil(
    state: AppState,
    id: Uuid,
    user: User,
) -> Result<StatusCode> {
    tracing::debug!("deleting pupil {id}");
    let pupil = Pupil::by_id(id, state.database()).await?;
    if !user.years.contains(&pupil.year) {
        return Err(Unauthorised!(format!("you don't have permission to modify year {}", pupil.year)));
    }
    match pupil.delete(state.database()).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!(error.to_string())),
            ErrorKind::PupilDoesNotExist => Err(PupilDoesNotExist!()),
            _ => Err(UnknownError!()),
        },
    }
}
