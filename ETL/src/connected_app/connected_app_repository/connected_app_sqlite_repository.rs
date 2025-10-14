use crate::connected_app::connected_app_model::ConnectedApp;
use crate::connected_app::connected_app_repository::{
    ConnectedAppRepository, CreateConnectedAppParams,
};
use crate::shared::errors::Error;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::sqlite::SqlitePool;
pub struct ConnectedAppSQLiteRepository<'a> {
    pub pool: &'a SqlitePool,
}

#[async_trait]
impl<'a> ConnectedAppRepository for ConnectedAppSQLiteRepository<'a> {
    async fn create_connected_app(
        &self,
        params: &CreateConnectedAppParams,
    ) -> Result<ConnectedApp, Error> {
        let connected_app = ConnectedApp {
            id: params.id.clone(),
            name: params.name.clone(),
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
        };
        sqlx::query(
            "INSERT INTO connected_apps (id, name, created_at, updated_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(&connected_app.id)
        .bind(&connected_app.name)
        .bind(&connected_app.created_at)
        .bind(&connected_app.updated_at)
        .execute(self.pool)
        .await?;

        Ok(connected_app)
    }

    async fn get_connected_app(&self, id: &String) -> Result<ConnectedApp, Error> {
        let connected_app: ConnectedApp = sqlx::query_as(
            "SELECT * FROM connected_apps WHERE id = $1 LIMIT 1",
        )
        .bind(id)
        .fetch_one(self.pool)
        .await?;
        Ok(connected_app)
    }

    async fn get_all_connected_apps(&self) -> Result<Vec<ConnectedApp>, Error> {
        let connected_apps: Vec<ConnectedApp> = sqlx::query_as(
            "SELECT * FROM connected_apps",
        )
        .fetch_all(self.pool)
        .await?;
        Ok(connected_apps)
    }
}
