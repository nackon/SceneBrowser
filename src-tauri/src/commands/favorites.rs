use crate::commands::folder::AppState;
use crate::models::Video;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn toggle_favorite(
    folder_id: i64,
    video_id: i64,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let db_manager = state.db_manager.lock().await;

    // Get folder path
    let folder = db_manager
        .global_db()
        .get_folder_by_id(folder_id)
        .await
        .map_err(|e| e.to_string())?;

    let folder_path = PathBuf::from(&folder.path);

    // Get folder database
    let folder_db = db_manager
        .get_folder_db(&folder_path)
        .await
        .map_err(|e| e.to_string())?;

    // Toggle favorite
    folder_db
        .toggle_favorite(video_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_favorite_videos(
    folder_id: i64,
    limit: i64,
    offset: i64,
    state: State<'_, AppState>,
) -> Result<Vec<Video>, String> {
    let db_manager = state.db_manager.lock().await;

    // Get folder path
    let folder = db_manager
        .global_db()
        .get_folder_by_id(folder_id)
        .await
        .map_err(|e| e.to_string())?;

    let folder_path = PathBuf::from(&folder.path);

    // Get folder database
    let folder_db = db_manager
        .get_folder_db(&folder_path)
        .await
        .map_err(|e| e.to_string())?;

    // Get favorite videos
    folder_db
        .get_favorite_videos(limit, offset)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_favorite_count(folder_id: i64, state: State<'_, AppState>) -> Result<i64, String> {
    let db_manager = state.db_manager.lock().await;

    // Get folder path
    let folder = db_manager
        .global_db()
        .get_folder_by_id(folder_id)
        .await
        .map_err(|e| e.to_string())?;

    let folder_path = PathBuf::from(&folder.path);

    // Get folder database
    let folder_db = db_manager
        .get_folder_db(&folder_path)
        .await
        .map_err(|e| e.to_string())?;

    // Get favorite count
    folder_db
        .get_favorite_count()
        .await
        .map_err(|e| e.to_string())
}
