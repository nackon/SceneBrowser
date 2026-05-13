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
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
