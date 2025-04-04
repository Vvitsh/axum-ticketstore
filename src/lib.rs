mod database;
mod middleware;
mod routes;
mod utils;

use routes::routes;
use sea_orm::Database;
use tokio::net::TcpListener;

pub async fn run(db_uri: &str) {
    utils::logging::init_tracing();
    tracing::info!("Initialized server");

    let db_conn = Database::connect(db_uri).await.unwrap();
    tracing::info!("Database connected");

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    let app_router = routes(db_conn);

    axum::serve(listener, app_router.into_make_service())
        .await
        .unwrap()
}
