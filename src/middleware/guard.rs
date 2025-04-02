use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::headers::{Authorization, HeaderMapExt, authorization::Bearer};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{database::users, utils::api_error::ApiError};
use crate::{database::users::Entity as Users, utils::jwt::is_valid};

pub async fn guard(mut request: Request<Body>, next: Next) -> Result<Response, ApiError> {
    // dbg!(request
    //     .headers()
    //     .typed_get::<Authorization<Bearer>>()
    //     .unwrap());

    // Get the bearer token from the request headers
    let req_token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or_else(|| ApiError::new(StatusCode::BAD_REQUEST, "Missing bearer token"))?
        .token()
        .to_owned();

    // Connect to db
    let db = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or_else(|| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal error"))?;

    // Find the associated user based on their token
    let user = Users::find()
        .filter(users::Column::Token.eq(Some(req_token.clone())))
        .one(db)
        .await
        .map_err(|_err| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal error"))?;

    let Some(user_model) = user else {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Unauthorized, please log in",
        ));
    };

    // Validate the token
    // Validate post db connection to obfuscate delay in case of timing attack
    is_valid(&req_token)?;

    request.extensions_mut().insert(user_model);

    Ok(next.run(request).await)
}
