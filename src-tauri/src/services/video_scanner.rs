use crate::error::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Supported video file extensions
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "mov", "webm", "m4v"];

/// Video Scanner service for discovering video files in directories
pub struct VideoScanner;

impl VideoScanner {
    /// Scan a directory for video files
    pub fn scan_directory(root: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
        let mut videos = Vec::new();

        let walker = if recursive {
            WalkDir::new(root).follow_links(false)
        } else {
            WalkDir::new(root).max_depth(1).follow_links(false)
        };

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            // Skip directories
            if path.is_dir() {
                continue;
            }

            // Check if it's a video file
            if Self::is_video_file(path) {
                videos.push(path.to_path_buf());
            }
        }

        Ok(videos)
    }

    /// Check if a file is a video based on extension
    fn is_video_file(path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| VIDEO_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
    }

    /// Get the list of supported video extensions
    pub fn supported_extensions() -> &'static [&'static str] {
        VIDEO_EXTENSIONS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_is_video_file() {
        assert!(VideoScanner::is_video_file(Path::new("test.mp4")));
        assert!(VideoScanner::is_video_file(Path::new("test.MP4")));
        assert!(VideoScanner::is_video_file(Path::new("test.mkv")));
        assert!(VideoScanner::is_video_file(Path::new("test.avi")));
        assert!(!VideoScanner::is_video_file(Path::new("test.txt")));
        assert!(!VideoScanner::is_video_file(Path::new("test.jpg")));
    }

    #[test]
    fn test_scan_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        // Create test files
        fs::write(temp_path.join("video1.mp4"), b"test")?;
        fs::write(temp_path.join("video2.mkv"), b"test")?;
        fs::write(temp_path.join("not-a-video.txt"), b"test")?;

        // Create subdirectory with video
        let sub_dir = temp_path.join("subdir");
        fs::create_dir(&sub_dir)?;
        fs::write(sub_dir.join("video3.avi"), b"test")?;

        // Non-recursive scan
        let videos = VideoScanner::scan_directory(temp_path, false)?;
        assert_eq!(videos.len(), 2);

        // Recursive scan
        let videos_recursive = VideoScanner::scan_directory(temp_path, true)?;
        assert_eq!(videos_recursive.len(), 3);

        Ok(())
    }

    #[test]
    fn test_supported_extensions() {
        let extensions = VideoScanner::supported_extensions();
        assert!(extensions.contains(&"mp4"));
        assert!(extensions.contains(&"mkv"));
        assert!(extensions.contains(&"avi"));
        assert!(extensions.contains(&"mov"));
        assert!(extensions.contains(&"webm"));
        assert!(extensions.contains(&"m4v"));
    }
}
