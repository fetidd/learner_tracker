use crate::{core::error::Result, pupil::model::*, metric::model::*};
use chrono::NaiveDate;
use entity::metric::{ActiveModel, Column, Entity, Model};
use migration::Condition;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, Unchanged,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub id: Uuid,
    pub pupil: Uuid,
    pub metric: String,
    pub score: u32,
    pub note: String, 
    pub date: NaiveDate,
}