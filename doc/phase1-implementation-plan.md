# Phase 1 (MVP) Implementation Plan

## Context

SceneBrowserは、macOS向けのローカル動画コレクション管理アプリケーションです。大量の動画ファイル（目標10万件）を複数サムネイル表示で視覚的に一覧・検索・管理することを目的とします。WhiteBrowserのような「内容を視覚的に探す体験」をmacOSネイティブに近いUXで実現します。

spec.mdに基づき、Phase 1 (MVP) の実装設計を行います：
- フォルダ登録・スキャン
- サムネイル生成（16枚デフォルト）
- 一覧表示（グリッド、仮想スクロール）
- 基本的な動画プレビュー

## Recommended Approach

### 1. Project Structure Setup

Tauriの標準構成に従い、以下のディレクトリ構造を構築：

```
SceneBrowser/
├── src/                              # React/TypeScript frontend
│   ├── main.tsx
│   ├── App.tsx
│   ├── components/
│   │   ├── VideoGrid.tsx
│   │   ├── VideoCard.tsx
│   │   ├── Sidebar.tsx
│   │   ├── FolderList.tsx
│   │   └── VideoPreview.tsx
│   ├── services/
│   │   └── commands.ts              # Tauri command wrappers
│   ├── types/
│   │   └── video.ts                 # TypeScript interfaces
│   └── hooks/
│       └── useVideos.ts
│
├── src-tauri/                        # Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── folder.rs           # Folder registration
│   │   │   ├── video.rs            # Video scanning/queries
│   │   │   └── thumbnail.rs        # Thumbnail generation
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── video.rs
│   │   │   └── folder.rs
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── database.rs         # SQLite operations
│   │   │   ├── video_scanner.rs    # Video file scanning
│   │   │   ├── thumbnail_gen.rs    # ffmpeg thumbnail gen
│   │   │   └── metadata.rs         # ffprobe integration
│   │   ├── utils/
│   │   │   ├── mod.rs
│   │   │   ├── ffmpeg.rs
│   │   │   ├── paths.rs
│   │   │   └── hashing.rs
│   │   └── error.rs
│   ├── binaries/                     # ffmpeg/ffprobe sidecars
│   └── migrations/
│       └── 001_initial_schema.sql
│
├── package.json
├── vite.config.ts
├── tsconfig.json
└── README.md
```

### 2. Database Schema Design

SQLiteを使用。spec.mdの設計を拡張：

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
    hash TEXT NOT NULL,                -- SHA256ハッシュ（重複検出用）
    duration INTEGER NOT NULL,         -- 秒
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    size INTEGER NOT NULL,             -- ファイルサイズ（バイト）
    codec TEXT,                        -- 動画コーデック
    framerate REAL,                    -- フレームレート
    thumbnail_path TEXT,               -- サムネイル画像パス
    thumbnail_count INTEGER DEFAULT 16, -- サムネイル枚数
    rating INTEGER DEFAULT 0,          -- 0-5段階評価（Phase 2）
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

-- indexes: パフォーマンス最適化
CREATE INDEX idx_videos_folder_id ON videos(folder_id);
CREATE INDEX idx_videos_path ON videos(path);
CREATE INDEX idx_videos_hash ON videos(hash);
CREATE INDEX idx_videos_created_at ON videos(created_at DESC);
CREATE INDEX idx_video_tags_video_id ON video_tags(video_id);
CREATE INDEX idx_video_tags_tag_id ON video_tags(tag_id);
```

### 3. Core Services Architecture

#### 3.1 Database Service (`src-tauri/src/services/database.rs`)

```rust
use sqlx::SqlitePool;
use std::path::PathBuf;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let db_path = get_app_data_dir().join("scenebrowser.db");
        let pool = SqlitePool::connect(&format!("sqlite:{}", db_path.display())).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Self { pool })
    }
    
    pub async fn add_folder(&self, path: &str, recursive: bool) -> Result<i64>;
    pub async fn get_folders(&self) -> Result<Vec<Folder>>;
    pub async fn add_video(&self, video: &VideoInsert) -> Result<i64>;
    pub async fn get_videos(&self, folder_id: Option<i64>, limit: i64, offset: i64) -> Result<Vec<Video>>;
    pub async fn search_videos(&self, query: &str) -> Result<Vec<Video>>;
}
```

#### 3.2 Video Scanner Service (`src-tauri/src/services/video_scanner.rs`)

```rust
use walkdir::WalkDir;
use std::path::{Path, PathBuf};

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "mov", "webm", "m4v"];

