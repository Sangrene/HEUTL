use crate::entity_sharing::entity_sharing_model::EntitySharing;
use crate::entity_sharing::entity_sharing_model::EntitySharingPollingInfos;
use crate::shared::errors::Error;
use serde_json::Value;
use async_trait::async_trait;
pub mod entity_sharing_sqlite_repository;



pub struct CreateEntitySharingParams {
    pub id: String,
    pub name: String,
    pub connected_app_id: String,
    pub json_schema: Value,
    pub polling_infos: Option<EntitySharingPollingInfos>,
    pub data_path: Option<String>,
    pub is_array: bool,
    pub python_script: Option<String>,
}

#[async_trait]
pub trait EntitySharingRepository: Send + Sync {
    async fn create_entity_sharing(
        &self,
        params: &CreateEntitySharingParams,
    ) -> Result<EntitySharing, Error>;
    async fn get_entity_sharing(&self, id: &String) -> Result<EntitySharing, Error>;
    async fn get_all_polling_entity_sharings(&self) -> Result<Vec<EntitySharing>, Error>;
    async fn get_all_entity_sharings(&self) -> Result<Vec<EntitySharing>, Error>;
}
