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
        };

        sqlx::query("INSERT INTO entity_subscriptions (id, entity_sharing_id, created_at, updated_at, connected_app_id) VALUES ($1, $2, $3, $4, $5)")
        .bind(&entity_subscription.id)
        .bind(&entity_subscription.entity_sharing_id)
        .bind(&entity_subscription.created_at)
        .bind(&entity_subscription.updated_at)
        .bind(&entity_subscription.connected_app_id)
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
}
