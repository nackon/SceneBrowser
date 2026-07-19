use crate::commands::folder::AppState;
use crate::services::ThumbnailGenerator;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{Emitter, State};

/// Generate a thumbnail for a video
#[tauri::command]
pub async fn generate_thumbnail(
    folder_id: i64,
    video_id: i64,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let db_manager = state.db_manager.lock().await;

    // Resolve the specific folder this video belongs to
    let folder = db_manager
        .global_db()
        .get_folder_by_id(folder_id)
        .await
        .map_err(|e| e.to_string())?;

    let folder_path = PathBuf::from(&folder.path);
    let folder_db = db_manager
        .get_folder_db(&folder_path)
        .await
        .map_err(|e| e.to_string())?;

    let video = folder_db
        .get_video_by_id(video_id)
        .await
        .map_err(|e| e.to_string())?;

    drop(db_manager); // Release lock before long operation

    // Generate thumbnail
    let thumbnail_path = ThumbnailGenerator::generate(
        Path::new(&video.path),
        video.duration as f64,
        video.thumbnail_count as usize,
    )
    .await
    .map_err(|e| format!("Failed to generate thumbnail: {}", e))?;

    let thumbnail_path_str = thumbnail_path.to_string_lossy().to_string();

    // Update database with thumbnail path in a separate task to avoid blocking
    let state_clone = state.inner().clone();
    let thumbnail_path_clone = thumbnail_path_str.clone();
    let folder_path_clone = folder_path.clone();
    tokio::spawn(async move {
        let db_manager = state_clone.db_manager.lock().await;
        if let Ok(folder_db) = db_manager.get_folder_db(&folder_path_clone).await {
            let _ = folder_db
                .update_video_thumbnail(video_id, &thumbnail_path_clone)
                .await;
        }
    });

    Ok(thumbnail_path_str)
}

/// Regenerate a thumbnail for a video (deletes existing and generates new)
#[tauri::command]
pub async fn regenerate_thumbnail(
    folder_id: i64,
    video_id: i64,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let db_manager = state.db_manager.lock().await;

    // Resolve the specific folder this video belongs to
    let folder = db_manager
        .global_db()
        .get_folder_by_id(folder_id)
        .await
        .map_err(|e| e.to_string())?;

    let folder_path = PathBuf::from(&folder.path);
    let folder_db = db_manager
        .get_folder_db(&folder_path)
        .await
        .map_err(|e| e.to_string())?;

    let video = folder_db
        .get_video_by_id(video_id)
        .await
        .map_err(|e| e.to_string())?;

    drop(db_manager); // Release lock before long operation

    // Delete existing thumbnail if it exists
    let video_path = Path::new(&video.path);
    let _ = ThumbnailGenerator::delete_thumbnail(video_path);

    // Generate new thumbnail
    let thumbnail_path = ThumbnailGenerator::generate(
        video_path,
        video.duration as f64,
        video.thumbnail_count as usize,
    )
    .await
    .map_err(|e| format!("Failed to generate thumbnail: {}", e))?;

    let thumbnail_path_str = thumbnail_path.to_string_lossy().to_string();

    // Update database with thumbnail path
    let db_manager = state.db_manager.lock().await;
    let folder_db = db_manager
        .get_folder_db(&folder_path)
        .await
        .map_err(|e| e.to_string())?;
    folder_db
        .update_video_thumbnail(video_id, &thumbnail_path_str)
        .await
        .map_err(|e| e.to_string())?;

    Ok(thumbnail_path_str)
}

/// Read thumbnail file and return as base64 data URL
#[tauri::command]
pub async fn read_thumbnail(thumbnail_path: String) -> Result<String, String> {
    let bytes =
        fs::read(&thumbnail_path).map_err(|e| format!("Failed to read thumbnail file: {}", e))?;

    let base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
    Ok(format!("data:image/jpeg;base64,{}", base64))
}

