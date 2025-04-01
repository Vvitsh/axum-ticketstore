mod middleware;
mod routes;
mod utils;

use routes::routes;
use sea_orm::Database;
use tokio::net::TcpListener;

pub async fn run(db_uri: &str) {
    let db_conn = Database::connect(db_uri).await.unwrap();

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app_router = routes(db_conn);

    axum::serve(listener, app_router.into_make_service())
        .await
        .unwrap();

    todo!()
}
