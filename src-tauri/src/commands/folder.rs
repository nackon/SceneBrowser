use crate::models::Folder;
use crate::services::DatabaseManager;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub db_manager: Arc<Mutex<DatabaseManager>>,
}

/// Add a new folder to monitor
#[tauri::command]
pub async fn add_folder(
    path: String,
    recursive: bool,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let db_manager = state.db_manager.lock().await;
    let global_db = db_manager.global_db();

    // Add folder to global database
    let folder_id = global_db
        .add_folder(&path, recursive)
        .await
        .map_err(|e| e.to_string())?;

    // Initialize folder database (create .scenebrowser directory and db.sqlite)
    let folder_path = PathBuf::from(&path);
    db_manager
        .get_folder_db(&folder_path)
        .await
        .map_err(|e| e.to_string())?;

    Ok(folder_id)
}

/// Get all registered folders
#[tauri::command]
pub async fn get_folders(state: State<'_, AppState>) -> Result<Vec<Folder>, String> {
    let db_manager = state.db_manager.lock().await;
    let global_db = db_manager.global_db();
    global_db.get_folders().await.map_err(|e| e.to_string())
}

/// Remove a folder and all its videos
#[tauri::command]
pub async fn remove_folder(folder_id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db_manager = state.db_manager.lock().await;
    let global_db = db_manager.global_db();

    // Get folder path before deletion
    let _folder = global_db
        .get_folder_by_id(folder_id)
        .await
        .map_err(|e| e.to_string())?;

    // Remove from global database
    global_db
        .remove_folder(folder_id)
        .await
        .map_err(|e| e.to_string())?;

    // Optionally delete .scenebrowser directory
    // (Commented out for safety - user might want to keep the data)
    // let scenebrowser_dir = crate::utils::paths::get_scenebrowser_dir(&PathBuf::from(&folder.path));
    // if scenebrowser_dir.exists() {
    //     std::fs::remove_dir_all(scenebrowser_dir).map_err(|e| e.to_string())?;
    // }

    Ok(())
}
