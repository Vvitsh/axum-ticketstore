mod health_check;
mod ticket_create;
mod ticket_get;
mod users;

use health_check::health_check;
use ticket_create::ticket_create;
use ticket_get::{ticket_get_all, ticket_get_single};
use users::{create_user, login, logout};

use axum::{
    Router,
    extract::FromRef,
    http::Method,
    middleware,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

use crate::middleware::guard::guard;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
}

pub fn routes(db_conn: DatabaseConnection) -> Router {
    let app_state = AppState { db_conn };

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            // Method::PUT,
            // Method::DELETE,
        ])
        .allow_origin(Any);

    // FIX: nesting routes
    // let user_routes = Router::new()
    // .route("/logout", post(logout))
    // .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
    // .route("/create_user", post(create_user))
    // .route("/login", post(login));

    // .nest("/users", user_routes)
    Router::new()
        // Auth routes
        .route("/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
        .route("/create_user", post(create_user))
        .route("/login", post(login))
        // Ticket routes
        .route("/tickets", post(ticket_create))
        .route("/tickets", get(ticket_get_all))
        .route("/tickets/{id}", get(ticket_get_single))
        // Health check routes
        .route("/health_check", get(health_check))
        .with_state(app_state)
        .layer(cors)
}
