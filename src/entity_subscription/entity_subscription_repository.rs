use crate::entity_subscription::entity_subscription_model::EntitySubscription;
use crate::shared::errors::Error;
use async_trait::async_trait;
pub mod entity_subscription_sqlite_repository;

pub struct CreateEntitySubscriptionParams {
    pub id: String,
    pub entity_sharing_id: String,
    pub connected_app_id: String,
}

#[async_trait]
pub trait EntitySubscriptionRepository {
    async fn create_entity_subscription(
        &self,
        params: &CreateEntitySubscriptionParams,
    ) -> Result<EntitySubscription, Error>;
    async fn get_entity_subscription_by_id(&self, id: &String) -> Result<EntitySubscription, Error>;
}
