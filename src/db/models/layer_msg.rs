use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(sqlx::FromRow, Clone, Debug, Serialize)]
pub struct LayerMsg {
    pub height: i64,
    pub status: i32,
    pub layer1_hash: Option<String>,
    pub layer2_hash: Option<String>,
    pub created_time: NaiveDateTime,
    pub event: String,
}
