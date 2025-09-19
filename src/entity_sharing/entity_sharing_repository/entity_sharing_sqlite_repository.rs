use crate::entity_sharing::entity_sharing_model::EntitySharing;
use crate::entity_sharing::entity_sharing_repository::{
    CreateEntitySharingParams, EntitySharingRepository,
};
use crate::shared::errors::Error;
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct EntitySharingDTO {
    pub id: String,
    pub name: String,
    pub connected_app_id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub polling_infos: Option<String>,
    pub jdm_transform: Option<String>,
    pub json_schema: String,
}

fn entity_sharing_dto_to_entity_sharing(
    entity_sharing_dto: EntitySharingDTO,
) -> Result<EntitySharing, Error> {
    let entity_sharing = EntitySharing {
        id: entity_sharing_dto.id.clone(),
        name: entity_sharing_dto.name.clone(),
        connected_app_id: entity_sharing_dto.connected_app_id.clone(),
        created_at: entity_sharing_dto.created_at,
        updated_at: entity_sharing_dto.updated_at,
        jdm_transform: match entity_sharing_dto.jdm_transform {
            Some(s) => serde_json::from_str(&s)?,
            None => None,
        },
        polling_infos: match entity_sharing_dto.polling_infos {
            Some(s) => serde_json::from_str(&s)?,
            None => None,
        },
        json_schema: serde_json::from_str(&entity_sharing_dto.json_schema)?,
    };
    return Ok(entity_sharing);
}

pub struct EntitySharingSQLiteRepository<'a> {
    pub pool: &'a SqlitePool,
}

#[async_trait]
impl<'a> EntitySharingRepository for EntitySharingSQLiteRepository<'a> {
    async fn create_entity_sharing(
        &self,
        params: &CreateEntitySharingParams,
    ) -> Result<EntitySharing, Error> {
        let entity_sharing = EntitySharing {
            id: params.id.clone(),
            name: params.name.clone(),
            connected_app_id: params.connected_app_id.clone(),
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
            jdm_transform: params.jdm_transform.clone(),
            polling_infos: params.polling_infos.clone(),
            json_schema: params.json_schema.clone(),
        };

        sqlx::query("INSERT INTO entity_sharings (id, name, created_at, updated_at, jdm_transform, polling_infos, json_schema, connected_app_id) 
        VALUES ($1, $2, $3, $4, json($5), json($6), json($7), $8)").bind(&entity_sharing.id)
        .bind(&entity_sharing.name)
        .bind(&entity_sharing.created_at)
        .bind(&entity_sharing.updated_at)
        .bind(serde_json::to_string(&entity_sharing.jdm_transform).unwrap())
        .bind(serde_json::to_string(&entity_sharing.polling_infos).unwrap())
        .bind(serde_json::to_string(&entity_sharing.json_schema).unwrap())
        .bind(&entity_sharing.connected_app_id)
        .execute(self.pool).await?;
        Ok(entity_sharing)
    }

    async fn get_entity_sharing(&self, id: &String) -> Result<EntitySharing, Error> {
        let result: EntitySharingDTO =
            sqlx::query_as("SELECT * FROM entity_sharings WHERE id = $1 LIMIT 1")
                .bind(id)
                .fetch_one(self.pool)
                .await?;
        return entity_sharing_dto_to_entity_sharing(result);
    }
}
