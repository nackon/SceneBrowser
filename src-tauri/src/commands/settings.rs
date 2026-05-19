use crate::commands::folder::AppState;
use crate::models::VideoPlayerSetting;
use tauri::State;

#[tauri::command]
pub async fn get_video_player_settings(
    state: State<'_, AppState>,
) -> Result<Vec<VideoPlayerSetting>, String> {
    let db_manager = state.db_manager.lock().await;
    db_manager
        .global_db()
        .get_video_player_settings()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_video_player_setting(
    file_extension: String,
    player_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db_manager = state.db_manager.lock().await;
    db_manager
        .global_db()
        .set_video_player(&file_extension, &player_path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_video_player_setting(
    file_extension: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db_manager = state.db_manager.lock().await;
    db_manager
        .global_db()
        .delete_video_player_setting(&file_extension)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_video_with_player(
    video_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    use std::path::Path;
    use std::process::Command;

    let path = Path::new(&video_path);
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let db_manager = state.db_manager.lock().await;
    let player_path_opt = db_manager
        .global_db()
        .get_player_for_extension(extension)
        .await?;

    match player_path_opt {
        Some(player_path) if !player_path.is_empty() => {
            // Open with specified player
            #[cfg(target_os = "macos")]
            {
                Command::new("open")
                    .arg("-a")
                    .arg(&player_path)
                    .arg(&video_path)
                    .spawn()
                    .map_err(|e| format!("Failed to open video: {}", e))?;
            }

            #[cfg(target_os = "windows")]
            {
                Command::new(&player_path)
                    .arg(&video_path)
                    .spawn()
                    .map_err(|e| format!("Failed to open video: {}", e))?;
            }

            #[cfg(target_os = "linux")]
            {
                Command::new(&player_path)
                    .arg(&video_path)
                    .spawn()
                    .map_err(|e| format!("Failed to open video: {}", e))?;
            }

            Ok(())
        }
        _ => {
            // Use system default
            #[cfg(target_os = "macos")]
            {
                Command::new("open")
                    .arg(&video_path)
                    .spawn()
                    .map_err(|e| format!("Failed to open video: {}", e))?;
            }

            #[cfg(target_os = "windows")]
            {
                Command::new("cmd")
                    .args(&["/C", "start", "", &video_path])
                    .spawn()
                    .map_err(|e| format!("Failed to open video: {}", e))?;
            }

            #[cfg(target_os = "linux")]
            {
                Command::new("xdg-open")
                    .arg(&video_path)
                    .spawn()
                    .map_err(|e| format!("Failed to open video: {}", e))?;
            }

            Ok(())
        }
    }
}
