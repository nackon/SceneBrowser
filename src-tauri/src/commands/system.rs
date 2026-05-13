use crate::utils::check_ffmpeg_availability;

#[tauri::command]
pub async fn check_ffmpeg() -> Result<(), String> {
    check_ffmpeg_availability().map_err(|e| e.to_string())
}
