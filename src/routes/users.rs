use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{PasswordHash, PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Extension, Json, extract::State, http::StatusCode};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::users::{self, Entity as Users, Model},
    utils::{api_error::ApiError, jwt::create_jwt},
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

fn argon_hash(password: String) -> Result<String, ApiError> {
    let pw_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(pw_bytes, &salt)
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Hash error"));

    Ok(hash.unwrap().to_string())
}

fn argon_verify(password: String, hash: &str) -> Result<bool, ApiError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_err| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal error"));

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash.unwrap())
        .map_err(|_err| ApiError::new(StatusCode::UNAUTHORIZED, "Incorrect password"))?;

    Ok(true)
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

pub async fn login(
    State(db_conn): State<DatabaseConnection>,
    Json(req_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, ApiError> {
    let db_search = Users::find()
        .filter(users::Column::Username.eq(&req_user.username))
        .one(&db_conn)
        .await
        .map_err(|_err| ApiError::new(StatusCode::UNAUTHORIZED, "Invalid username"))?;

    if let Some(user_model) = db_search {
        if !argon_verify(req_user.password, &user_model.password)? {
            return Err(ApiError::new(StatusCode::UNAUTHORIZED, "Invalid password"));
        }

        let token = create_jwt()?;

        let mut user = user_model.into_active_model();

        user.token = Set(Some(token));

        let save_user = user.save(&db_conn).await.map_err(|err| {
            tracing::error!("Error: {:?}", err);
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal error")
        })?;

        Ok(Json(ResponseUser {
            id: save_user.id.unwrap(),
            username: save_user.username.unwrap(),
            token: save_user.token.unwrap().unwrap(),
        }))
    } else {
        Err(ApiError::new(
            StatusCode::NOT_FOUND,
            "Requested user not found",
        ))
    }
}

pub async fn logout(
    State(db_conn): State<DatabaseConnection>,
    Extension(user_model): Extension<Model>,
) -> Result<(), ApiError> {
    let mut user = user_model.into_active_model();

    user.token = Set(None);

    user.save(&db_conn).await.map_err(|err| {
        tracing::error!("Error: {:?}", err);
        ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to update user")
    })?;

    Ok(())
}
