mod commands;
mod error;
mod models;
mod services;
mod utils;

pub use error::{AppError, Result};
pub use models::{Folder, Video, VideoInsert};
pub use services::{Database, MetadataExtractor, ThumbnailGenerator, VideoMetadata, VideoScanner};
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
            // Initialize database
            let db_path = get_database_path();
            let db = tauri::async_runtime::block_on(async { Database::new(db_path).await })
                .expect("Failed to initialize database");

            // Create app state
            app.manage(AppState {
                db: Arc::new(Mutex::new(db)),
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
