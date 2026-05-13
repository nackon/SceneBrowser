use crate::commands::folder::AppState;
use crate::services::ThumbnailGenerator;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::State;

/// Generate a thumbnail for a video
#[tauri::command]
pub async fn generate_thumbnail(
    video_id: i64,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let db_manager = state.db_manager.lock().await;

    // Search for video in all folders
    let global_db = db_manager.global_db();
    let folders = global_db.get_folders().await.map_err(|e| e.to_string())?;

    let mut video_opt = None;
    let mut folder_path_opt = None;

    for folder in folders {
        let folder_path = PathBuf::from(&folder.path);
        let folder_db = db_manager
            .get_folder_db(&folder_path)
            .await
            .map_err(|e| e.to_string())?;

        if let Ok(video) = folder_db.get_video_by_id(video_id).await {
            video_opt = Some(video);
            folder_path_opt = Some(folder_path);
            break;
        }
    }

    let video = video_opt.ok_or_else(|| format!("Video with ID {} not found", video_id))?;
    let folder_path = folder_path_opt.unwrap();

    drop(db_manager); // Release lock before long operation

    // Generate thumbnail
    let thumbnail_path = ThumbnailGenerator::generate(
        Path::new(&video.path),
        video.duration as f64,
        video.thumbnail_count as usize,
    )
    .await
    .map_err(|e| format!("Failed to generate thumbnail: {}", e))?;

    let thumbnail_path_str = thumbnail_path.to_string_lossy().to_string();

    // Update database with thumbnail path in a separate task to avoid blocking
    let state_clone = state.inner().clone();
    let thumbnail_path_clone = thumbnail_path_str.clone();
    let folder_path_clone = folder_path.clone();
    tokio::spawn(async move {
        let db_manager = state_clone.db_manager.lock().await;
        if let Ok(folder_db) = db_manager.get_folder_db(&folder_path_clone).await {
            let _ = folder_db
                .update_video_thumbnail(video_id, &thumbnail_path_clone)
                .await;
        }
    });

    Ok(thumbnail_path_str)
}

/// Read thumbnail file and return as base64 data URL
#[tauri::command]
pub async fn read_thumbnail(thumbnail_path: String) -> Result<String, String> {
    let bytes =
        fs::read(&thumbnail_path).map_err(|e| format!("Failed to read thumbnail file: {}", e))?;

    let base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
    Ok(format!("data:image/jpeg;base64,{}", base64))
}
