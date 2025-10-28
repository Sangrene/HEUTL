use crate::entity_sharing::entity_sharing_model::EntitySharing;
use crate::entity_sharing::entity_sharing_model::EntitySharingPollingInfos;
use crate::shared::errors::Error;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub mod entity_sharing_sqlite_repository;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CreateEntitySharingParams {
    pub id: String,
    pub name: String,
    pub connected_app_id: String,
    pub json_schema: Value,
    pub polling_infos: Option<EntitySharingPollingInfos>,
    pub is_array: bool,
    pub python_script: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct UpdateEntitySharingParams {
    pub name: Option<String>,
    pub polling_infos: Option<EntitySharingPollingInfos>,
    pub python_script: Option<String>,
    pub is_array: Option<bool>,
    pub json_schema: Option<Value>,
}

#[async_trait]
pub trait EntitySharingRepository: Send + Sync {
    async fn create_entity_sharing(
        &self,
        params: &CreateEntitySharingParams,
    ) -> Result<EntitySharing, Error>;
    async fn get_entity_sharing(&self, id: &String) -> Result<EntitySharing, Error>;
    async fn get_all_polling_entity_sharings(&self) -> Result<Vec<EntitySharing>, Error>;
    async fn update_entity_sharing(&self, entity_sharing: &EntitySharing) -> Result<u64, Error>;
    async fn get_all_entity_sharings(&self) -> Result<Vec<EntitySharing>, Error>;
}
