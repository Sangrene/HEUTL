use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::entity_sharing::entity_sharing_repository::UpdateEntitySharingParams;
use crate::shared::merge_struct::Merge;
use chrono::Utc;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct EntitySharingPollingInfos {
    pub polling_interval: u64,
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
    pub is_array: bool,
    pub python_script: Option<String>,
}

impl Merge<UpdateEntitySharingParams> for EntitySharing {
    fn merge(self, other: UpdateEntitySharingParams) -> Self {
        let mut merged = self.clone();
        if let Some(name) = other.name {
            merged.name = name;
        }
        if let Some(polling_infos) = other.polling_infos {
            merged.polling_infos = Some(polling_infos);
        }
        if let Some(python_script) = other.python_script {
            merged.python_script = Some(python_script);
        }
        if let Some(is_array) = other.is_array {
            merged.is_array = is_array;
        }
        if let Some(json_schema) = other.json_schema {
            merged.json_schema = json_schema;
        }
        merged.updated_at = Utc::now().timestamp();
        return merged;
    }
}
