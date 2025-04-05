use axum::{Json, extract::State, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

use crate::database::users::Entity as Users;
use crate::{
    database::{tickets, users},
    utils::api_error::ApiError,
};

#[derive(Deserialize, Debug)]
pub struct RequestTicket {
    title: String,
    description: Option<String>,
    priority: Option<String>,
}

pub async fn ticket_create(
    State(db_conn): State<DatabaseConnection>,
    auth: TypedHeader<Authorization<Bearer>>,
    Json(req_ticket): Json<RequestTicket>,
) -> Result<(), ApiError> {
    tracing::info!("Attempting to create ticket...");
    let header_token = auth.token();

    let user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(header_token))
        .one(&db_conn)
        .await
        .map_err(|err| {
            tracing::error!("Error: {:?}", err);
            ApiError::new(StatusCode::UNAUTHORIZED, "Log in to create a ticket")
        })? {
        user
    } else {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Unauthorized, please log in",
        ));
    };

    let ticket = tickets::ActiveModel {
        title: Set(req_ticket.title),
        description: Set(req_ticket.description),
        priority: Set(req_ticket.priority),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    ticket.save(&db_conn).await.map_err(|err| {
        tracing::error!("Error: {:?}", err);
        ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Database error, cannot save ticket",
        )
    })?;

    tracing::info!("New ticket created by User {:?}", user.username);
    Ok(())
}
