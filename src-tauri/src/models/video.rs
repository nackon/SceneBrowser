use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Video model representing a video file with metadata
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Video {
    pub id: i64,
    pub folder_id: i64,
    pub path: String,
    pub filename: String,
    pub hash: String,
    pub duration: i64,
    pub width: i64,
    pub height: i64,
    pub size: i64,
    pub codec: Option<String>,
    pub framerate: Option<f64>,
    pub thumbnail_path: Option<String>,
    pub thumbnail_count: i64,
    pub rating: i64,
    pub is_favorite: i64,
    pub created_at: String,
    pub updated_at: String,
    pub scanned_at: String,
}

/// Input struct for inserting a new video
#[derive(Debug, Clone)]
pub struct VideoInsert {
    pub path: String,
    pub filename: String,
    pub hash: String,
    pub duration: i64,
    pub width: i64,
    pub height: i64,
    pub size: i64,
    pub codec: Option<String>,
    pub framerate: Option<f64>,
}
