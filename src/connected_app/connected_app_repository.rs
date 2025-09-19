use crate::connected_app::connected_app_model::ConnectedApp;
use crate::shared::errors::Error;
use async_trait::async_trait;   
pub mod connected_app_sqlite_repository;

pub struct CreateConnectedAppParams {
    pub id: String,
    pub name: String,
}

#[async_trait]
pub trait ConnectedAppRepository {
    async fn create_connected_app(&self, params: &CreateConnectedAppParams) -> Result<ConnectedApp, Error>;
    async fn get_connected_app(&self, id: &String) -> Result<ConnectedApp, Error>;
}

