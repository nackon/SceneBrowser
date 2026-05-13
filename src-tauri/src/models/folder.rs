use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Folder model representing a registered video folder
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Folder {
    pub id: i64,
    pub path: String,
    pub recursive: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Input struct for inserting a new folder
#[derive(Debug, Clone)]
pub struct FolderInsert {
    pub path: String,
    pub recursive: bool,
}
