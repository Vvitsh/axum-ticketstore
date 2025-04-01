mod health_check;
mod users;

use health_check::health_check;
use users::create_user;

use axum::{
    Router,
    extract::FromRef,
    http::Method,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone, FromRef)]
pub struct AppState {
    db_conn: DatabaseConnection,
}

pub fn routes(db_conn: DatabaseConnection) -> Router {
    let app_state = AppState { db_conn };

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_origin(Any);

    Router::new()
        .route("/health_check", get(health_check))
        .route("/create_account", post(create_user))
        .with_state(app_state)
        .layer(cors)
}
