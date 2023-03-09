#![allow(dead_code)]
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveModelTrait, TransactionTrait},
};

#[derive(Iden)]
pub enum Metric {
    Table,
    Id,
    Name,
    Description,
    Score1,
    Score2,
    Score3,
    Score4,
}

pub async fn build_metric_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(Metric::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Metric::Id).uuid().not_null().primary_key()
                )
                .col(
                    ColumnDef::new(Metric::Name)
                        .string()
                        .not_null()
                )
                .col(ColumnDef::new(Metric::Description).string())
                .col(ColumnDef::new(Metric::Score1).string().not_null())
                .col(ColumnDef::new(Metric::Score2).string().not_null())
                .col(ColumnDef::new(Metric::Score3).string().not_null())
                .col(ColumnDef::new(Metric::Score4).string().not_null())
                .to_owned(),
        )
        .await
}

pub async fn drop_metric_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(Metric::Table).to_owned())
        .await?;
    Ok(())
}
