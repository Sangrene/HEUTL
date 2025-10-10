use crate::shared::bus::{Commands, TopicIds};
use pubsub_bus::{EventEmitter, Publisher};

use crate::connected_app::connected_app_core::ConnectedAppCore;
use crate::entity_sharing::entity_sharing_model::EntitySharing;
use crate::entity_sharing::entity_sharing_repository::{
    CreateEntitySharingParams, EntitySharingRepository,
};
use crate::shared::errors::Error;
use std::sync::Arc;

pub struct EntitySharingCore<'a> {
    emitter: EventEmitter<Commands, TopicIds>,
    pub connected_app_core: Arc<ConnectedAppCore<'a>>,
    pub entity_sharing_repository: Box<dyn EntitySharingRepository + 'a>,
}

impl<'a> EntitySharingCore<'a> {
    pub fn new(
        emitter: EventEmitter<Commands, TopicIds>,
        connected_app_core: Arc<ConnectedAppCore<'a>>,
        entity_sharing_repository: Box<dyn EntitySharingRepository + 'a>,
    ) -> Self {
        Self {
            emitter,
            connected_app_core,
            entity_sharing_repository,
        }
    }
    pub async fn create_entity_sharing_with_polling(
        &mut self,
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
        self.emitter.publish(
            Commands::EntitySharingCreated {
                entity_sharing: (result.clone()),
            },
            Some(TopicIds::EntitySharingCreated),
        );
        return Ok(result);
    }

    pub async fn get_entity_sharing(&self, id: &String) -> Result<EntitySharing, Error> {
        return self.entity_sharing_repository.get_entity_sharing(id).await;
    }

    pub async fn get_all_entity_sharings(&self) -> Result<Vec<EntitySharing>, Error> {
        return self.entity_sharing_repository.get_all_entity_sharings().await;
    }

    pub async fn get_all_polling_entity_sharings(&self) -> Result<Vec<EntitySharing>, Error> {
        return self
            .entity_sharing_repository
            .get_all_polling_entity_sharings()
            .await;
    }
}

impl<'a> Publisher<Commands, TopicIds> for EntitySharingCore<'a> {
    fn get_mut_emitter(&mut self) -> &mut EventEmitter<Commands, TopicIds> {
        &mut self.emitter
    }
}
