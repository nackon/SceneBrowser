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
            commands::thumbnail::generate_thumbnails_batch,
            commands::thumbnail::read_thumbnail,
            commands::system::check_ffmpeg,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
