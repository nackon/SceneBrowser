#[cfg(test)]
mod tests {
    use super::super::folder::*;
    use crate::services::Database;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    async fn setup_test_db() -> AppState {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let count = COUNTER.fetch_add(1, Ordering::SeqCst);
        let db_path = std::env::temp_dir().join(format!(
            "test_scenebrowser_{}_{}.db",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            count
        ));
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

        // Add a folder directly through database
        let db = state.db.lock().await;
        let folder_id = db
            .add_folder("/test/path", true)
            .await
            .expect("Failed to add folder");

        assert!(folder_id > 0);

        // Get folders
        let folders = db.get_folders().await.expect("Failed to get folders");

        assert_eq!(folders.len(), 1);
        assert_eq!(folders[0].path, "/test/path");
        assert_eq!(folders[0].recursive, true);
    }

    #[tokio::test]
    async fn test_remove_folder() {
        let state = setup_test_db().await;

        let db = state.db.lock().await;

        // Add a folder
        let folder_id = db
            .add_folder("/test/path2", false)
            .await
            .expect("Failed to add folder");

        // Remove the folder
        db.remove_folder(folder_id)
            .await
            .expect("Failed to remove folder");

        // Verify it's gone
        let folders = db.get_folders().await.expect("Failed to get folders");

        assert_eq!(folders.len(), 0);
    }

    #[tokio::test]
    async fn test_duplicate_folder() {
        let state = setup_test_db().await;

        let db = state.db.lock().await;

        // Add a folder
        let _ = db
            .add_folder("/test/dup", true)
            .await
            .expect("Failed to add folder");

        // Try to add the same folder again
        let result = db.add_folder("/test/dup", true).await;

        assert!(result.is_err());
    }
}
