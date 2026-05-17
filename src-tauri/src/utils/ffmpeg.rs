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
    // Debug: Log current PATH
    if let Ok(path_env) = std::env::var("PATH") {
        eprintln!("[DEBUG] PATH environment variable: {}", path_env);
    } else {
        eprintln!("[DEBUG] PATH environment variable not set");
    }

    // Try system PATH first (works in development and when PATH is properly set)
    match which::which("ffmpeg") {
        Ok(path) => {
            eprintln!("[DEBUG] Found ffmpeg in PATH: {:?}", path);
            return Ok(FFmpegSource::System(path));
        }
        Err(e) => {
            eprintln!("[DEBUG] ffmpeg not found in PATH: {}", e);
        }
    }

    // Check common installation paths (for macOS .app bundles where PATH is limited)
    let common_paths = [
        "/opt/homebrew/bin/ffmpeg", // Apple Silicon Homebrew
        "/usr/local/bin/ffmpeg",    // Intel Homebrew
        "/opt/local/bin/ffmpeg",    // MacPorts
        "/usr/bin/ffmpeg",          // System install
    ];

    eprintln!("[DEBUG] Checking common installation paths:");
    for path_str in &common_paths {
        let path = PathBuf::from(path_str);
        eprintln!("[DEBUG]   Trying: {}", path_str);

        // Check if file exists before canonicalizing
        if path.exists() {
            eprintln!("[DEBUG]     File exists");
            // Try to canonicalize the path (resolves symlinks and checks existence)
            match path.canonicalize() {
                Ok(canonical_path) => {
                    eprintln!("[DEBUG]     Canonicalized to: {:?}", canonical_path);
                    return Ok(FFmpegSource::System(canonical_path));
                }
                Err(e) => {
                    eprintln!("[DEBUG]     Failed to canonicalize: {}", e);
                }
            }
        } else {
            eprintln!("[DEBUG]     File does not exist");
        }
    }

    eprintln!("[DEBUG] ffmpeg not found in any location");
    Err(AppError::FFmpegNotFound)
}

/// Find ffprobe binary (sidecar or system)
pub fn find_ffprobe() -> Result<FFmpegSource> {
    // Try system PATH first (works in development and when PATH is properly set)
    match which::which("ffprobe") {
        Ok(path) => {
            eprintln!("[DEBUG] Found ffprobe in PATH: {:?}", path);
            return Ok(FFmpegSource::System(path));
        }
        Err(e) => {
            eprintln!("[DEBUG] ffprobe not found in PATH: {}", e);
        }
    }

    // Check common installation paths (for macOS .app bundles where PATH is limited)
    let common_paths = [
        "/opt/homebrew/bin/ffprobe", // Apple Silicon Homebrew
        "/usr/local/bin/ffprobe",    // Intel Homebrew
        "/opt/local/bin/ffprobe",    // MacPorts
        "/usr/bin/ffprobe",          // System install
    ];

    eprintln!("[DEBUG] Checking common installation paths for ffprobe:");
    for path_str in &common_paths {
        let path = PathBuf::from(path_str);
        eprintln!("[DEBUG]   Trying: {}", path_str);

        // Check if file exists before canonicalizing
        if path.exists() {
            eprintln!("[DEBUG]     File exists");
            // Try to canonicalize the path (resolves symlinks and checks existence)
            match path.canonicalize() {
                Ok(canonical_path) => {
                    eprintln!("[DEBUG]     Canonicalized to: {:?}", canonical_path);
                    return Ok(FFmpegSource::System(canonical_path));
                }
                Err(e) => {
                    eprintln!("[DEBUG]     Failed to canonicalize: {}", e);
                }
            }
        } else {
            eprintln!("[DEBUG]     File does not exist");
        }
    }

    eprintln!("[DEBUG] ffprobe not found in any location");
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
        return Err(AppError::FFmpegExecution(
            "Failed to get version".to_string(),
        ));
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
        return Err(AppError::FFmpegExecution(
            "Failed to get version".to_string(),
        ));
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