pub struct VideoScanner;

impl VideoScanner {
    pub fn scan_directory(root: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
        let walker = if recursive {
            WalkDir::new(root)
        } else {
            WalkDir::new(root).max_depth(1)
        };
        
        let mut videos = Vec::new();
        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            if self.is_video_file(entry.path()) {
                videos.push(entry.path().to_path_buf());
            }
        }
        Ok(videos)
    }
    
    fn is_video_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| VIDEO_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
    }
    
    pub async fn compute_hash(path: &Path) -> Result<String> {
        // SHA256ハッシュ計算
    }
}
```

#### 3.3 Metadata Extractor (`src-tauri/src/services/metadata.rs`)

```rust
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub codec: String,
    pub framerate: f64,
}

pub struct MetadataExtractor;

impl MetadataExtractor {
    pub async fn extract(video_path: &Path) -> Result<VideoMetadata> {
        // ffprobeを使用してJSON形式でメタデータ取得
        let output = Command::new("ffprobe")
            .args(&[
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                "-show_streams",
                video_path.to_str().unwrap(),
            ])
            .output()?;
        
        // JSONパース処理
        let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
        
        // 動画ストリーム抽出・構造体へマッピング
        Ok(VideoMetadata { /* ... */ })
    }
}
```

#### 3.4 Thumbnail Generator (`src-tauri/src/services/thumbnail_gen.rs`)

```rust
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct ThumbnailGenerator;

impl ThumbnailGenerator {
    /// 動画から16枚のサムネイルをタイル状に生成
    pub async fn generate(
        video_path: &Path,
        duration: f64,
        thumbnail_count: usize,
    ) -> Result<PathBuf> {
        let cache_dir = get_thumbnail_cache_dir();
        std::fs::create_dir_all(&cache_dir)?;
        
        // ハッシュベースのファイル名
        let hash = compute_video_hash(video_path)?;
        let output_path = cache_dir.join(format!("{}.jpg", hash));
        
        // 既にキャッシュが存在する場合はスキップ
        if output_path.exists() {
            return Ok(output_path);
        }
        
        // ffmpegで4x4タイル生成
        let interval = duration / thumbnail_count as f64;
        let fps = 1.0 / interval;
        
        Command::new("ffmpeg")
            .args(&[
                "-i", video_path.to_str().unwrap(),
                "-vf", &format!("fps={},scale=320:-1,tile=4x4", fps),
                "-frames:v", "1",
                output_path.to_str().unwrap(),
            ])
            .output()?;
        
        Ok(output_path)
    }
}
```

### 4. Tauri Commands Design

Phase 1に必要なコマンド：

```rust
// src-tauri/src/commands/folder.rs
#[tauri::command]
pub async fn add_folder(
    path: String,
    recursive: bool,
    state: State<'_, AppState>,
) -> Result<i64, String>

#[tauri::command]
pub async fn get_folders(
    state: State<'_, AppState>,
) -> Result<Vec<Folder>, String>

