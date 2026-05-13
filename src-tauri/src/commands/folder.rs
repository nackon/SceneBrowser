use crate::models::Folder;
use crate::services::Database;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
}

/// Add a new folder to monitor
#[tauri::command]
pub async fn add_folder(
    path: String,
    recursive: bool,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let db = state.db.lock().await;
    db.add_folder(&path, recursive)
        .await
        .map_err(|e| e.to_string())
}

/// Get all registered folders
#[tauri::command]
pub async fn get_folders(state: State<'_, AppState>) -> Result<Vec<Folder>, String> {
    let db = state.db.lock().await;
    db.get_folders().await.map_err(|e| e.to_string())
}

/// Remove a folder and all its videos (cascade)
#[tauri::command]
pub async fn remove_folder(folder_id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().await;
    db.remove_folder(folder_id).await.map_err(|e| e.to_string())
}
