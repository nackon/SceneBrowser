use crate::commands::folder::AppState;
use crate::services::ThumbnailGenerator;
use std::fs;
use std::path::Path;
use tauri::State;

/// Generate a thumbnail for a video
#[tauri::command]
pub async fn generate_thumbnail(
    video_id: i64,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let db = state.db.lock().await;

    // Get video info
    let video = db
        .get_video_by_id(video_id)
        .await
        .map_err(|e| format!("Failed to get video info: {}", e))?;

    drop(db); // Release lock before long operation

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
    tokio::spawn(async move {
        let db = state_clone.db.lock().await;
        let _ = db
            .update_video_thumbnail(video_id, &thumbnail_path_clone)
            .await;
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
