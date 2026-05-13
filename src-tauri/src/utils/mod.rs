pub mod ffmpeg;
pub mod hashing;
pub mod paths;

pub use ffmpeg::{
    check_ffmpeg_availability, find_ffmpeg, find_ffprobe, get_ffmpeg_version, get_ffprobe_version,
};
pub use hashing::{compute_file_hash, compute_string_hash};
pub use paths::{get_app_data_dir, get_database_path, get_thumbnail_cache_dir};
