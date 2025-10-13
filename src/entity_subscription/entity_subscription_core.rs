use crate::entity_sharing::entity_sharing_core::EntitySharingCore;
use crate::entity_subscription::entity_subscription_model::EntitySubscription;
use crate::entity_subscription::entity_subscription_repository::{
    CreateEntitySubscriptionParams, EntitySubscriptionRepository,
};
use crate::shared::errors::Error;
use crate::shared::python_runner::run_python_script_output_json;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use futures::future;

pub struct EntitySubscriptionCore<'a> {
    pub entity_subscription_repository: Box<dyn EntitySubscriptionRepository + 'a>,
    pub entity_sharing_core: Arc<EntitySharingCore<'a>>,
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
                let result = self
                    .entity_subscription_repository
                    .create_entity_subscription(params)
                    .await;
                return result;
            })?
            .await;
        return result;
    }

    pub async fn get_all_entity_subscriptions_for_entity_sharing(
        &self,
        entity_sharing_id: &String,
    ) -> Result<Vec<EntitySubscription>, Error> {
        return self
            .entity_subscription_repository
            .get_all_entity_subscriptions_for_entity_sharing(entity_sharing_id)
            .await;
    }

    pub async fn notify_all_subscriptions_of_new_entity_list(
        &self,
        entity_sharing_id: &String,
        data: &Value,
    ) -> Result<Vec<()>, Error> {
        let entity_subscriptions = self
            .get_all_entity_subscriptions_for_entity_sharing(entity_sharing_id)
            .await?;
        let result = future::join_all(entity_subscriptions.into_iter().map(async |sub| {
            self.notify_subscription_of_new_entity_list(&sub, data)
                .await;
        })).await;
        return Ok(result);
    }

    pub async fn notify_subscription_of_new_entity_list(
        &self,
        entity_subscription: &EntitySubscription,
        data: &Value,
    ) -> Result<(), Error> {
        println!("Notifying subscription of new entity: {:?}", data);
        if let Some(python_script) = &entity_subscription.python_script {
            run_python_script_output_json(python_script, data)?;
        }
        Ok(())
    }
}
