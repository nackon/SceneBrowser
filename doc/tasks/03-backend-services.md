# Task: Backend Core Services Implementation

## Description
動画スキャン、メタデータ抽出、サムネイル生成などのコアサービスを実装します。

## Objectives
- [ ] Database Service の完全実装
- [ ] Video Scanner Service の実装
- [ ] Metadata Extractor の実装
- [ ] Thumbnail Generator の実装
- [ ] Utility Functions の実装

## Steps

### 1. Database Service (`src-tauri/src/services/database.rs`)
実装する主要メソッド:
- `new()` - データベース接続とマイグレーション
- `add_folder()` - フォルダ登録
- `get_folders()` - フォルダ一覧取得
- `add_video()` - 動画登録
- `get_videos()` - 動画一覧取得（ページネーション対応）
- `search_videos()` - 動画検索
- `update_video_thumbnail()` - サムネイルパス更新

### 2. Video Scanner (`src-tauri/src/services/video_scanner.rs`)
機能:
- ディレクトリの再帰的スキャン
- 動画ファイル拡張子の判定 (mp4, mkv, avi, mov, webm, m4v)
- ファイルパスリストの返却

### 3. Metadata Extractor (`src-tauri/src/services/metadata.rs`)
機能:
- ffprobeを使用したメタデータ取得
- JSON形式での出力パース
- VideoMetadata構造体への変換
- エラーハンドリング

### 4. Thumbnail Generator (`src-tauri/src/services/thumbnail_gen.rs`)
機能:
- ffmpegでの4x4タイル生成
- キャッシュディレクトリ管理
- 既存サムネイルのチェック
- ハッシュベースのファイル名生成

### 5. Utility Functions

`src-tauri/src/utils/paths.rs`:
- `get_app_data_dir()` - アプリデータディレクトリ取得
- `get_thumbnail_cache_dir()` - サムネイルキャッシュディレクトリ
- `ensure_dir_exists()` - ディレクトリ作成

`src-tauri/src/utils/hashing.rs`:
- `compute_file_hash()` - SHA256ハッシュ計算

`src-tauri/src/utils/ffmpeg.rs`:
- `check_ffmpeg_available()` - ffmpeg/ffprobe存在確認
- `build_ffprobe_command()` - ffprobeコマンド生成
- `build_ffmpeg_command()` - ffmpegコマンド生成

### 6. Error Handling (`src-tauri/src/error.rs`)
カスタムエラー型の定義:
```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("FFmpeg error: {0}")]
    FFmpeg(String),
    
    #[error("Video not found: {0}")]
    VideoNotFound(String),
}
```

## Acceptance Criteria
- [ ] 全サービスが正常にコンパイルできる
- [ ] 単体テストが通る
- [ ] ffmpeg/ffprobeとの統合が動作する
- [ ] エラーハンドリングが適切に実装されている

## Testing
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_video_scanner_detects_videos() { }
    
    #[test]
    fn test_metadata_extraction() { }
    
    #[test]
    fn test_hash_computation() { }
}
```

## Estimated Time
8 hours

## Dependencies
- Task #02 (Database Schema)

## Labels
- backend
- services
- phase-1
