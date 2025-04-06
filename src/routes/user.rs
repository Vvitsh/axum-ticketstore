use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use crate::{
    database::users::{self},
    utils::{api_error::ApiError, hashing::argon_hash, jwt::create_jwt},
};

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct ResponseUser {
    id: i32,
    username: String,
    token: String,
}

pub async fn create_user(
    State(db_conn): State<DatabaseConnection>,
    Json(req_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, ApiError> {
    tracing::info!("Attempting to create a new user...");
    let jwt = create_jwt().map_err(|err| {
        tracing::error!("Error: {:?}", err);
        ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Error")
    })?;

    let new_user = users::ActiveModel {
        username: Set(req_user.username),
        password: Set(argon_hash(req_user.password)?),
        token: Set(Some(jwt)),
        ..Default::default()
    }
    .save(&db_conn)
    .await
    .map_err(|err| {
        tracing::error!("Error: {:?}", err);
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to create new user at this time, please try again later",
        )
    })?;

    tracing::info!("New user created: {:?}", &new_user);
    Ok(Json(ResponseUser {
        id: new_user.id.unwrap(),
        username: new_user.username.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}
