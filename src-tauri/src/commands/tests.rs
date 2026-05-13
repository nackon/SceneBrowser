#[cfg(test)]
mod tests {
    use super::super::folder::*;
    use crate::services::Database;
    use crate::utils::get_database_path;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    async fn setup_test_db() -> AppState {
        let db_path = std::env::temp_dir().join("test_scenebrowser.db");
        let _ = std::fs::remove_file(&db_path); // Clean up

        let db = Database::new(db_path)
            .await
            .expect("Failed to create test database");

        AppState {
            db: Arc::new(Mutex::new(db)),
        }
    }

    #[tokio::test]
    async fn test_add_and_get_folders() {
        let state = setup_test_db().await;

        // Add a folder
        let folder_id = add_folder("/test/path".to_string(), true, tauri::State::from(&state))
            .await
            .expect("Failed to add folder");

        assert!(folder_id > 0);

        // Get folders
        let folders = get_folders(tauri::State::from(&state))
            .await
            .expect("Failed to get folders");

        assert_eq!(folders.len(), 1);
        assert_eq!(folders[0].path, "/test/path");
        assert_eq!(folders[0].recursive, true);
    }

    #[tokio::test]
    async fn test_remove_folder() {
        let state = setup_test_db().await;

        // Add a folder
        let folder_id = add_folder("/test/path2".to_string(), false, tauri::State::from(&state))
            .await
            .expect("Failed to add folder");

        // Remove the folder
        remove_folder(folder_id, tauri::State::from(&state))
            .await
            .expect("Failed to remove folder");

        // Verify it's gone
        let folders = get_folders(tauri::State::from(&state))
            .await
            .expect("Failed to get folders");

        assert_eq!(folders.len(), 0);
    }

    #[tokio::test]
    async fn test_duplicate_folder() {
        let state = setup_test_db().await;

        // Add a folder
        let _ = add_folder("/test/dup".to_string(), true, tauri::State::from(&state))
            .await
            .expect("Failed to add folder");

        // Try to add the same folder again
        let result = add_folder("/test/dup".to_string(), true, tauri::State::from(&state)).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }
}
