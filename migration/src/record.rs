#![allow(dead_code)]
use super::{metric::Metric, pupil::Pupil};
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveModelTrait, TransactionTrait},
};

#[derive(Iden)]
pub enum Record {
    Table,
    Id,
    Pupil,
    Metric,
    Score,
    Note,
    Date,
}

pub async fn build_record_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .table(Record::Table)
                .if_not_exists()
                .col(ColumnDef::new(Record::Id).uuid().not_null().primary_key())
                .col(ColumnDef::new(Record::Pupil).uuid().not_null())
                .col(ColumnDef::new(Record::Metric).uuid().not_null())
                .col(ColumnDef::new(Record::Score).integer().not_null())
                .col(ColumnDef::new(Record::Note).string())
                .col(ColumnDef::new(Record::Date).date().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_record_pupil")
                        .from(Record::Table, Record::Pupil)
                        .to(Pupil::Table, Pupil::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_record_metric")
                        .from(Record::Table, Record::Metric)
                        .to(Metric::Table, Metric::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
}

pub async fn drop_record_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(Record::Table).to_owned())
        .await?;
    Ok(())
}
