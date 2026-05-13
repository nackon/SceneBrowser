use crate::error::{AppError, Result};
use crate::models::{Folder, Video, VideoInsert};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::path::PathBuf;
use std::str::FromStr;

/// Database service for managing SQLite operations
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create a new database connection and run migrations
    pub async fn new(db_path: PathBuf) -> Result<Self> {
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
        // Read migration file
        let migration_sql = include_str!("../../migrations/001_initial_schema.sql");

        // Execute migration
        sqlx::query(migration_sql).execute(pool).await?;

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
            "SELECT id, path, recursive, created_at, updated_at FROM folders WHERE id = ?"
        )
        .bind(folder_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::FolderNotFound(folder_id))?;

        Ok(folder)
    }

    /// Remove a folder (cascades to videos)
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

    // --- Video Operations ---

    /// Add a new video
    pub async fn add_video(&self, video: &VideoInsert) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO videos (
                folder_id, path, filename, hash, duration, width, height, size, codec, framerate
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(video.folder_id)
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
    pub async fn get_videos(
        &self,
        folder_id: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Video>> {
        let videos = if let Some(fid) = folder_id {
            sqlx::query_as::<_, Video>(
                r#"
                SELECT id, folder_id, path, filename, hash, duration, width, height, size,
                       codec, framerate, thumbnail_path, thumbnail_count, rating,
                       created_at, updated_at, scanned_at
                FROM videos
                WHERE folder_id = ?
                ORDER BY created_at DESC
                LIMIT ? OFFSET ?
                "#
            )
            .bind(fid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, Video>(
                r#"
                SELECT id, folder_id, path, filename, hash, duration, width, height, size,
                       codec, framerate, thumbnail_path, thumbnail_count, rating,
                       created_at, updated_at, scanned_at
                FROM videos
                ORDER BY created_at DESC
                LIMIT ? OFFSET ?
                "#
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?
        };

        Ok(videos)
    }

    /// Search videos by filename or path
    pub async fn search_videos(&self, query: &str) -> Result<Vec<Video>> {
        let search_pattern = format!("%{}%", query);

        let videos = sqlx::query_as::<_, Video>(
            r#"
            SELECT id, folder_id, path, filename, hash, duration, width, height, size,
                   codec, framerate, thumbnail_path, thumbnail_count, rating,
                   created_at, updated_at, scanned_at
            FROM videos
            WHERE filename LIKE ? COLLATE NOCASE OR path LIKE ? COLLATE NOCASE
            ORDER BY created_at DESC
            LIMIT 1000
            "#
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
            SELECT id, folder_id, path, filename, hash, duration, width, height, size,
                   codec, framerate, thumbnail_path, thumbnail_count, rating,
                   created_at, updated_at, scanned_at
            FROM videos
            WHERE id = ?
            "#
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
            "UPDATE videos SET thumbnail_path = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
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
        let result: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM videos WHERE path = ?"
        )
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
            "#
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
}
