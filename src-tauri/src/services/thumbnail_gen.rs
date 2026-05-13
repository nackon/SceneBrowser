use crate::error::{AppError, Result};
use crate::utils::ffmpeg::find_ffmpeg;
use crate::utils::hashing::compute_string_hash;
use crate::utils::paths::get_thumbnail_cache_dir;
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
        let cache_dir = get_thumbnail_cache_dir();
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

        // Calculate interval between thumbnails
        let interval = duration / thumbnail_count as f64;
        let fps = 1.0 / interval;

        // Get ffmpeg path
        let ffmpeg_path = match find_ffmpeg()? {
            crate::utils::ffmpeg::FFmpegSource::Sidecar(path)
            | crate::utils::ffmpeg::FFmpegSource::System(path) => path,
        };

        // Generate thumbnail grid with ffmpeg
        let output = Command::new(&ffmpeg_path)
            .args([
                "-i",
                video_path_str,
                "-vf",
                &format!("fps={},scale=320:-1,tile={}x{}", fps, grid_size, grid_size),
                "-frames:v",
                "1",
                "-y", // Overwrite output file
                output_path
                    .to_str()
                    .ok_or_else(|| AppError::InvalidPath("Invalid output path".to_string()))?,
            ])
            .output()
            .map_err(|e| AppError::FFmpegExecution(format!("Failed to run ffmpeg: {}", e)))?;

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
        let cache_dir = get_thumbnail_cache_dir();
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

        assert!(thumbnail_path.ends_with(".jpg"));
        assert!(thumbnail_path.to_str().unwrap().contains("thumbnails"));
    }

    #[test]
    fn test_thumbnail_exists() {
        let video_path = Path::new("/nonexistent/video.mp4");
        assert!(!ThumbnailGenerator::thumbnail_exists(video_path));
    }
}
