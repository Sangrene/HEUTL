use crate::entity_sharing::entity_sharing_core::EntitySharingCore;
use crate::entity_subscription::entity_subscription_model::EntitySubscription;
use crate::entity_subscription::entity_subscription_repository::{
    CreateEntitySubscriptionParams, EntitySubscriptionRepository,
};
use crate::shared::errors::Error;

pub struct EntitySubscriptionCore<'a> {
    pub entity_subscription_repository: Box<dyn EntitySubscriptionRepository + 'a>,
    pub entity_sharing_core: &'a EntitySharingCore<'a>,
}

impl<'a> EntitySubscriptionCore<'a> {
    pub async fn create_entity_subscription(
        &self,
        params: &CreateEntitySubscriptionParams,
    ) -> Result<EntitySubscription, Error> {
        let result = self
            .entity_sharing_core
            .get_entity_sharing(&params.entity_sharing_id)
            .await
            .map(async move |_| {
                let result =self.entity_subscription_repository
                    .create_entity_subscription(params).await;
                return result;
            })?.await;
        return result;
    }

    pub async fn get_entity_subscription_by_id(
        &self,
        id: &String,
    ) -> Result<EntitySubscription, Error> {
        return self
            .entity_subscription_repository
            .get_entity_subscription_by_id(id)
            .await;
    }
}
