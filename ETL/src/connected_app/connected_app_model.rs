use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct ConnectedApp {
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
}