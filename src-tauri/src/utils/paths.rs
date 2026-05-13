use std::path::{Path, PathBuf};

/// Get the .scenebrowser directory for a folder
pub fn get_scenebrowser_dir(folder_path: &Path) -> PathBuf {
    folder_path.join(".scenebrowser")
}

/// Get the database file path for a folder
#[allow(dead_code)]
pub fn get_database_path_for_folder(folder_path: &Path) -> PathBuf {
    get_scenebrowser_dir(folder_path).join("db.sqlite")
}

/// Get the thumbnail cache directory for a folder
pub fn get_thumbnail_cache_dir_for_folder(folder_path: &Path) -> PathBuf {
    get_scenebrowser_dir(folder_path).join("thumbnails")
}

/// Get the global app data directory (for storing folder list only)
pub fn get_app_data_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        dirs::home_dir()
            .expect("Failed to get home directory")
            .join("Library")
            .join("Application Support")
            .join("SceneBrowser")
    }

    #[cfg(target_os = "linux")]
    {
        dirs::data_dir()
            .expect("Failed to get data directory")
            .join("scenebrowser")
    }

    #[cfg(target_os = "windows")]
    {
        dirs::data_dir()
            .expect("Failed to get data directory")
            .join("SceneBrowser")
    }
}

/// Get the global database path (for storing folder list only)
pub fn get_database_path() -> PathBuf {
    get_app_data_dir().join("scenebrowser.db")
}

/// Get the thumbnail cache directory (deprecated - use get_thumbnail_cache_dir_for_folder instead)
pub fn get_thumbnail_cache_dir() -> PathBuf {
    get_app_data_dir().join("thumbnails")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paths_are_valid() {
        let app_dir = get_app_data_dir();
        assert!(!app_dir.as_os_str().is_empty());

        let db_path = get_database_path();
        assert!(db_path.ends_with("scenebrowser.db"));

        let cache_dir = get_thumbnail_cache_dir();
        assert!(cache_dir.ends_with("thumbnails"));
    }
}
