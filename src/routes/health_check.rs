use axum::extract::State;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr};

use crate::utils::api_error::ApiError;

#[allow(dead_code)]
async fn db_check(db_conn: DatabaseConnection) -> Result<(), DbErr> {
    db_conn.execute_unprepared("SELECT 1").await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn health_check(State(_db_conn): State<DatabaseConnection>) -> Result<(), ApiError> {
    todo!()
}
