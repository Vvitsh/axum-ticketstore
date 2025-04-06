use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Json, extract::State};
use chrono::{DateTime, FixedOffset};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use crate::database::tickets::Entity as Tickets;
use crate::utils::api_error::ApiError;

#[derive(Serialize)]
pub struct ResponseTicket {
    id: i32,
    title: String,
    description: Option<String>,
    priority: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
    user_id: Option<i32>,
}

// #[derive(Deserialize)]
// pub struct FilterTicketsParams {
//     priority: Option<String>,
// }

// FIX: impl query filtering and remove deleted tics
pub async fn ticket_get_all(
    State(db_conn): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTicket>>, ApiError> {
    let tickets = Tickets::find()
        .all(&db_conn)
        .await
        .map_err(|err| {
            tracing::error!("Error: {:?}", err);
            ApiError::new(StatusCode::UNAUTHORIZED, "Error fetching tickets")
        })?
        .into_iter()
        .map(|ticket| ResponseTicket {
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

pub async fn ticket_get_single(
    State(db_conn): State<DatabaseConnection>,
    Path(ticket_id): Path<i32>,
) -> Result<Json<ResponseTicket>, ApiError> {
    let ticket = Tickets::find_by_id(ticket_id)
        //
        .one(&db_conn)
        .await
        .unwrap();

    if let Some(ticket) = ticket {
        Ok(Json(ResponseTicket {
            id: ticket.id,
            title: ticket.title,
            description: ticket.description,
            priority: ticket.priority,
            deleted_at: ticket.deleted_at,
            user_id: ticket.user_id,
        }))
    } else {
        Err(ApiError::new(
            StatusCode::NOT_FOUND,
            "Requested ticket not found",
        ))
    }
}
