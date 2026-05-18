use crate::error::{AppError, Result};
use crate::models::{Folder, Video, VideoInsert, VideoPlayerSetting};
use crate::utils::paths::{get_database_path, get_database_path_for_folder, get_scenebrowser_dir};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Global database service for managing folders
pub struct GlobalDatabase {
    pool: SqlitePool,
}

impl GlobalDatabase {
    /// Create a new global database connection
    pub async fn new() -> Result<Self> {
        let db_path = get_database_path();

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Create connection options
        let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path.display()))?
            .create_if_missing(true);

        // Create connection pool
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        // Run migrations
        Self::run_migrations(&pool).await?;

        Ok(Self { pool })
    }

    /// Run database migrations
    async fn run_migrations(pool: &SqlitePool) -> Result<()> {
        // Read migration files
        let migration_001 = include_str!("../../migrations/001_initial_schema.sql");
        let migration_002 = include_str!("../../migrations/002_video_player_settings.sql");

        // Execute migrations in order
        sqlx::query(migration_001).execute(pool).await?;
        sqlx::query(migration_002).execute(pool).await?;

        Ok(())
    }

    // --- Folder Operations ---

    /// Add a new folder
    pub async fn add_folder(&self, path: &str, recursive: bool) -> Result<i64> {
        // Check if folder already exists
        let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM folders WHERE path = ?")
            .bind(path)
            .fetch_optional(&self.pool)
            .await?;

        if existing.is_some() {
            return Err(AppError::FolderAlreadyExists(path.to_string()));
        }

        // Insert new folder
        let result = sqlx::query("INSERT INTO folders (path, recursive) VALUES (?, ?)")
            .bind(path)
            .bind(recursive)
            .execute(&self.pool)
            .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get all folders
    pub async fn get_folders(&self) -> Result<Vec<Folder>> {
        let folders = sqlx::query_as::<_, Folder>(
            "SELECT id, path, recursive, created_at, updated_at FROM folders ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(folders)
    }

    /// Get folder by ID
    pub async fn get_folder_by_id(&self, folder_id: i64) -> Result<Folder> {
        let folder = sqlx::query_as::<_, Folder>(
            "SELECT id, path, recursive, created_at, updated_at FROM folders WHERE id = ?",
        )
        .bind(folder_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::FolderNotFound(folder_id))?;

        Ok(folder)
    }

    /// Remove a folder
    pub async fn remove_folder(&self, folder_id: i64) -> Result<()> {
        let result = sqlx::query("DELETE FROM folders WHERE id = ?")
            .bind(folder_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::FolderNotFound(folder_id));
        }

        Ok(())
    }
}

/// Per-folder database service for managing videos
pub struct FolderDatabase {
    pool: SqlitePool,
    #[allow(dead_code)]
    folder_path: PathBuf,
}

