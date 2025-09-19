use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct EntitySharingPollingInfos {
    polling_interval: u64,
    polling_url: String,
    polling_method: String,
    polling_headers: HashMap<String, String>,
    polling_body: String,
    polling_timeout: u64,
    polling_retries: u64,
    polling_retry_delay: u64,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct EntitySharing {
    pub id: String,
    pub name: String,
    pub connected_app_id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub polling_infos: Option<EntitySharingPollingInfos>,
    pub jdm_transform: Option<Value>,
    pub json_schema: Value,
}
