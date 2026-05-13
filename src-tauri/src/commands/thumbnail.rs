use crate::commands::folder::AppState;
use crate::services::ThumbnailGenerator;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{Emitter, State};

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

/// Generate thumbnails for multiple videos in parallel
#[tauri::command]
pub async fn generate_thumbnails_batch(
    folder_id: i64,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    use futures::stream::{self, StreamExt};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize)]
    struct ThumbnailProgress {
        current: usize,
        total: usize,
        current_file: String,
    }

    let db_manager = state.db_manager.lock().await;

    // Get folder info
    let global_db = db_manager.global_db();
    let folder = global_db
        .get_folder_by_id(folder_id)
        .await
        .map_err(|e| e.to_string())?;

    let folder_path = PathBuf::from(&folder.path);
    let folder_db = db_manager
        .get_folder_db(&folder_path)
        .await
        .map_err(|e| e.to_string())?;

    // Get all videos without thumbnails
    let videos = folder_db
        .get_all_videos()
        .await
        .map_err(|e| e.to_string())?;

    let videos_to_process: Vec<_> = videos
        .into_iter()
        .filter(|v| {
            v.thumbnail_path.is_none()
                || v.thumbnail_path
                    .as_ref()
                    .map(|p| p.is_empty())
                    .unwrap_or(true)
        })
        .collect();

    let total = videos_to_process.len();

    if total == 0 {
        return Ok(0);
    }

    drop(db_manager);

    // Process videos in parallel (max 4 concurrent)
    let generated = std::sync::atomic::AtomicUsize::new(0);

    stream::iter(videos_to_process)
        .enumerate()
        .map(|(idx, video)| {
            let window = window.clone();
            let state = state.inner().clone();
            let folder_path = folder_path.clone();
            let generated = &generated;

            async move {
                // Emit progress
                let _ = window.emit(
                    "thumbnail_progress",
                    ThumbnailProgress {
                        current: idx + 1,
                        total,
                        current_file: video.filename.clone(),
                    },
                );

                // Generate thumbnail
                let result = ThumbnailGenerator::generate(
                    Path::new(&video.path),
                    video.duration as f64,
                    video.thumbnail_count as usize,
                )
                .await;

                if let Ok(thumbnail_path) = result {
                    // Update database
                    let db_manager = state.db_manager.lock().await;
                    if let Ok(folder_db) = db_manager.get_folder_db(&folder_path).await {
                        let _ = folder_db
                            .update_video_thumbnail(video.id, &thumbnail_path.to_string_lossy())
                            .await;
                    }
                    generated.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            }
        })
        .buffer_unordered(4) // Process 4 videos concurrently
        .collect::<Vec<_>>()
        .await;

    Ok(generated.load(std::sync::atomic::Ordering::Relaxed))
}