/// Generate thumbnails for multiple videos in parallel
#[tauri::command]
pub async fn generate_thumbnails_batch(
    folder_id: i64,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    use futures::stream::{self, StreamExt};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize)]
    struct ThumbnailProgress {
        current: usize,
        total: usize,
        current_file: String,
    }

    #[derive(Clone, Serialize, Deserialize)]
    struct ThumbnailGenerated {
        video_id: i64,
        thumbnail_path: String,
    }

    let db_manager = state.db_manager.lock().await;

    // Get folder info
    let global_db = db_manager.global_db();
    let folder = global_db
        .get_folder_by_id(folder_id)
        .await
        .map_err(|e| e.to_string())?;

    let folder_path = PathBuf::from(&folder.path);
    let folder_db = db_manager
        .get_folder_db(&folder_path)
        .await
        .map_err(|e| e.to_string())?;

    // Get all videos without thumbnails
    let videos = folder_db
        .get_all_videos()
        .await
        .map_err(|e| e.to_string())?;

    let videos_to_process: Vec<_> = videos
        .into_iter()
        .filter(|v| {
            // Process if no thumbnail path in DB
            if v.thumbnail_path.is_none()
                || v.thumbnail_path
                    .as_ref()
                    .map(|p| p.is_empty())
                    .unwrap_or(true)
            {
                return true;
            }
            // Process if thumbnail file doesn't exist
            if let Some(path) = &v.thumbnail_path {
                !Path::new(path).exists()
            } else {
                true
            }
        })
        .collect();

    let total = videos_to_process.len();

    if total == 0 {
        return Ok(0);
    }

    drop(db_manager);

    // Process videos in parallel (max 4 concurrent)
    let generated = std::sync::atomic::AtomicUsize::new(0);

    stream::iter(videos_to_process)
        .enumerate()
        .map(|(idx, video)| {
            let window = window.clone();
            let state = state.inner().clone();
            let folder_path = folder_path.clone();
            let generated = &generated;

            async move {
                // Emit progress
                let _ = window.emit(
                    "thumbnail_progress",
                    ThumbnailProgress {
                        current: idx + 1,
                        total,
                        current_file: video.filename.clone(),
                    },
                );

                // Generate thumbnail
                let result = ThumbnailGenerator::generate(
                    Path::new(&video.path),
                    video.duration as f64,
                    video.thumbnail_count as usize,
                )
                .await;

                if let Ok(thumbnail_path) = result {
                    let thumbnail_path_str = thumbnail_path.to_string_lossy().to_string();

                    // Update database
                    let db_manager = state.db_manager.lock().await;
                    if let Ok(folder_db) = db_manager.get_folder_db(&folder_path).await {
                        let _ = folder_db
                            .update_video_thumbnail(video.id, &thumbnail_path_str)
                            .await;
                    }
                    drop(db_manager);

                    // Emit event for individual thumbnail completion
                    let _ = window.emit(
                        "thumbnail_generated",
                        ThumbnailGenerated {
                            video_id: video.id,
                            thumbnail_path: thumbnail_path_str,
                        },
                    );

                    generated.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            }
        })
        .buffer_unordered(4) // Process 4 videos concurrently
        .collect::<Vec<_>>()
        .await;

    Ok(generated.load(std::sync::atomic::Ordering::Relaxed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_thumbnail_with_valid_file() {
        // Create a temporary JPEG file
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_thumbnail.jpg");

        // Write a minimal JPEG (1x1 pixel, black)
        let jpeg_data = vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x08, 0x06, 0x06,
            0x07, 0x06, 0x05, 0x08, 0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14, 0x0D,
            0x0C, 0x0B, 0x0B, 0x0C, 0x19, 0x12, 0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D,
            0x1A, 0x1C, 0x1C, 0x20, 0x24, 0x2E, 0x27, 0x20, 0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28,
            0x37, 0x29, 0x2C, 0x30, 0x31, 0x34, 0x34, 0x34, 0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32,
            0x3C, 0x2E, 0x33, 0x34, 0x32, 0xFF, 0xC0, 0x00, 0x0B, 0x08, 0x00, 0x01, 0x00, 0x01,
            0x01, 0x01, 0x11, 0x00, 0xFF, 0xC4, 0x00, 0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xFF, 0xC4,
            0x00, 0x14, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00,
            0x3F, 0x00, 0x37, 0xFF, 0xD9,
        ];

        std::fs::write(&test_file, &jpeg_data).unwrap();

        // Test reading the thumbnail
        let result = tokio_test::block_on(read_thumbnail(test_file.to_string_lossy().to_string()));

        assert!(result.is_ok());
        let data_url = result.unwrap();
        assert!(data_url.starts_with("data:image/jpeg;base64,"));

        // Cleanup
        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_read_thumbnail_with_nonexistent_file() {
        let result = tokio_test::block_on(read_thumbnail("/nonexistent/path.jpg".to_string()));

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Failed to read thumbnail file"));
    }

    #[test]
    fn test_thumbnail_progress_struct() {
        // Test that ThumbnailProgress can be serialized
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Serialize, Deserialize)]
        struct ThumbnailProgress {
            current: usize,
            total: usize,
            current_file: String,
        }

        let progress = ThumbnailProgress {
            current: 5,
            total: 10,
            current_file: "test.mp4".to_string(),
        };

        let json = serde_json::to_string(&progress).unwrap();
        assert!(json.contains("\"current\":5"));
        assert!(json.contains("\"total\":10"));
        assert!(json.contains("\"current_file\":\"test.mp4\""));
    }

    #[test]
    fn test_thumbnail_generated_struct() {
        // Test that ThumbnailGenerated can be serialized
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Serialize, Deserialize)]
        struct ThumbnailGenerated {
            video_id: i64,
            thumbnail_path: String,
        }

        let generated = ThumbnailGenerated {
            video_id: 42,
            thumbnail_path: "/cache/thumbnail.jpg".to_string(),
        };

        let json = serde_json::to_string(&generated).unwrap();
        assert!(json.contains("\"video_id\":42"));
        assert!(json.contains("\"thumbnail_path\":\"/cache/thumbnail.jpg\""));
    }
}
