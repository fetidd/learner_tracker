#![allow(dead_code)]
use chrono::Utc;
use entity::user::ActiveModel;
use sea_orm::DatabaseConnection;
use sea_orm_migration::{prelude::*, sea_orm::{Set, ActiveModelTrait}};

pub async fn build_user_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
        Table::create()
            .table(User::Table)
            .col(ColumnDef::new(User::FirstNames).string().not_null())
            .col(ColumnDef::new(User::LastName).string().not_null())
            .col(
                ColumnDef::new(User::EmailAddress)
                    .string()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(User::HashedPassword).string().not_null())
            .col(ColumnDef::new(User::Years).string().not_null().default(""))
            .col(ColumnDef::new(User::Secret).blob(BlobSize::Tiny).not_null())
            .col(ColumnDef::new(User::LastRefresh).date_time().not_null())
            .to_owned(),
        ).await
}

pub async fn drop_user_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(User::Table).to_owned())
        .await?;
    Ok(())
}

pub async fn seed_users(db: &DatabaseConnection) -> Result<(), DbErr> {
    ActiveModel {
        first_names: Set("test".into()),
        last_name: Set("user".into()),
        email_address: Set("test@test.com".into()),
        hashed_password: Set("password".into()),
        years: Set("1,6".into()),
        secret: Set(vec![127; 64]),
        last_refresh: Set(Utc::now().naive_local())
    }.insert(db).await?;
    Ok(())
}

#[derive(Iden)]
enum User {
    Table,
    FirstNames,
    LastName,
    EmailAddress,
    HashedPassword,
    Years,
    Secret,
    LastRefresh,
}