use crate::error::{AppError, Result};
use crate::utils::ffmpeg::find_ffmpeg;
use crate::utils::hashing::compute_string_hash;
use crate::utils::paths::get_thumbnail_cache_dir_for_folder;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Thumbnail Generator service using ffmpeg
pub struct ThumbnailGenerator;

impl ThumbnailGenerator {
    /// Generate a thumbnail grid from a video file
    ///
    /// # Arguments
    /// * `video_path` - Path to the video file
    /// * `duration` - Duration of the video in seconds
    /// * `thumbnail_count` - Number of thumbnails to generate (default: 16)
    ///
    /// # Returns
    /// Path to the generated thumbnail file
    pub async fn generate(
        video_path: &Path,
        duration: f64,
        thumbnail_count: usize,
    ) -> Result<PathBuf> {
        // Get the folder containing the video
        let folder_path = video_path
            .parent()
            .ok_or_else(|| AppError::InvalidPath("Video has no parent directory".to_string()))?;

        let cache_dir = get_thumbnail_cache_dir_for_folder(folder_path);
        std::fs::create_dir_all(&cache_dir)?;

        // Generate hash-based filename
        let video_path_str = video_path
            .to_str()
            .ok_or_else(|| AppError::InvalidPath("Invalid video path".to_string()))?;
        let hash = compute_string_hash(video_path_str);
        let output_path = cache_dir.join(format!("{}.jpg", hash));

        // Check if thumbnail already exists
        if output_path.exists() {
            return Ok(output_path);
        }

        // Calculate grid dimensions (e.g., 16 -> 4x4)
        let grid_size = (thumbnail_count as f64).sqrt().ceil() as usize;

        // Get ffmpeg path
        let ffmpeg_path = match find_ffmpeg()? {
            crate::utils::ffmpeg::FFmpegSource::Sidecar(path)
            | crate::utils::ffmpeg::FFmpegSource::System(path) => path,
        };

        // Calculate fps to extract exactly thumbnail_count frames
        // fps = total_frames / duration, where total_frames = thumbnail_count
        let fps = thumbnail_count as f64 / duration;

        // Generate thumbnail grid with ffmpeg
        // Use fps filter to extract frames at regular intervals, then tile them
        // Run in blocking task to allow true parallelism
        let ffmpeg_path_clone = ffmpeg_path.clone();
        let output_path_clone = output_path.clone();
        let vf_string = format!("fps={},scale=320:-1,tile={}x{}", fps, grid_size, grid_size);
        let video_path_string = video_path_str.to_string();

        let output = tokio::task::spawn_blocking(move || {
            Command::new(&ffmpeg_path_clone)
                .args([
                    "-threads",
                    "2", // Use 2 threads per process for faster encoding
                    "-i",
                    &video_path_string,
                    "-vf",
                    &vf_string,
                    "-frames:v",
                    "1",
                    "-q:v",
                    "3",        // JPEG quality (2-5 is good, lower = better quality)
                    "-y",       // Overwrite output file
                    "-nostats", // Disable progress stats
                    "-loglevel",
                    "error", // Only show errors
                    output_path_clone
                        .to_str()
                        .ok_or_else(|| AppError::InvalidPath("Invalid output path".to_string()))?,
                ])
                .output()
                .map_err(|e| AppError::FFmpegExecution(format!("Failed to run ffmpeg: {}", e)))
        })
        .await
        .map_err(|e| AppError::FFmpegExecution(format!("Failed to spawn ffmpeg task: {}", e)))??;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::FFmpegExecution(format!(
                "ffmpeg failed: {}",
                error
            )));
        }

        // Verify thumbnail was created
        if !output_path.exists() {
            return Err(AppError::FFmpegExecution(
                "Thumbnail file was not created".to_string(),
            ));
        }

        Ok(output_path)
    }

    /// Get the cached thumbnail path for a video (without generating)
    pub fn get_cached_thumbnail_path(video_path: &Path) -> PathBuf {
        let folder_path = video_path.parent().unwrap_or(Path::new("."));
        let cache_dir = get_thumbnail_cache_dir_for_folder(folder_path);
        let video_path_str = video_path.to_str().unwrap_or("");
        let hash = compute_string_hash(video_path_str);
        cache_dir.join(format!("{}.jpg", hash))
    }

    /// Check if a thumbnail exists for a video
    pub fn thumbnail_exists(video_path: &Path) -> bool {
        let thumbnail_path = Self::get_cached_thumbnail_path(video_path);
        thumbnail_path.exists()
    }

    /// Delete cached thumbnail for a video
    pub fn delete_thumbnail(video_path: &Path) -> Result<()> {
        let thumbnail_path = Self::get_cached_thumbnail_path(video_path);
        if thumbnail_path.exists() {
            std::fs::remove_file(thumbnail_path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cached_thumbnail_path() {
        let video_path = Path::new("/test/video.mp4");
        let thumbnail_path = ThumbnailGenerator::get_cached_thumbnail_path(video_path);

        assert!(thumbnail_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e == "jpg")
            .unwrap_or(false));
        assert!(thumbnail_path.to_str().unwrap().contains("thumbnails"));
    }

    #[test]
    fn test_thumbnail_exists() {
        let video_path = Path::new("/nonexistent/video.mp4");
        assert!(!ThumbnailGenerator::thumbnail_exists(video_path));
    }
}
