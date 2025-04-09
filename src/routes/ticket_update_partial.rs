use axum::{Extension, Json, extract::Path, http::StatusCode};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set,
    prelude::DateTimeWithTimeZone,
};
use serde::Deserialize;

use crate::{
    database::tickets::{self, Entity as Tickets},
    utils::api_error::ApiError,
};

#[derive(Deserialize)]
pub struct RequestTicket {
    #[allow(dead_code)]
    id: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    priority: Option<Option<String>>,
    title: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    description: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    deleted_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    in_progress: Option<Option<bool>>,
}

pub async fn ticket_update_partial(
    Extension(db_conn): Extension<DatabaseConnection>,
    Path(ticket_id): Path<i32>,
    Json(request_ticket): Json<RequestTicket>,
) -> Result<(), ApiError> {
    let mut db_ticket = if let Some(ticket) = Tickets::find_by_id(ticket_id)
        .one(&db_conn)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch ticket"))?
    {
        ticket.into_active_model()
    } else {
        return Err(ApiError::new(StatusCode::NOT_FOUND, "No ticket found"));
    };

    if let Some(priority) = request_ticket.priority {
        db_ticket.priority = Set(priority);
    }

    if let Some(title) = request_ticket.title {
        db_ticket.title = Set(title);
    }

    if let Some(completed_at) = request_ticket.completed_at {
        db_ticket.completed_at = Set(completed_at);
    }

    if let Some(description) = request_ticket.description {
        db_ticket.description = Set(description);
    }

    if let Some(deleted_at) = request_ticket.deleted_at {
        db_ticket.deleted_at = Set(deleted_at);
    }

    if let Some(in_progress) = request_ticket.in_progress {
        db_ticket.in_progress = Set(in_progress);
    }

    Tickets::update(db_ticket)
        .filter(tickets::Column::Id.eq(ticket_id))
        .exec(&db_conn)
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to update ticket"))?;

    Ok(())
}
