#![allow(dead_code)]
use entity::user::ActiveModel;
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
            .to_owned(),
        ).await
}

pub async fn drop_user_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(User::Table).to_owned())
        .await?;
    Ok(())
}

pub async fn seed_users(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let db = manager.get_connection();
    ActiveModel {
        first_names: Set("Ben".into()),
        last_name: Set("Jones".into()),
        email_address: Set("fetiddius@gmail.com".into()),
        hashed_password: Set("hashedpassword".into()),
        years: Set("5,6".into()),
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
}