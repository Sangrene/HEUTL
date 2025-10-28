use crate::entity_subscription::entity_subscription_model::EntitySubscription;
use crate::shared::errors::Error;
use async_trait::async_trait;
use serde_json::Value;
pub mod entity_subscription_sqlite_repository;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CreateEntitySubscriptionParams {
    pub id: String,
    pub entity_sharing_id: String,
    pub connected_app_id: String,
    pub jdm_transform: Option<Value>,
    pub python_script: Option<String>,
}

#[async_trait]
pub trait EntitySubscriptionRepository: Send + Sync {
    async fn create_entity_subscription(
        &self,
        params: &CreateEntitySubscriptionParams,
    ) -> Result<EntitySubscription, Error>;
    async fn get_entity_subscription_by_id(&self, id: &String) -> Result<EntitySubscription, Error>;
    async fn get_all_entity_subscriptions_for_entity_sharing(&self, entity_sharing_id: &String) -> Result<Vec<EntitySubscription>, Error>;
}
