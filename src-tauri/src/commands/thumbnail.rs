use crate::commands::folder::AppState;
use crate::services::ThumbnailGenerator;
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
        .map_err(|e| e.to_string())?;

    drop(db); // Release lock before long operation

    // Generate thumbnail
    let thumbnail_path = ThumbnailGenerator::generate(
        Path::new(&video.path),
        video.duration as f64,
        video.thumbnail_count as usize,
    )
    .await
    .map_err(|e| e.to_string())?;

    // Update database with thumbnail path
    let db = state.db.lock().await;
    db.update_video_thumbnail(video_id, thumbnail_path.to_str().unwrap())
        .await
        .map_err(|e| e.to_string())?;

    Ok(thumbnail_path.to_string_lossy().to_string())
}
