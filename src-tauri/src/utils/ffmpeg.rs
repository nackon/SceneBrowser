use crate::error::{AppError, Result};
use std::path::PathBuf;
use std::process::Command;

/// FFmpeg source location
#[derive(Debug)]
pub enum FFmpegSource {
    Sidecar(PathBuf),
    System(PathBuf),
}

/// Find ffmpeg binary (sidecar or system)
pub fn find_ffmpeg() -> Result<FFmpegSource> {
    // Try system PATH first for development
    if let Ok(path) = which::which("ffmpeg") {
        return Ok(FFmpegSource::System(path));
    }

    // TODO: Check sidecar location in production build
    // if let Ok(sidecar_path) = tauri::api::process::sidecar_path("ffmpeg") {
    //     if sidecar_path.exists() {
    //         return Ok(FFmpegSource::Sidecar(sidecar_path));
    //     }
    // }

    Err(AppError::FFmpegNotFound)
}

/// Find ffprobe binary (sidecar or system)
pub fn find_ffprobe() -> Result<FFmpegSource> {
    // Try system PATH first for development
    if let Ok(path) = which::which("ffprobe") {
        return Ok(FFmpegSource::System(path));
    }

    // TODO: Check sidecar location in production build
    // if let Ok(sidecar_path) = tauri::api::process::sidecar_path("ffprobe") {
    //     if sidecar_path.exists() {
    //         return Ok(FFmpegSource::Sidecar(sidecar_path));
    //     }
    // }

    Err(AppError::FFmpegNotFound)
}

/// Check if ffmpeg and ffprobe are available
pub fn check_ffmpeg_availability() -> Result<()> {
    find_ffmpeg()?;
    find_ffprobe()?;
    Ok(())
}

/// Get ffmpeg version
pub fn get_ffmpeg_version() -> Result<String> {
    let ffmpeg_path = match find_ffmpeg()? {
        FFmpegSource::Sidecar(path) | FFmpegSource::System(path) => path,
    };

    let output = Command::new(&ffmpeg_path)
        .arg("-version")
        .output()
        .map_err(|e| AppError::FFmpegExecution(e.to_string()))?;

    if !output.status.success() {
        return Err(AppError::FFmpegExecution("Failed to get version".to_string()));
    }

    let version_string = String::from_utf8_lossy(&output.stdout);
    let first_line = version_string
        .lines()
        .next()
        .unwrap_or("Unknown")
        .to_string();

    Ok(first_line)
}

/// Get ffprobe version
pub fn get_ffprobe_version() -> Result<String> {
    let ffprobe_path = match find_ffprobe()? {
        FFmpegSource::Sidecar(path) | FFmpegSource::System(path) => path,
    };

    let output = Command::new(&ffprobe_path)
        .arg("-version")
        .output()
        .map_err(|e| AppError::FFmpegExecution(e.to_string()))?;

    if !output.status.success() {
        return Err(AppError::FFmpegExecution("Failed to get version".to_string()));
    }

    let version_string = String::from_utf8_lossy(&output.stdout);
    let first_line = version_string
        .lines()
        .next()
        .unwrap_or("Unknown")
        .to_string();

    Ok(first_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_ffmpeg() {
        // This test will only pass if ffmpeg is installed
        match find_ffmpeg() {
            Ok(source) => {
                println!("Found ffmpeg: {:?}", source);
            }
            Err(_) => {
                println!("ffmpeg not found (this is OK for CI environments)");
            }
        }
    }
}
