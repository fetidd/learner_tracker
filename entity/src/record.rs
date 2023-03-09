use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "record")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub pupil: Uuid,
    pub metric: Uuid,
    pub score: u32,
    pub note: String, 
    pub date: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::pupil::Entity")]
    Pupil,
    #[sea_orm(has_one = "super::metric::Entity")]
    Metric,
}

impl Related<super::pupil::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pupil.def()
    }
}

impl Related<super::metric::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Metric.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}