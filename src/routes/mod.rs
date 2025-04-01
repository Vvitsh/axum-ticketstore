mod health_check;

use health_check::health_check;

use axum::{Router, extract::FromRef, http::Method, routing::get};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone, FromRef)]
pub struct AppState {
    db_conn: DatabaseConnection,
}

pub fn routes(db_conn: DatabaseConnection) -> Router {
    let app_state = AppState { db_conn };

    let cors = CorsLayer::new().allow_origin(Any).allow_methods([
        Method::GET,
        Method::POST,
        Method::PATCH,
        Method::PUT,
        Method::DELETE,
    ]);

    Router::new()
        .route("/health_check", get(health_check))
        .with_state(app_state)
        .layer(cors)
}
