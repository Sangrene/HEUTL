use crate::connected_app::connected_app_core::ConnectedAppCore;
use crate::entity_sharing::entity_sharing_model::EntitySharing;
use crate::entity_sharing::entity_sharing_repository::{
    CreateEntitySharingParams, EntitySharingRepository,
};
use crate::shared::errors::Error;

pub struct EntitySharingCore<'a> {
    pub connected_app_core: &'a ConnectedAppCore<'a>,
    pub entity_sharing_repository: Box<dyn EntitySharingRepository + 'a>,
}

impl<'a> EntitySharingCore<'a> {
    pub async fn create_entity_sharing_with_polling(
        &self,
        params: &CreateEntitySharingParams,
    ) -> Result<EntitySharing, Error> {
        let result = self
            .connected_app_core
            .get_connected_app(&params.connected_app_id)
            .await
            .map(async move |_| {
                self.entity_sharing_repository
                    .create_entity_sharing(params)
                    .await
            })?
            .await;
        return result;
    }

    pub async fn get_entity_sharing(&self, id: &String) -> Result<EntitySharing, Error> {
        return self.entity_sharing_repository.get_entity_sharing(id).await;
    }

    pub async fn get_all_polling_entity_sharings(&self) -> Result<Vec<EntitySharing>, Error> {
        return self.entity_sharing_repository.get_all_polling_entity_sharings().await;
    }
}
