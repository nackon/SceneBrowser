use crate::commands::folder::AppState;
use crate::models::{Video, VideoInsert};
use crate::services::{MetadataExtractor, VideoScanner};
use crate::utils::compute_file_hash;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{Emitter, State};

/// Result of a folder scan operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub videos_found: usize,
    pub videos_added: usize,
    pub videos_updated: usize,
    pub errors: Vec<String>,
}

/// Progress information during scanning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub current: usize,
    pub total: usize,
    pub current_file: String,
}

/// Scan a folder for videos and add them to the database
#[tauri::command]
pub async fn scan_folder(
    folder_id: i64,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<ScanResult, String> {
    let db_manager = state.db_manager.lock().await;

    // Get folder info from global database
    let global_db = db_manager.global_db();
    let folder = global_db
        .get_folder_by_id(folder_id)
        .await
        .map_err(|e| e.to_string())?;

    let folder_path = PathBuf::from(&folder.path);

    // Get folder database
    let folder_db = db_manager
        .get_folder_db(&folder_path)
        .await
        .map_err(|e| e.to_string())?;

    drop(db_manager); // Release lock before long-running operations

    // Scan directory for video files
    let video_paths =
        VideoScanner::scan_directory(&folder_path, folder.recursive).map_err(|e| e.to_string())?;

    let total = video_paths.len();
    let mut added = 0;
    let mut updated = 0;
    let mut errors = Vec::new();

    // Process each video
    for (idx, video_path) in video_paths.iter().enumerate() {
        // Emit progress
        let _ = window.emit(
            "scan_progress",
            ScanProgress {
                current: idx + 1,
                total,
                current_file: video_path.to_string_lossy().to_string(),
            },
        );

        // Extract metadata
        let metadata = match MetadataExtractor::extract(video_path).await {
            Ok(m) => m,
            Err(e) => {
                errors.push(format!("{}: {}", video_path.display(), e));
                continue;
            }
        };

        // Compute file hash
        let hash = match compute_file_hash(video_path) {
            Ok(h) => h,
            Err(e) => {
                errors.push(format!("{}: Failed to hash - {}", video_path.display(), e));
                continue;
            }
        };

        // Get file size
        let size = match std::fs::metadata(video_path) {
            Ok(m) => m.len() as i64,
            Err(e) => {
                errors.push(format!(
                    "{}: Failed to get size - {}",
                    video_path.display(),
                    e
                ));
                continue;
            }
        };

        let filename = video_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let video_insert = VideoInsert {
            path: video_path.to_string_lossy().to_string(),
            filename,
            hash,
            duration: metadata.duration as i64,
            width: metadata.width as i64,
            height: metadata.height as i64,
            size,
            codec: Some(metadata.codec),
            framerate: Some(metadata.framerate),
        };

        // Check if video already exists
        match folder_db.video_exists(&video_insert.path).await {
            Ok(Some(video_id)) => {
                // Update existing video
                if let Err(e) = folder_db.update_video(video_id, &video_insert).await {
                    errors.push(format!(
                        "{}: Failed to update - {}",
                        video_path.display(),
                        e
                    ));
                } else {
                    updated += 1;
                }
            }
            Ok(None) => {
                // Add new video
                if let Err(e) = folder_db.add_video(&video_insert).await {
                    errors.push(format!("{}: Failed to add - {}", video_path.display(), e));
                } else {
                    added += 1;
                }
            }
            Err(e) => {
                errors.push(format!("{}: Database error - {}", video_path.display(), e));
            }
        }
    }

    Ok(ScanResult {
        videos_found: total,
        videos_added: added,
        videos_updated: updated,
        errors,
    })
}

/// Get videos with pagination
#[tauri::command]
pub async fn get_videos(
    folder_id: Option<i64>,
    limit: i64,
    offset: i64,
    state: State<'_, AppState>,
) -> Result<Vec<Video>, String> {
    let db_manager = state.db_manager.lock().await;

    if let Some(fid) = folder_id {
        // Get videos from specific folder
        let global_db = db_manager.global_db();
        let folder = global_db
            .get_folder_by_id(fid)
            .await
            .map_err(|e| e.to_string())?;

        let folder_path = PathBuf::from(&folder.path);
        let folder_db = db_manager
            .get_folder_db(&folder_path)
            .await
            .map_err(|e| e.to_string())?;

        folder_db
            .get_videos(limit, offset)
            .await
            .map_err(|e| e.to_string())
    } else {
        // Get videos from all folders
        let all_videos = db_manager
            .get_all_videos()
            .await
            .map_err(|e| e.to_string())?;

        // Apply pagination
        let start = offset as usize;
        let end = (start + limit as usize).min(all_videos.len());
        Ok(all_videos[start..end].to_vec())
    }
}

/// Search videos by filename or path
#[tauri::command]
pub async fn search_videos(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<Video>, String> {
    let db_manager = state.db_manager.lock().await;
    db_manager
        .search_all_videos(&query)
        .await
        .map_err(|e| e.to_string())
}

/// Get a specific video by ID
/// Note: This searches across all folders since we don't know which folder contains the video
#[tauri::command]
pub async fn get_video_by_id(video_id: i64, state: State<'_, AppState>) -> Result<Video, String> {
    let db_manager = state.db_manager.lock().await;
    let global_db = db_manager.global_db();
    let folders = global_db.get_folders().await.map_err(|e| e.to_string())?;

    // Search in each folder database
    for folder in folders {
        let folder_path = PathBuf::from(&folder.path);
        let folder_db = db_manager
            .get_folder_db(&folder_path)
            .await
            .map_err(|e| e.to_string())?;

        if let Ok(video) = folder_db.get_video_by_id(video_id).await {
            return Ok(video);
        }
    }

    Err(format!("Video with ID {} not found", video_id))
}
