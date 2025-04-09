use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, prelude::DateTimeWithTimeZone,
};
use serde::Deserialize;

use crate::{
    database::tickets::{self, Entity as Tickets},
    utils::api_error::ApiError,
};

#[derive(Deserialize)]
pub struct RequestTask {
    #[allow(dead_code)]
    id: Option<i32>,
    priority: Option<String>,
    title: String,
    completed_at: Option<DateTimeWithTimeZone>,
    description: Option<String>,
    deleted_at: Option<DateTimeWithTimeZone>,
    user_id: Option<i32>,
    in_progress: Option<bool>,
}

pub async fn ticket_update_atomic(
    State(db_conn): State<DatabaseConnection>,
    Path(ticket_id): Path<i32>,
    Json(req_ticket): Json<RequestTask>,
) -> Result<(), ApiError> {
    tracing::info!("Attempting to update ticket {:?}", &ticket_id);
    let atomic_ticket = tickets::ActiveModel {
        id: Set(ticket_id),
        priority: Set(req_ticket.priority),
        title: Set(req_ticket.title),
        completed_at: Set(req_ticket.completed_at),
        description: Set(req_ticket.description),
        deleted_at: Set(req_ticket.deleted_at),
        user_id: Set(req_ticket.user_id),
        in_progress: Set(req_ticket.in_progress),
    };

    Tickets::update(atomic_ticket)
        .filter(tickets::Column::Id.eq(ticket_id))
        .exec(&db_conn)
        .await
        .map_err(|err| {
            tracing::error!("Error: {:?}", err);
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to update ticket")
        })?;

    tracing::info!("Ticket updated");
    Ok(())
}
