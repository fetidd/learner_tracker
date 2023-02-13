use dotenv::dotenv;
use sea_orm_migration::prelude::*;
mod pupil;
mod user;
mod utils;

#[async_std::main]
async fn main() {
    dotenv().ok();
    cli::run_cli(migration::Migrator).await;
}
