-- SceneBrowser Per-Folder Database Schema
-- Created: 2026-05-13
-- This schema is applied to each folder's .scenebrowser/db.sqlite

-- videos: 動画メタデータ (folder_id不要、このフォルダの動画のみ)
CREATE TABLE IF NOT EXISTS videos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    filename TEXT NOT NULL,
    hash TEXT NOT NULL,
    duration INTEGER NOT NULL,         -- 秒
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    size INTEGER NOT NULL,             -- ファイルサイズ（バイト）
    codec TEXT,                        -- 動画コーデック
    framerate REAL,                    -- フレームレート
    thumbnail_path TEXT,               -- サムネイル画像パス（相対パス: .scenebrowser/thumbnails/xxx.jpg）
    thumbnail_count INTEGER DEFAULT 16, -- サムネイル枚数
    rating INTEGER DEFAULT 0,          -- 0-5段階評価（Phase 2）
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    scanned_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- tags: タグマスタ（Phase 2で使用）
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- video_tags: 動画-タグ関連（Phase 2で使用）
CREATE TABLE IF NOT EXISTS video_tags (
    video_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (video_id, tag_id),
    FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Performance indexes
CREATE INDEX IF NOT EXISTS idx_videos_path ON videos(path);
CREATE INDEX IF NOT EXISTS idx_videos_hash ON videos(hash);
CREATE INDEX IF NOT EXISTS idx_videos_created_at ON videos(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_video_tags_video_id ON video_tags(video_id);
CREATE INDEX IF NOT EXISTS idx_video_tags_tag_id ON video_tags(tag_id);
