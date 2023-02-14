use sea_orm_migration::prelude::*;

use crate::{
    build_pupil_table, build_user_table, drop_pupil_table, drop_user_table
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        build_pupil_table(manager).await?;
        build_user_table(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_pupil_table(manager).await?;
        drop_user_table(manager).await?;
        Ok(())
    }
}
