mod pupil;
mod user;
mod utils;

pub use sea_orm_migration::prelude::*;
pub use crate::{user::*, pupil::*};

mod m20220101_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}