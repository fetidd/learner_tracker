use crate::{app::state::AppState, core::error::*};
use axum::{extract::Json, http::StatusCode};
use entity::*;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
use serde_json::json;
use uuid::Uuid;

pub async fn handle_create_pupil(state: AppState, pupil: pupil::Model) -> Result<StatusCode> {
    tracing::debug!("creating pupil");
    match pupil
        .into_active_model()
        .save(state.database().as_ref())
        .await
        .map_err(|e| Error::from(e))
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn handle_get_pupils(
    state: AppState,
    user: user::Model,
) -> Result<Json<serde_json::Value>> {
    tracing::debug!("requested all pupils for years {:?}", user.years);
    match pupil::Entity::find()
        .all(state.database().as_ref())
        .await
        .map_err(|e| Error::from(e))
    {
        Ok(pupils) => Ok(Json(json!(pupils))),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!()),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn handle_get_pupil(
    state: AppState,
    id: Uuid,
    user: user::Model,
) -> Result<Json<serde_json::Value>> {
    tracing::debug!("requested pupil {id}");
    match pupil::Entity::find_by_id(id)
        .one(state.database().as_ref())
        .await
        .map_err(|e| Error::from(e))
    {
        Ok(Some(pupil)) if user.years.contains(&pupil.year) => Ok(Json(json!(pupil))),
        Ok(Some(pupil)) => Err(Unauthorised!(format!(
            "you don't have permission to view year {}",
            pupil.year
        ))),
        Ok(None) => Err(PupilDoesNotExist!()),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!(error.to_string())),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn handle_update_pupil(
    state: AppState,
    id: Uuid,
    user: user::Model,
    update: pupil::Model,
) -> Result<Json<serde_json::Value>> {
    tracing::debug!("updating pupil {id}");
    let pupil = pupil::Entity::find_by_id(id)
        .one(state.database().as_ref())
        .await
        .map_err(|e| Error::from(e))?;
    if pupil.is_none() {
        return Err(PupilDoesNotExist!());
    }
    let pupil = pupil.unwrap();
    if !user.years.contains(&pupil.year) {
        return Err(Unauthorised!(format!(
            "you don't have permission to modify year {}",
            pupil.year
        )));
    }
    match update
        .into_active_model()
        .update(state.database().as_ref())
        .await
        .map_err(|e| Error::from(e))
    {
        Ok(pupil) => Ok(Json(json!(pupil))),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!(error.to_string())),
            _ => Err(UnknownError!()),
        },
    }
}

pub async fn handle_delete_pupil(
    state: AppState,
    id: Uuid,
    user: user::Model,
) -> Result<StatusCode> {
    tracing::debug!("deleting pupil {id}");
    let pupil = pupil::Entity::find_by_id(id)
        .one(state.database().as_ref())
        .await?;
    if pupil.is_none() {
        return Err(PupilDoesNotExist!());
    }
    let pupil = pupil.unwrap();
    if !user.years.contains(&pupil.year) {
        return Err(Unauthorised!(format!(
            "you don't have permission to modify year {}",
            pupil.year
        )));
    }
    match pupil::Entity::delete_by_id(pupil.id)
        .exec(state.database().as_ref())
        .await
        .map_err(|e| Error::from(e))
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(error) => match error.kind {
            ErrorKind::DatabaseError => Err(DatabaseError!(error.to_string())),
            ErrorKind::PupilDoesNotExist => Err(PupilDoesNotExist!()),
            _ => Err(UnknownError!()),
        },
    }
}
