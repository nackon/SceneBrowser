# Task: Database Schema and Migrations

## Description
SQLiteデータベースのスキーマ定義とマイグレーションシステムのセットアップを行います。

## Objectives
- [ ] マイグレーションファイルの作成
- [ ] テーブル定義の実装
- [ ] インデックスの設定
- [ ] データベース初期化コードの実装

## Steps

### 1. Create Migration File
`src-tauri/migrations/001_initial_schema.sql`:
```sql
-- folders: 登録フォルダ管理
CREATE TABLE folders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    recursive BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- videos: 動画メタデータ
CREATE TABLE videos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    folder_id INTEGER NOT NULL,
    path TEXT NOT NULL UNIQUE,
    filename TEXT NOT NULL,
    hash TEXT NOT NULL,
    duration INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    size INTEGER NOT NULL,
    codec TEXT,
    framerate REAL,
    thumbnail_path TEXT,
    thumbnail_count INTEGER DEFAULT 16,
    rating INTEGER DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    scanned_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE CASCADE
);

-- tags: タグマスタ（Phase 2で使用）
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- video_tags: 動画-タグ関連（Phase 2で使用）
CREATE TABLE video_tags (
    video_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (video_id, tag_id),
    FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Performance indexes
CREATE INDEX idx_videos_folder_id ON videos(folder_id);
CREATE INDEX idx_videos_path ON videos(path);
CREATE INDEX idx_videos_hash ON videos(hash);
CREATE INDEX idx_videos_created_at ON videos(created_at DESC);
CREATE INDEX idx_video_tags_video_id ON video_tags(video_id);
CREATE INDEX idx_video_tags_tag_id ON video_tags(tag_id);
```

### 2. Create Models
`src-tauri/src/models/folder.rs`:
```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Folder {
    pub id: i64,
    pub path: String,
    pub recursive: bool,
    pub created_at: String,
    pub updated_at: String,
}
```

`src-tauri/src/models/video.rs`:
```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Video {
    pub id: i64,
    pub folder_id: i64,
    pub path: String,
    pub filename: String,
    pub hash: String,
    pub duration: i64,
    pub width: i64,
    pub height: i64,
    pub size: i64,
    pub codec: Option<String>,
    pub framerate: Option<f64>,
    pub thumbnail_path: Option<String>,
    pub thumbnail_count: i64,
    pub rating: i64,
    pub created_at: String,
    pub updated_at: String,
    pub scanned_at: String,
}
```

### 3. Database Service Skeleton
`src-tauri/src/services/database.rs`:
基本的な接続とマイグレーション実行機能を実装

## Acceptance Criteria
- [ ] マイグレーションファイルが正しく作成されている
- [ ] モデルが全フィールドを含んでいる
- [ ] データベースが正常に初期化できる
- [ ] インデックスが適用されている

## Estimated Time
3 hours

## Dependencies
- Task #01 (Project Setup)

## Labels
- database
- phase-1
