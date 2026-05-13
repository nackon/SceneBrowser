#[cfg(test)]
mod tests {
    use super::super::folder::*;
    use crate::services::DatabaseManager;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    async fn setup_test_db() -> AppState {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let count = COUNTER.fetch_add(1, Ordering::SeqCst);

        // Create a temporary directory for testing
        let test_dir = std::env::temp_dir().join(format!(
            "test_scenebrowser_{}_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            count
        ));
        std::fs::create_dir_all(&test_dir).expect("Failed to create test directory");

        // Temporarily override get_database_path for testing
        // Since we can't override functions, we'll just use the real DatabaseManager
        // but clean up the test folders afterward
        let db_manager = DatabaseManager::new()
            .await
            .expect("Failed to create test database manager");

        AppState {
            db_manager: Arc::new(Mutex::new(db_manager)),
        }
    }

    #[tokio::test]
    async fn test_add_and_get_folders() {
        let state = setup_test_db().await;

        // Add a folder directly through database
        let db_manager = state.db_manager.lock().await;
        let global_db = db_manager.global_db();

        let test_path = "/test/path_add_get";
        let folder_id = global_db
            .add_folder(test_path, true)
            .await
            .expect("Failed to add folder");

        assert!(folder_id > 0);

        // Get folders
        let folders = global_db
            .get_folders()
            .await
            .expect("Failed to get folders");

        // Find our test folder
        let test_folder = folders.iter().find(|f| f.path == test_path);
        assert!(test_folder.is_some());
        assert!(test_folder.unwrap().recursive);

        // Cleanup
        global_db.remove_folder(folder_id).await.ok();
    }

    #[tokio::test]
    async fn test_remove_folder() {
        let state = setup_test_db().await;

        let db_manager = state.db_manager.lock().await;
        let global_db = db_manager.global_db();

        let test_path = "/test/path_remove";
        // Add a folder
        let folder_id = global_db
            .add_folder(test_path, false)
            .await
            .expect("Failed to add folder");

        // Remove the folder
        global_db
            .remove_folder(folder_id)
            .await
            .expect("Failed to remove folder");

        // Verify it's gone
        let folders = global_db
            .get_folders()
            .await
            .expect("Failed to get folders");
        let test_folder = folders.iter().find(|f| f.path == test_path);
        assert!(test_folder.is_none());
    }

    #[tokio::test]
    async fn test_duplicate_folder() {
        let state = setup_test_db().await;

        let db_manager = state.db_manager.lock().await;
        let global_db = db_manager.global_db();

        let test_path = "/test/dup_test";
        // Add a folder
        let folder_id = global_db
            .add_folder(test_path, true)
            .await
            .expect("Failed to add folder");

        // Try to add the same folder again
        let result = global_db.add_folder(test_path, true).await;

        assert!(result.is_err());

        // Cleanup
        global_db.remove_folder(folder_id).await.ok();
    }
}
