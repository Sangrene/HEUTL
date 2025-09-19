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
    pub jdm_transform: Option<Value>,
    pub json_schema: Value,
    pub polling_infos: Option<EntitySharingPollingInfos>,
}

#[async_trait]
pub trait EntitySharingRepository {
    async fn create_entity_sharing(
        &self,
        params: &CreateEntitySharingParams,
    ) -> Result<EntitySharing, Error>;
    async fn get_entity_sharing(&self, id: &String) -> Result<EntitySharing, Error>;
}
