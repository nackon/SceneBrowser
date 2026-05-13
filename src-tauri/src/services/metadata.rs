use crate::error::{AppError, Result};
use crate::utils::ffmpeg::find_ffprobe;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

/// Video metadata extracted from ffprobe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub codec: String,
    pub framerate: f64,
}

/// FFprobe format information
#[derive(Debug, Deserialize)]
struct FFprobeFormat {
    duration: Option<String>,
}

/// FFprobe stream information
#[derive(Debug, Deserialize)]
struct FFprobeStream {
    codec_type: Option<String>,
    codec_name: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    r_frame_rate: Option<String>,
}

/// FFprobe output structure
#[derive(Debug, Deserialize)]
struct FFprobeOutput {
    format: Option<FFprobeFormat>,
    streams: Option<Vec<FFprobeStream>>,
}

/// Metadata Extractor service using ffprobe
pub struct MetadataExtractor;

impl MetadataExtractor {
    /// Extract metadata from a video file
    pub async fn extract(video_path: &Path) -> Result<VideoMetadata> {
        let ffprobe_path = match find_ffprobe()? {
            crate::utils::ffmpeg::FFmpegSource::Sidecar(path)
            | crate::utils::ffmpeg::FFmpegSource::System(path) => path,
        };

        // Run ffprobe
        let output = Command::new(&ffprobe_path)
            .args(&[
                "-v",
                "quiet",
                "-print_format",
                "json",
                "-show_format",
                "-show_streams",
                video_path.to_str().ok_or_else(|| {
                    AppError::InvalidPath("Invalid video path".to_string())
                })?,
            ])
            .output()
            .map_err(|e| AppError::FFmpegExecution(format!("Failed to run ffprobe: {}", e)))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::FFmpegExecution(format!(
                "ffprobe failed: {}",
                error
            )));
        }

        // Parse JSON output
        let probe_data: FFprobeOutput = serde_json::from_slice(&output.stdout)
            .map_err(|e| AppError::InvalidVideo(format!("Failed to parse ffprobe output: {}", e)))?;

        // Extract duration from format
        let duration = probe_data
            .format
            .and_then(|f| f.duration)
            .and_then(|d| d.parse::<f64>().ok())
            .ok_or_else(|| AppError::InvalidVideo("No duration found".to_string()))?;

        // Find video stream
        let video_stream = probe_data
            .streams
            .and_then(|streams| {
                streams.into_iter().find(|s| {
                    s.codec_type
                        .as_ref()
                        .map(|t| t == "video")
                        .unwrap_or(false)
                })
            })
            .ok_or_else(|| AppError::InvalidVideo("No video stream found".to_string()))?;

        // Extract video properties
        let width = video_stream
            .width
            .ok_or_else(|| AppError::InvalidVideo("No width found".to_string()))?;

        let height = video_stream
            .height
            .ok_or_else(|| AppError::InvalidVideo("No height found".to_string()))?;

        let codec = video_stream
            .codec_name
            .ok_or_else(|| AppError::InvalidVideo("No codec found".to_string()))?;

        // Parse frame rate (e.g., "30/1" -> 30.0)
        let framerate = video_stream
            .r_frame_rate
            .and_then(|fr| Self::parse_framerate(&fr))
            .unwrap_or(0.0);

        Ok(VideoMetadata {
            duration,
            width,
            height,
            codec,
            framerate,
        })
    }

    /// Parse frame rate string like "30/1" to f64
    fn parse_framerate(framerate_str: &str) -> Option<f64> {
        let parts: Vec<&str> = framerate_str.split('/').collect();
        if parts.len() == 2 {
            let numerator = parts[0].parse::<f64>().ok()?;
            let denominator = parts[1].parse::<f64>().ok()?;
            if denominator != 0.0 {
                return Some(numerator / denominator);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_framerate() {
        assert_eq!(MetadataExtractor::parse_framerate("30/1"), Some(30.0));
        assert_eq!(MetadataExtractor::parse_framerate("60/1"), Some(60.0));
        assert_eq!(MetadataExtractor::parse_framerate("24000/1001"), Some(23.976023976023978));
        assert_eq!(MetadataExtractor::parse_framerate("invalid"), None);
        assert_eq!(MetadataExtractor::parse_framerate("30/0"), None);
    }
}
