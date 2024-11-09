use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Todos {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub status: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
