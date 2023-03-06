use crate::{core::error::Result, record::model::*};
use chrono::NaiveDate;
use entity::metric::{ActiveModel, Column, Entity, Model};
use migration::Condition;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, Unchanged,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {

}