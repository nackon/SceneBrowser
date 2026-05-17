mod commands;
mod error;
mod models;
mod services;
mod utils;

pub use error::{AppError, Result};
pub use models::{Folder, Video, VideoInsert};
pub use services::{
    DatabaseManager, FolderDatabase, GlobalDatabase, MetadataExtractor, ThumbnailGenerator,
    VideoMetadata, VideoScanner,
};
pub use utils::{
    check_ffmpeg_availability, compute_file_hash, compute_string_hash, find_ffmpeg, find_ffprobe,
    get_app_data_dir, get_database_path, get_ffmpeg_version, get_ffprobe_version,
    get_thumbnail_cache_dir,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use commands::folder::AppState;
    use std::sync::Arc;
    use tauri::Manager;
    use tokio::sync::Mutex;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Debug: Log FFmpeg detection on startup
            // Write to both stderr and a log file for debugging
            use std::fs::OpenOptions;
            use std::io::Write;

            let log_path = std::env::var("HOME")
                .map(|home| format!("{}/scenebrowser-debug.log", home))
                .unwrap_or_else(|_| "/tmp/scenebrowser-debug.log".to_string());

            let mut log_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
                .ok();

            let mut log_msg = |msg: &str| {
                eprintln!("{}", msg);
                if let Some(ref mut file) = log_file {
                    let _ = writeln!(file, "{}", msg);
                }
            };

            log_msg("=== SceneBrowser Starting ===");
            log_msg(&format!("[DEBUG] Log file: {}", log_path));
            log_msg("[DEBUG] Checking FFmpeg availability on startup...");

            match check_ffmpeg_availability() {
                Ok(_) => {
                    log_msg("[DEBUG] FFmpeg check successful");
                    if let Ok(ffmpeg_version) = get_ffmpeg_version() {
                        log_msg(&format!("[DEBUG] FFmpeg version: {}", ffmpeg_version));
                    }
                }
                Err(e) => {
                    log_msg(&format!("[DEBUG] FFmpeg check FAILED: {:?}", e));
                }
            }

            // Initialize database manager
            let db_manager = tauri::async_runtime::block_on(async { DatabaseManager::new().await })
                .expect("Failed to initialize database manager");

            // Create app state
            app.manage(AppState {
                db_manager: Arc::new(Mutex::new(db_manager)),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::folder::add_folder,
            commands::folder::get_folders,
            commands::folder::remove_folder,
            commands::video::scan_folder,
            commands::video::get_videos,
            commands::video::search_videos,
            commands::video::get_video_by_id,
            commands::thumbnail::generate_thumbnail,
            commands::thumbnail::regenerate_thumbnail,
            commands::thumbnail::generate_thumbnails_batch,
            commands::thumbnail::read_thumbnail,
            commands::system::check_ffmpeg,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
