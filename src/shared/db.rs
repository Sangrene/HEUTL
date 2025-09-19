use crate::shared::errors::Error;
use sqlx::sqlite::SqlitePool;

pub struct DB {
    pool: SqlitePool,
}

impl<'a> DB {
    pub async fn new() -> Result<DB, Error> {
        let pool = SqlitePool::connect("sqlite:file:in-memory-db?mode=memory&cache=shared").await?;
        sqlx::migrate!("./src/shared/migrations")
            .run(&pool)
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?;
        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
}
