use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoPlayerSetting {
    pub id: i64,
    pub file_extension: String, // "*" for default, or specific extension like "mp4"
    pub player_path: String,    // Empty string means system default
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoPlayerSettingInput {
    pub file_extension: String,
    pub player_path: String,
}
