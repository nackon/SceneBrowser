-- Video Player Settings
-- Created: 2026-05-19
-- Add video player configuration

-- video_player_settings: 動画プレーヤー設定
CREATE TABLE IF NOT EXISTS video_player_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_extension TEXT NOT NULL UNIQUE, -- e.g., "mp4", "mov", "*" for default
    player_path TEXT NOT NULL,           -- Full path to the player application
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- デフォルト設定を挿入（システムデフォルトプレーヤーを使用）
INSERT OR IGNORE INTO video_player_settings (file_extension, player_path)
VALUES ('*', '');
