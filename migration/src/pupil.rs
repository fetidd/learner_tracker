#![allow(dead_code)]
use sea_orm::DatabaseConnection;
use sea_orm_migration::{prelude::*, sea_orm::{ActiveModelTrait, TransactionTrait}};

use crate::utils::generate_pupils;

pub async fn build_pupil_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
    .create_table(
        Table::create()
            .table(Pupil::Table)
            .if_not_exists()
            .col(ColumnDef::new(Pupil::Id).uuid().not_null().primary_key())
            .col(ColumnDef::new(Pupil::FirstNames).string().not_null())
            .col(ColumnDef::new(Pupil::LastName).string().not_null())
            .col(ColumnDef::new(Pupil::Year).integer().not_null())
            .col(ColumnDef::new(Pupil::StartDate).date().not_null())
            .col(ColumnDef::new(Pupil::EndDate).date().not_null())
            .col(
                ColumnDef::new(Pupil::Active)
                    .boolean()
                    .not_null()
                    .default(true),
            )
            .col(
                ColumnDef::new(Pupil::MoreAbleAndTalented)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .col(
                ColumnDef::new(Pupil::EnglishAsAdditionalLanguage)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .col(
                ColumnDef::new(Pupil::FreeSchoolMeals)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .col(
                ColumnDef::new(Pupil::AdditionalLearningNeeds)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .col(
                ColumnDef::new(Pupil::LookedAfterChild)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .col(ColumnDef::new(Pupil::Gender).string().not_null())
            .to_owned(),
    )
    .await
}

pub async fn drop_pupil_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(Pupil::Table).to_owned())
        .await?;
    Ok(())
}

pub async fn seed_pupils(db: &DatabaseConnection) -> Result<(), DbErr> {
    let trx = db.begin().await?;
    for pupil in generate_pupils(300) {
        pupil.insert(&trx).await?; // TODO speed this up by making one insert rather than 1 trxn?
    }
    trx.commit().await?;
    Ok(())
}

#[derive(Iden)]
enum Pupil {
    Table,
    Id,
    FirstNames,
    LastName,
    Year,
    StartDate,
    EndDate,
    Active,
    MoreAbleAndTalented,
    EnglishAsAdditionalLanguage,
    FreeSchoolMeals,
    AdditionalLearningNeeds,
    LookedAfterChild,
    Gender,
}