use crate::shared::errors::Error;
use sqlx::sqlite::SqlitePool;

pub async fn get_db() -> Result<SqlitePool, Error> {
    let pool = SqlitePool::connect("sqlite:file:in-memory-db?mode=memory&cache=shared").await?;
    sqlx::migrate!("./src/shared/migrations")
        .run(&pool)
        .await
        .map_err(|e| Error::DatabaseError(e.to_string()))?;
    Ok(pool)
}
