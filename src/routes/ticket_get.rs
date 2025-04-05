use axum::http::StatusCode;
use axum::{Json, extract::State};
use chrono::{DateTime, FixedOffset};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::database::tickets::Entity as Tickets;
use crate::utils::api_error::ApiError;

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    description: Option<String>,
    priority: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
    user_id: Option<i32>,
}

// #[derive(Deserialize)]
// pub struct FilterTasksParams {
//     priority: Option<String>,
// }

// FIX: impl filtering for query and remove deleted
pub async fn ticket_get_all(
    State(db_conn): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTask>>, ApiError> {
    let tickets = Tickets::find()
        .all(&db_conn)
        .await
        .map_err(|err| {
            tracing::error!("Error: {:?}", err);
            ApiError::new(StatusCode::UNAUTHORIZED, "Error fetching tickets")
        })?
        .into_iter()
        .map(|ticket| ResponseTask {
            id: ticket.id,
            title: ticket.title,
            description: ticket.description,
            priority: ticket.priority,
            deleted_at: ticket.deleted_at,
            user_id: ticket.user_id,
        })
        .collect();

    Ok(Json(tickets))
}

pub async fn ticket_get_single() {}
