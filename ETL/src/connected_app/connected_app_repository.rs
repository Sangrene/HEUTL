use crate::connected_app::connected_app_model::ConnectedApp;
use crate::shared::errors::Error;
use async_trait::async_trait;   
use serde::{Deserialize, Serialize};
pub mod connected_app_sqlite_repository;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CreateConnectedAppParams {
    pub id: String,
    pub name: String,
}

#[async_trait]
pub trait ConnectedAppRepository: Send + Sync {
    async fn create_connected_app(&self, params: &CreateConnectedAppParams) -> Result<ConnectedApp, Error>;
    async fn get_connected_app(&self, id: &String) -> Result<ConnectedApp, Error>;
    async fn get_all_connected_apps(&self) -> Result<Vec<ConnectedApp>, Error>;
}

