use std::path::PathBuf;

/// Get the application data directory
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

/// Get the database file path
pub fn get_database_path() -> PathBuf {
    get_app_data_dir().join("scenebrowser.db")
}

/// Get the thumbnail cache directory
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
