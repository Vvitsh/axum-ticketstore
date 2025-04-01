use axum::extract::State;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr};

use crate::utils::api_error::ApiError;

async fn db_check(db_conn: DatabaseConnection) -> Result<(), DbErr> {
    db_conn.execute_unprepared("SELECT 1").await?;
    Ok(())
}

pub async fn health_check(State(db_conn): State<DatabaseConnection>) -> Result<(), ApiError> {
    todo!()
}
