use crate::models::Folder;
use crate::services::DatabaseManager;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};
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

/// Progress information for folder deletion
#[derive(Clone, Serialize)]
pub struct DeleteProgress {
    pub phase: String,
    pub current: usize,
    pub total: usize,
}

/// Remove a folder and all its videos/thumbnails
#[tauri::command]
pub async fn remove_folder(
    folder_id: i64,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db_manager = state.db_manager.lock().await;
    let global_db = db_manager.global_db();

    // Get folder path before deletion
    let folder = global_db
        .get_folder_by_id(folder_id)
        .await
        .map_err(|e| e.to_string())?;

    let folder_path = PathBuf::from(&folder.path);
    let scenebrowser_dir = crate::utils::paths::get_scenebrowser_dir(&folder_path);
    let thumbnails_dir = crate::utils::paths::get_thumbnail_cache_dir_for_folder(&folder_path);

    // Phase 1: Count thumbnails for progress tracking
    let mut total_thumbnails = 0;
    if thumbnails_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&thumbnails_dir) {
            total_thumbnails = entries.count();
        }
    }

    // Phase 2: Delete thumbnails
    if thumbnails_dir.exists() {
        app.emit(
            "delete_progress",
            DeleteProgress {
                phase: "Deleting thumbnails".to_string(),
                current: 0,
                total: total_thumbnails,
            },
        )
        .ok();

        if let Ok(entries) = std::fs::read_dir(&thumbnails_dir) {
            for (i, entry) in entries.enumerate() {
                if let Ok(entry) = entry {
                    let _ = std::fs::remove_file(entry.path());

                    if i % 10 == 0 || i == total_thumbnails - 1 {
                        app.emit(
                            "delete_progress",
                            DeleteProgress {
                                phase: "Deleting thumbnails".to_string(),
                                current: i + 1,
                                total: total_thumbnails,
                            },
                        )
                        .ok();
                    }
                }
            }
        }
    }

    // Phase 3: Delete database
    app.emit(
        "delete_progress",
        DeleteProgress {
            phase: "Removing from database".to_string(),
            current: 0,
            total: 1,
        },
    )
    .ok();

    global_db
        .remove_folder(folder_id)
        .await
        .map_err(|e| e.to_string())?;

    // Phase 4: Delete .scenebrowser directory
    if scenebrowser_dir.exists() {
        app.emit(
            "delete_progress",
            DeleteProgress {
                phase: "Cleaning up directory".to_string(),
                current: 0,
                total: 1,
            },
        )
        .ok();

        std::fs::remove_dir_all(&scenebrowser_dir).map_err(|e| e.to_string())?;
    }

    app.emit(
        "delete_progress",
        DeleteProgress {
            phase: "Complete".to_string(),
            current: 1,
            total: 1,
        },
    )
    .ok();

    Ok(())
}