// src-tauri/src/commands/video.rs
#[tauri::command]
pub async fn scan_folder(
    folder_id: i64,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<ScanResult, String>
// 進捗: window.emit("scan_progress", { current, total })

#[tauri::command]
pub async fn get_videos(
    folder_id: Option<i64>,
    limit: i64,
    offset: i64,
    state: State<'_, AppState>,
) -> Result<Vec<Video>, String>

#[tauri::command]
pub async fn search_videos(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<Video>, String>

// src-tauri/src/commands/thumbnail.rs
#[tauri::command]
pub async fn generate_thumbnail(
    video_id: i64,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<String, String>
// サムネイル画像パスを返す
```

### 5. Frontend Implementation

#### 5.1 State Management

Zustandを使用した軽量状態管理：

```typescript
// src/store/videoStore.ts
import { create } from 'zustand';
import { Video } from '../types/video';

interface VideoStore {
    videos: Video[];
    selectedFolder: number | null;
    isLoading: boolean;
    setVideos: (videos: Video[]) => void;
    setSelectedFolder: (folderId: number | null) => void;
    setIsLoading: (loading: boolean) => void;
}

export const useVideoStore = create<VideoStore>((set) => ({
    videos: [],
    selectedFolder: null,
    isLoading: false,
    setVideos: (videos) => set({ videos }),
    setSelectedFolder: (folderId) => set({ selectedFolder: folderId }),
    setIsLoading: (loading) => set({ isLoading: loading }),
}));
```

#### 5.2 Virtual Scrolling Grid

react-windowを使用した高速グリッド表示：

```typescript
// src/components/VideoGrid.tsx
import { FixedSizeGrid } from 'react-window';
import AutoSizer from 'react-virtualized-auto-sizer';
import VideoCard from './VideoCard';

const CARD_WIDTH = 360;
const CARD_HEIGHT = 280;
const GUTTER = 16;

export const VideoGrid = ({ videos }: { videos: Video[] }) => {
    return (
        <AutoSizer>
            {({ height, width }) => {
                const columnCount = Math.floor(width / (CARD_WIDTH + GUTTER));
                const rowCount = Math.ceil(videos.length / columnCount);
                
                return (
                    <FixedSizeGrid
                        columnCount={columnCount}
                        columnWidth={CARD_WIDTH + GUTTER}
                        height={height}
                        rowCount={rowCount}
                        rowHeight={CARD_HEIGHT + GUTTER}
                        width={width}
                    >
                        {({ columnIndex, rowIndex, style }) => {
                            const index = rowIndex * columnCount + columnIndex;
                            const video = videos[index];
                            return video ? (
                                <div style={style}>
                                    <VideoCard video={video} />
                                </div>
                            ) : null;
                        }}
                    </FixedSizeGrid>
                );
            }}
        </AutoSizer>
    );
};
```

#### 5.3 VideoCard Component

```typescript
// src/components/VideoCard.tsx
import { convertFileSrc } from '@tauri-apps/api/core';
import { Video } from '../types/video';

export const VideoCard = ({ video }: { video: Video }) => {
    const thumbnailUrl = convertFileSrc(video.thumbnail_path);
    
    return (
        <div className="video-card">
            <img 
                src={thumbnailUrl} 
                alt={video.filename}
                loading="lazy"
            />
            <div className="video-info">
                <h3>{video.filename}</h3>
                <span>{formatDuration(video.duration)}</span>
                <span>{video.width}x{video.height}</span>
            </div>
        </div>
    );
};
```

### 6. FFmpeg Integration

#### 6.1 Bundling Strategy

macOS用にffmpeg/ffprobeをsidecarとして同梱：

1. `src-tauri/binaries/`配下に配置
2. `tauri.conf.json`で登録
3. ビルド時に自動バンドル

ダウンロード元：
- https://evermeet.cx/ffmpeg/ (macOS static builds)
- または公式ビルドから取得

#### 6.2 Configuration

```json
// src-tauri/tauri.conf.json
{
  "bundle": {
    "externalBin": [
      "binaries/ffmpeg-aarch64-apple-darwin",
      "binaries/ffmpeg-x86_64-apple-darwin",
      "binaries/ffprobe-aarch64-apple-darwin",
      "binaries/ffprobe-x86_64-apple-darwin"
    ]
  }
}
```

### 7. Key Technology Choices

| Component | Technology | Rationale |
|-----------|-----------|-----------|
| Frontend Framework | React | コンポーネント再利用性、豊富なエコシステム |
| State Management | Zustand | 軽量、シンプルAPI、TypeScript完全対応 |
| Virtual Scrolling | react-window | パフォーマンス、10万件対応 |
| Backend | Rust (Tauri) | 高速、メモリ安全、ネイティブバイナリ |
| Database | SQLite + sqlx | ローカル完結、非同期、型安全 |
| Video Processing | ffmpeg/ffprobe | 業界標準、全フォーマット対応 |
| CSS | CSS Modules | スコープ付きスタイル、衝突回避 |

### 8. Implementation Steps

#### Step 1: Project Initialization
- Tauri + React + TypeScript プロジェクト作成
- package.json, tsconfig.json, vite.config.ts 設定
- src-tauri/Cargo.toml に依存関係追加

#### Step 2: Database Layer
- migrations/001_initial_schema.sql 作成
- services/database.rs 実装
- モデル定義（Video, Folder）

#### Step 3: Core Services
- video_scanner.rs 実装（フォルダスキャン）
- metadata.rs 実装（ffprobe統合）
- thumbnail_gen.rs 実装（ffmpeg統合）

#### Step 4: Tauri Commands
- folder.rs コマンド実装
- video.rs コマンド実装
- thumbnail.rs コマンド実装

#### Step 5: Frontend UI
- レイアウト構築（Sidebar + MainArea）
- FolderList コンポーネント
- VideoGrid + VideoCard 実装
- commands.ts（Tauri呼び出しラッパー）

#### Step 6: Integration
- フォルダ登録フロー
- スキャン処理（進捗表示付き）
- サムネイル生成・表示
- 検索機能

#### Step 7: Polish
- エラーハンドリング
- ローディング状態
- キーボードショートカット（Space, 矢印キー）

### 9. Critical Files to Create

**Backend:**
- `src-tauri/Cargo.toml` - Dependencies
- `src-tauri/tauri.conf.json` - App configuration
- `src-tauri/migrations/001_initial_schema.sql` - Database schema
- `src-tauri/src/main.rs` - Entry point
- `src-tauri/src/services/database.rs` - Database layer
- `src-tauri/src/services/video_scanner.rs` - Video scanning
- `src-tauri/src/services/thumbnail_gen.rs` - Thumbnail generation
- `src-tauri/src/commands/video.rs` - Video commands

**Frontend:**
- `package.json` - Dependencies
- `vite.config.ts` - Build config
- `src/main.tsx` - React entry
- `src/App.tsx` - Root component
- `src/components/VideoGrid.tsx` - Grid display
- `src/components/VideoCard.tsx` - Video card
- `src/services/commands.ts` - Tauri wrappers
- `src/types/video.ts` - TypeScript types

### 10. Performance Considerations

1. **Lazy Thumbnail Generation**: 初回表示時のみ生成、以降はキャッシュ利用
2. **Pagination**: 一度に1000件ずつ取得（無限スクロール）
3. **Indexing**: path, hash, created_at にインデックス
4. **Virtual Scrolling**: 可視領域のみレンダリング
5. **Async Processing**: スキャン・サムネイル生成は非同期・並列化

### 11. Verification Plan

Phase 1実装完了後の確認項目：

1. **フォルダ登録**
   - 複数フォルダ登録可能
   - 再帰/非再帰切り替え
   - 外付けHDD対応

2. **動画スキャン**
   - 対応形式（mp4, mkv, avi, mov, webm, m4v）検出
   - メタデータ正確取得（解像度、長さ、コーデック）
   - 進捗表示

3. **サムネイル生成**
   - 16枚タイル生成
   - キャッシュ動作確認
   - 生成速度（1動画あたり1-2秒以内）

4. **一覧表示**
   - グリッド表示
   - 仮想スクロール（1000件以上でテスト）
   - サムネイル遅延ロード

5. **検索**
   - ファイル名検索
   - レスポンス速度（100ms以内）

6. **パフォーマンス**
   - 初回起動3秒以内
   - 1000件スクロール滑らか
   - メモリ使用量監視

## Summary

このプランは、SceneBrowser Phase 1 (MVP) の完全な実装設計です。Tauriの標準アーキテクチャに従い、モジュール化されたサービス層、型安全なデータベース操作、高パフォーマンスなフロントエンドを実現します。10万件の動画を扱える設計になっており、Phase 2（タグ、レーティング）への拡張も容易です。
