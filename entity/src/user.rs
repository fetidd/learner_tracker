use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    pub first_names: String,
    pub last_name: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub email_address: String,
    pub hashed_password: String,
    pub years: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
