use thiserror::Error;

/// Application error types
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("FFmpeg not found")]
    FFmpegNotFound,

    #[error("FFmpeg execution failed: {0}")]
    FFmpegExecution(String),

    #[error("Invalid video file: {0}")]
    InvalidVideo(String),

    #[error("Video not found: {0}")]
    VideoNotFound(i64),

    #[error("Folder not found: {0}")]
    FolderNotFound(i64),

    #[error("Folder already exists: {0}")]
    FolderAlreadyExists(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

/// Result type alias for AppError
pub type Result<T> = std::result::Result<T, AppError>;

/// Convert AppError to String for Tauri commands
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}
