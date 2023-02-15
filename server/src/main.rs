use migration::MigratorTrait;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> learner_tracker_server::error::LTResult<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let db = Arc::new(sea_orm::Database::connect(std::env::var("DATABASE_URL")?).await?);
    migration::Migrator::up(db.as_ref(), None).await?;
    migration::seed_database(db.as_ref()).await; // TODO make this only happen for DEV environment
    let app_state: learner_tracker_server::app_state::AppState = Arc::new(learner_tracker_server::app_state::AppStateObj::new(db));
    let address: SocketAddr = std::env::var("SERVER_ADDR")?.parse()?;
    axum::Server::bind(&address)
        .serve(
            learner_tracker_server::router::router()
                .layer(TraceLayer::new_for_http())
                .with_state(app_state)
                .into_make_service(),
        )
        .await?;
    Ok(())
}


