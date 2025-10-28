use crate::shared::bus::{Commands, TopicIds};

use crate::connected_app::connected_app_core::ConnectedAppCore;
use crate::entity_sharing::entity_sharing_model::EntitySharing;
use crate::entity_sharing::entity_sharing_repository::{
    CreateEntitySharingParams, EntitySharingRepository, UpdateEntitySharingParams,
};
use crate::shared::errors::Error;
use crate::shared::merge_struct::Merge;
use std::sync::Arc;

pub struct EntitySharingCore<'a> {
    pub connected_app_core: Arc<ConnectedAppCore<'a>>,
    pub entity_sharing_repository: Box<dyn EntitySharingRepository + 'a>,
    pub publish: Box<dyn Fn(Commands, Option<TopicIds>) -> () + Send + Sync>,
}

impl<'a> EntitySharingCore<'a> {
    pub fn new(
        connected_app_core: Arc<ConnectedAppCore<'a>>,
        entity_sharing_repository: Box<dyn EntitySharingRepository + 'a>,
        publish: Box<dyn Fn(Commands, Option<TopicIds>) -> () + Send + Sync>,
    ) -> Self {
        Self {
            connected_app_core,
            entity_sharing_repository,
            publish,
        }
    }
    pub async fn create_entity_sharing(
        &self,
        params: &CreateEntitySharingParams,
    ) -> Result<EntitySharing, Error> {
        let connected_app_core = self.connected_app_core.clone();
        let entity_sharing_repository = &self.entity_sharing_repository;

        let result = connected_app_core
            .get_connected_app(&params.connected_app_id)
            .await
            .map(async move |_| {
                entity_sharing_repository
                    .create_entity_sharing(params)
                    .await
            })?
            .await?;
        (self.publish)(
            Commands::EntitySharingCreated {
                entity_sharing: (result.clone()),
            },
            Some(TopicIds::EntitySharingCreated),
        );
        return Ok(result);
    }

    pub async fn update_entity_sharing(
        &self,
        id: &String,
        params: &UpdateEntitySharingParams,
    ) -> Result<EntitySharing, Error> {
        let current_entity_sharing = self.get_entity_sharing(id).await?;
        let updated_entity_sharing = current_entity_sharing.merge(params.clone());
        let _rows_affected = self
            .entity_sharing_repository
            .update_entity_sharing(&updated_entity_sharing)
            .await?;
        return Ok(updated_entity_sharing);
    }

    pub async fn get_entity_sharing(&self, id: &String) -> Result<EntitySharing, Error> {
        return self.entity_sharing_repository.get_entity_sharing(id).await;
    }

    pub async fn get_all_entity_sharings(&self) -> Result<Vec<EntitySharing>, Error> {
        return self
            .entity_sharing_repository
            .get_all_entity_sharings()
            .await;
    }

    pub async fn get_all_polling_entity_sharings(&self) -> Result<Vec<EntitySharing>, Error> {
        return self
            .entity_sharing_repository
            .get_all_polling_entity_sharings()
            .await;
    }
}
