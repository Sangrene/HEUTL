use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct EntitySubscription {
    pub id: String,
    pub entity_sharing_id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub connected_app_id: String,
    pub jdm_transform: Option<Value>,
}
