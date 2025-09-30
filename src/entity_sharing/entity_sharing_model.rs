use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct EntitySharingPollingInfos {
    pub polling_interval: u64,
    pub polling_url: String,
    pub polling_method: String,
    pub polling_headers: HashMap<String, String>,
    pub polling_body: String,
    pub polling_timeout: u64,
    pub polling_retries: u64,
    pub polling_retry_delay: u64,
}

impl From<String> for EntitySharingPollingInfos {
    fn from(s: String) -> Self {
        serde_json::from_str(&s).unwrap()
    }
}

impl ToString for EntitySharingPollingInfos {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntitySharing {
    pub id: String,
    pub name: String,
    pub connected_app_id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub polling_infos: Option<EntitySharingPollingInfos>,
    pub json_schema: Value,
}
