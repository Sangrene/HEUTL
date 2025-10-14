use crate::entity_subscription::entity_subscription_model::EntitySubscription;
use crate::entity_subscription::entity_subscription_repository::{
    CreateEntitySubscriptionParams, EntitySubscriptionRepository,
};
use crate::shared::errors::Error;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::sqlite::SqlitePool;

pub struct EntitySubscriptionSQLiteRepository<'a> {
    pub pool: &'a SqlitePool,
}

#[async_trait]
impl<'a> EntitySubscriptionRepository for EntitySubscriptionSQLiteRepository<'a> {
    async fn create_entity_subscription(
        &self,
        params: &CreateEntitySubscriptionParams,
    ) -> Result<EntitySubscription, Error> {
        let entity_subscription = EntitySubscription {
            id: params.id.clone(),
            entity_sharing_id: params.entity_sharing_id.clone(),
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
            connected_app_id: params.connected_app_id.clone(),
            jdm_transform: params.jdm_transform.clone(),
            python_script: params.python_script.clone(),
        };

        sqlx::query("INSERT INTO entity_subscriptions (id, entity_sharing_id, created_at, updated_at, connected_app_id, jdm_transform, python_script) VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(&entity_subscription.id)
        .bind(&entity_subscription.entity_sharing_id)
        .bind(&entity_subscription.created_at)
        .bind(&entity_subscription.updated_at)
        .bind(&entity_subscription.connected_app_id)
        .bind(serde_json::to_string(&entity_subscription.jdm_transform).unwrap_or_else(|_| "".to_string()))
        .bind(&entity_subscription.python_script)
        .execute(self.pool)
        .await?;

        return Ok(entity_subscription);
    }

    async fn get_entity_subscription_by_id(
        &self,
        id: &String,
    ) -> Result<EntitySubscription, Error> {
        let result: EntitySubscription =
            sqlx::query_as("SELECT * FROM entity_subscriptions WHERE id = :id LIMIT 1")
                .bind(id)
                .fetch_one(self.pool)
                .await?;
        return Ok(result);
    }

    async fn get_all_entity_subscriptions_for_entity_sharing(&self, entity_sharing_id: &String) -> Result<Vec<EntitySubscription>, Error> {
        let result: Vec<EntitySubscription> =
            sqlx::query_as("SELECT * FROM entity_subscriptions WHERE entity_sharing_id = $1")
                .bind(entity_sharing_id)
                .fetch_all(self.pool)
                .await?;
        return Ok(result);
    }
}