impl FolderDatabase {
    /// Create a new folder database connection
    pub async fn new(folder_path: &Path) -> Result<Self> {
        let scenebrowser_dir = get_scenebrowser_dir(folder_path);
        std::fs::create_dir_all(&scenebrowser_dir)?;

        let db_path = get_database_path_for_folder(folder_path);

        // Create connection options
        let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path.display()))?
            .create_if_missing(true);

        // Create connection pool
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        // Run migrations
        Self::run_migrations(&pool, folder_path).await?;

        Ok(Self {
            pool,
            folder_path: folder_path.to_path_buf(),
        })
    }

    /// Run database migrations
    async fn run_migrations(pool: &SqlitePool, folder_path: &Path) -> Result<()> {
        // Read per-folder migration file
        let migration_sql = include_str!("../../migrations/per_folder_schema.sql");

        // Execute migration
        sqlx::query(migration_sql).execute(pool).await?;

        // Update existing videos with old default (16) to new default (9)
        // Clear thumbnail_path to force regeneration with new grid size
        let updated_rows = sqlx::query(
            "UPDATE videos SET thumbnail_count = 9, thumbnail_path = NULL WHERE thumbnail_count = 16"
        )
        .execute(pool)
        .await?
        .rows_affected();

        // If thumbnails were cleared, delete the thumbnail files
        if updated_rows > 0 {
            let thumbnail_dir =
                crate::utils::paths::get_thumbnail_cache_dir_for_folder(folder_path);
            if thumbnail_dir.exists() {
                // Delete all thumbnail files (they'll be regenerated on next scan)
                if let Err(e) = std::fs::remove_dir_all(&thumbnail_dir) {
                    eprintln!("Warning: Failed to delete old thumbnails: {}", e);
                }
                // Recreate the directory
                let _ = std::fs::create_dir_all(&thumbnail_dir);
            }
        }

        Ok(())
    }

    // --- Video Operations ---

    /// Add a new video
    pub async fn add_video(&self, video: &VideoInsert) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO videos (
                path, filename, hash, duration, width, height, size, codec, framerate
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&video.path)
        .bind(&video.filename)
        .bind(&video.hash)
        .bind(video.duration)
        .bind(video.width)
        .bind(video.height)
        .bind(video.size)
        .bind(&video.codec)
        .bind(video.framerate)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get videos with pagination
    pub async fn get_videos(&self, limit: i64, offset: i64) -> Result<Vec<Video>> {
        let videos = sqlx::query_as::<_, Video>(
            r#"
            SELECT id, 0 as folder_id, path, filename, hash, duration, width, height, size,
                   codec, framerate, thumbnail_path, thumbnail_count, rating,
                   created_at, updated_at, scanned_at
            FROM videos
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(videos)
    }

    /// Search videos by filename or path
    pub async fn search_videos(&self, query: &str) -> Result<Vec<Video>> {
        let search_pattern = format!("%{}%", query);

        let videos = sqlx::query_as::<_, Video>(
            r#"
            SELECT id, 0 as folder_id, path, filename, hash, duration, width, height, size,
                   codec, framerate, thumbnail_path, thumbnail_count, rating,
                   created_at, updated_at, scanned_at
            FROM videos
            WHERE filename LIKE ? COLLATE NOCASE OR path LIKE ? COLLATE NOCASE
            ORDER BY created_at DESC
            LIMIT 1000
            "#,
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;

        Ok(videos)
    }

    /// Get video by ID
    pub async fn get_video_by_id(&self, video_id: i64) -> Result<Video> {
        let video = sqlx::query_as::<_, Video>(
            r#"
            SELECT id, 0 as folder_id, path, filename, hash, duration, width, height, size,
                   codec, framerate, thumbnail_path, thumbnail_count, rating,
                   created_at, updated_at, scanned_at
            FROM videos
            WHERE id = ?
            "#,
        )
        .bind(video_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::VideoNotFound(video_id))?;

        Ok(video)
    }

    /// Update video thumbnail path
    pub async fn update_video_thumbnail(&self, video_id: i64, thumbnail_path: &str) -> Result<()> {
        let result = sqlx::query(
            "UPDATE videos SET thumbnail_path = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(thumbnail_path)
        .bind(video_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::VideoNotFound(video_id));
        }

        Ok(())
    }

    /// Check if video exists by path
    pub async fn video_exists(&self, path: &str) -> Result<Option<i64>> {
        let result: Option<(i64,)> = sqlx::query_as("SELECT id FROM videos WHERE path = ?")
            .bind(path)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.map(|(id,)| id))
    }

    /// Update existing video metadata
    pub async fn update_video(&self, video_id: i64, video: &VideoInsert) -> Result<()> {
        let result = sqlx::query(
            r#"
            UPDATE videos
            SET filename = ?, hash = ?, duration = ?, width = ?, height = ?,
                size = ?, codec = ?, framerate = ?, updated_at = CURRENT_TIMESTAMP,
                scanned_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(&video.filename)
        .bind(&video.hash)
        .bind(video.duration)
        .bind(video.width)
        .bind(video.height)
        .bind(video.size)
        .bind(&video.codec)
        .bind(video.framerate)
        .bind(video_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::VideoNotFound(video_id));
        }

        Ok(())
    }

    /// Get all videos (no pagination)
    pub async fn get_all_videos(&self) -> Result<Vec<Video>> {
        let videos = sqlx::query_as::<_, Video>(
            r#"
            SELECT id, 0 as folder_id, path, filename, hash, duration, width, height, size,
                   codec, framerate, thumbnail_path, thumbnail_count, rating,
                   created_at, updated_at, scanned_at
            FROM videos
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(videos)
    }
}

/// Database manager that coordinates global and per-folder databases
pub struct DatabaseManager {
    global_db: Arc<GlobalDatabase>,
    folder_dbs: Arc<RwLock<HashMap<String, Arc<FolderDatabase>>>>,
}

impl DatabaseManager {
    /// Create a new database manager
    pub async fn new() -> Result<Self> {
        let global_db = GlobalDatabase::new().await?;

        Ok(Self {
            global_db: Arc::new(global_db),
            folder_dbs: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Get the global database
    pub fn global_db(&self) -> Arc<GlobalDatabase> {
        Arc::clone(&self.global_db)
    }

    /// Get or create a folder database
    pub async fn get_folder_db(&self, folder_path: &Path) -> Result<Arc<FolderDatabase>> {
        let folder_path_str = folder_path.to_string_lossy().to_string();

        // Check if already cached
        {
            let dbs = self.folder_dbs.read().await;
            if let Some(db) = dbs.get(&folder_path_str) {
                return Ok(Arc::clone(db));
            }
        }

        // Create new folder database
        let folder_db = FolderDatabase::new(folder_path).await?;
        let folder_db = Arc::new(folder_db);

        // Cache it
        {
            let mut dbs = self.folder_dbs.write().await;
            dbs.insert(folder_path_str, Arc::clone(&folder_db));
        }

        Ok(folder_db)
    }

    /// Get all videos from all folders
    pub async fn get_all_videos(&self) -> Result<Vec<Video>> {
        let folders = self.global_db.get_folders().await?;
        let mut all_videos = Vec::new();

        for folder in folders {
            let folder_path = PathBuf::from(&folder.path);
            let folder_db = self.get_folder_db(&folder_path).await?;
            let videos = folder_db.get_all_videos().await?;
            all_videos.extend(videos);
        }

        // Sort by created_at descending
        all_videos.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(all_videos)
    }

    /// Search videos across all folders
    pub async fn search_all_videos(&self, query: &str) -> Result<Vec<Video>> {
        let folders = self.global_db.get_folders().await?;
        let mut all_videos = Vec::new();

        for folder in folders {
            let folder_path = PathBuf::from(&folder.path);
            let folder_db = self.get_folder_db(&folder_path).await?;
            let videos = folder_db.search_videos(query).await?;
            all_videos.extend(videos);
        }

        // Sort by created_at descending
        all_videos.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(all_videos)
    }

    // --- Video Player Settings Operations ---

    /// Get all video player settings
    pub async fn get_video_player_settings(&self) -> Result<Vec<VideoPlayerSetting>> {
        let settings = sqlx::query_as::<_, VideoPlayerSetting>(
            "SELECT id, file_extension, player_path, created_at, updated_at
             FROM video_player_settings
             ORDER BY CASE WHEN file_extension = '*' THEN 1 ELSE 0 END, file_extension",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(settings)
    }

    /// Get video player setting for a specific file extension
    pub async fn get_player_for_extension(&self, extension: &str) -> Result<Option<String>> {
        // Try to find setting for specific extension
        let specific: Option<(String,)> = sqlx::query_as(
            "SELECT player_path FROM video_player_settings WHERE file_extension = ?",
        )
        .bind(extension.to_lowercase())
        .fetch_optional(&self.pool)
        .await?;

        if let Some((player_path,)) = specific {
            return Ok(Some(player_path));
        }

        // Fall back to default setting (*)
        let default: Option<(String,)> = sqlx::query_as(
            "SELECT player_path FROM video_player_settings WHERE file_extension = '*'",
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(default.map(|(player_path,)| player_path))
    }

    /// Set video player for a file extension
    pub async fn set_video_player(&self, file_extension: &str, player_path: &str) -> Result<()> {
        sqlx::query(
            "INSERT INTO video_player_settings (file_extension, player_path, updated_at)
             VALUES (?, ?, CURRENT_TIMESTAMP)
             ON CONFLICT(file_extension)
             DO UPDATE SET player_path = excluded.player_path, updated_at = CURRENT_TIMESTAMP",
        )
        .bind(file_extension.to_lowercase())
        .bind(player_path)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Delete video player setting for a file extension
    pub async fn delete_video_player_setting(&self, file_extension: &str) -> Result<()> {
        // Don't allow deletion of default setting
        if file_extension == "*" {
            return Err(AppError::Other(
                "Cannot delete default player setting".to_string(),
            ));
        }

        sqlx::query("DELETE FROM video_player_settings WHERE file_extension = ?")
            .bind(file_extension.to_lowercase())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
