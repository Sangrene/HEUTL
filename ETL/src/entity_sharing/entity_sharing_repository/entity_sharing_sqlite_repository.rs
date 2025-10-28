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
    pub json_schema: String,
    pub is_array: bool,
    pub python_script: Option<String>,
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
        is_array: entity_sharing_dto.is_array,
        polling_infos: match entity_sharing_dto.polling_infos {
            Some(s) => serde_json::from_str(&s)?,
            None => None,
        },
        json_schema: serde_json::from_str(&entity_sharing_dto.json_schema)?,
        python_script: entity_sharing_dto.python_script,
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
            polling_infos: params.polling_infos.clone(),
            json_schema: params.json_schema.clone(),
            is_array: params.is_array,
            python_script: params.python_script.clone(),
        };

        sqlx::query("INSERT INTO entity_sharings (id, name, created_at, updated_at, polling_infos, json_schema, connected_app_id, is_array, python_script) 
        VALUES ($1, $2, $3, $4, json($5), json($6), $7, $8, $9, $10)").bind(&entity_sharing.id)
        .bind(&entity_sharing.name)
        .bind(&entity_sharing.created_at)
        .bind(&entity_sharing.updated_at)
        .bind(serde_json::to_string(&entity_sharing.polling_infos).unwrap())
        .bind(serde_json::to_string(&entity_sharing.json_schema).unwrap())
        .bind(&entity_sharing.connected_app_id)
        .bind(&entity_sharing.python_script)
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

    async fn get_all_polling_entity_sharings(&self) -> Result<Vec<EntitySharing>, Error> {
        let result: Vec<EntitySharingDTO> =
            sqlx::query_as("SELECT * FROM entity_sharings WHERE polling_infos IS NOT NULL")
                .fetch_all(self.pool)
                .await?;
        let entity_sharings = result
            .into_iter()
            .map(entity_sharing_dto_to_entity_sharing)
            .collect::<Result<Vec<EntitySharing>, Error>>()?;
        return Ok(entity_sharings);
    }

    async fn get_all_entity_sharings(&self) -> Result<Vec<EntitySharing>, Error> {
        let result: Vec<EntitySharingDTO> = sqlx::query_as("SELECT * FROM entity_sharings")
            .fetch_all(self.pool)
            .await?;
        return result
            .into_iter()
            .map(entity_sharing_dto_to_entity_sharing)
            .collect::<Result<Vec<EntitySharing>, Error>>();
    }

    async fn update_entity_sharing(&self, entity_sharing: &EntitySharing) -> Result<u64, Error> {
        let result = sqlx::query("UPDATE entity_sharings SET name = $1, created_at = $2, updated_at = $3, polling_infos = json($4), 
        json_schema = json($5), connected_app_id = $6, python_script = $7 WHERE id = $8")
        .bind(&entity_sharing.name)
        .bind(&entity_sharing.created_at)
        .bind(&entity_sharing.updated_at)
        .bind(serde_json::to_string(&entity_sharing.polling_infos).unwrap())
        .bind(serde_json::to_string(&entity_sharing.json_schema).unwrap())
        .bind(&entity_sharing.connected_app_id)
        .bind(&entity_sharing.python_script)
        .bind(&entity_sharing.id)
        .execute(self.pool).await?;
        return Ok(result.rows_affected());
    }
}
