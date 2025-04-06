use axum::{Extension, Json, extract::State, http::StatusCode};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::users::{self, Entity as Users, Model},
    utils::{api_error::ApiError, hashing::argon_verify, jwt::create_jwt},
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
