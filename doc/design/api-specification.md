# SceneBrowser API Specification

## Overview

This document defines all Tauri commands (backend API) and their TypeScript wrappers (frontend API).

## Table of Contents

1. [Folder Commands](#1-folder-commands)
2. [Video Commands](#2-video-commands)
3. [Thumbnail Commands](#3-thumbnail-commands)
4. [System Commands](#4-system-commands)
5. [Data Types](#5-data-types)
6. [Error Codes](#6-error-codes)
7. [Events](#7-events)

---

## 1. Folder Commands

### 1.1 `add_folder`

登録フォルダを追加します。

**Rust Signature:**
```rust
#[tauri::command]
pub async fn add_folder(
    path: String,
    recursive: bool,
    state: State<'_, AppState>,
) -> Result<i64, String>
```

**TypeScript Wrapper:**
```typescript
async function addFolder(path: string, recursive: boolean): Promise<number>
```

**Parameters:**
| Name | Type | Description |
|------|------|-------------|
| path | string | フォルダの絶対パス |
| recursive | boolean | 再帰的にスキャンするか |

**Returns:**
- Success: `number` - 作成されたフォルダのID
- Error: `string` - エラーメッセージ

**Errors:**
- `"Folder already exists"` - 既に登録済み
- `"Invalid path"` - パスが存在しない
- `"Database error: ..."` - データベースエラー

**Example:**
```typescript
try {
  const folderId = await addFolder("/Users/user/Videos", true);
  console.log(`Folder added with ID: ${folderId}`);
} catch (error) {
  console.error("Failed to add folder:", error);
}
```

---

### 1.2 `get_folders`

登録済みフォルダ一覧を取得します。

**Rust Signature:**
```rust
#[tauri::command]
pub async fn get_folders(
    state: State<'_, AppState>,
) -> Result<Vec<Folder>, String>
```

**TypeScript Wrapper:**
```typescript
async function getFolders(): Promise<Folder[]>
```

**Parameters:** None

**Returns:**
- Success: `Folder[]` - フォルダ一覧
- Error: `string` - エラーメッセージ

**Example:**
```typescript
const folders = await getFolders();
folders.forEach(folder => {
  console.log(folder.path, folder.recursive);
});
```

---

### 1.3 `remove_folder`

登録フォルダを削除します（関連動画も CASCADE DELETE）。

**Rust Signature:**
```rust
#[tauri::command]
pub async fn remove_folder(
    folder_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String>
```

**TypeScript Wrapper:**
```typescript
async function removeFolder(folderId: number): Promise<void>
```

**Parameters:**
| Name | Type | Description |
|------|------|-------------|
| folderId | number | 削除するフォルダのID |

**Returns:**
- Success: `void`
- Error: `string` - エラーメッセージ

**Errors:**
- `"Folder not found"` - フォルダが存在しない

**Example:**
```typescript
await removeFolder(1);
```

---

## 2. Video Commands

### 2.1 `scan_folder`

指定フォルダ内の動画をスキャンし、データベースに登録します。

**Rust Signature:**
```rust
#[tauri::command]
pub async fn scan_folder(
    folder_id: i64,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<ScanResult, String>
```

**TypeScript Wrapper:**
```typescript
async function scanFolder(
  folderId: number,
  onProgress?: (progress: ScanProgress) => void
): Promise<ScanResult>
```

**Parameters:**
| Name | Type | Description |
|------|------|-------------|
| folderId | number | スキャンするフォルダのID |
| onProgress | function | 進捗コールバック（オプション） |

**Returns:**
- Success: `ScanResult`
- Error: `string` - エラーメッセージ

**Progress Events:**
イベント名: `scan_progress`
Payload: `ScanProgress`

**Example:**
```typescript
const result = await scanFolder(1, (progress) => {
  console.log(`${progress.current}/${progress.total}: ${progress.current_file}`);
});

console.log(`Found: ${result.videos_found}, Added: ${result.videos_added}`);
```

---

### 2.2 `get_videos`

動画一覧を取得します（ページネーション対応）。

**Rust Signature:**
```rust
#[tauri::command]
pub async fn get_videos(
    folder_id: Option<i64>,
    limit: i64,
    offset: i64,
    state: State<'_, AppState>,
) -> Result<Vec<Video>, String>
```

**TypeScript Wrapper:**
```typescript
async function getVideos(
  folderId: number | null,
  limit: number,
  offset: number
): Promise<Video[]>
```

**Parameters:**
| Name | Type | Description |
|------|------|-------------|
| folderId | number \| null | フォルダでフィルタ（nullは全件） |
| limit | number | 取得件数 |
| offset | number | オフセット |

**Returns:**
- Success: `Video[]` - 動画一覧
- Error: `string` - エラーメッセージ

**Example:**
```typescript
// 最初の100件を取得
const videos = await getVideos(null, 100, 0);

// 次の100件を取得
const moreVideos = await getVideos(null, 100, 100);

// 特定フォルダの動画のみ
const folderVideos = await getVideos(1, 100, 0);
```

---

### 2.3 `search_videos`

動画を検索します（ファイル名、パスで部分一致検索）。

**Rust Signature:**
```rust
#[tauri::command]
pub async fn search_videos(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<Video>, String>
```

**TypeScript Wrapper:**
```typescript
async function searchVideos(query: string): Promise<Video[]>
```

**Parameters:**
| Name | Type | Description |
|------|------|-------------|
| query | string | 検索クエリ |

**Returns:**
- Success: `Video[]` - 検索結果
- Error: `string` - エラーメッセージ

**Search Strategy:**
- ファイル名に対する部分一致（LIKE '%query%'）
- パスに対する部分一致
- 大文字小文字を区別しない（COLLATE NOCASE）

**Example:**
```typescript
const results = await searchVideos("vacation");
console.log(`Found ${results.length} videos`);
```

---

### 2.4 `get_video_by_id`

IDで特定の動画を取得します。

**Rust Signature:**
```rust
#[tauri::command]
pub async fn get_video_by_id(
    video_id: i64,
    state: State<'_, AppState>,
) -> Result<Video, String>
```

**TypeScript Wrapper:**
```typescript
async function getVideoById(videoId: number): Promise<Video>
```

**Parameters:**
| Name | Type | Description |
|------|------|-------------|
| videoId | number | 動画ID |

**Returns:**
- Success: `Video` - 動画オブジェクト
- Error: `string` - エラーメッセージ

**Errors:**
- `"Video not found"` - 動画が存在しない

**Example:**
```typescript
const video = await getVideoById(123);
console.log(video.filename, video.duration);
```

---

## 3. Thumbnail Commands

### 3.1 `generate_thumbnail`

動画のサムネイルを生成します。

**Rust Signature:**
```rust
#[tauri::command]
pub async fn generate_thumbnail(
    video_id: i64,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<String, String>
```

**TypeScript Wrapper:**
```typescript
async function generateThumbnail(videoId: number): Promise<string>
```

**Parameters:**
| Name | Type | Description |
|------|------|-------------|
| videoId | number | 動画ID |

**Returns:**
- Success: `string` - サムネイル画像の絶対パス
- Error: `string` - エラーメッセージ

**Errors:**
- `"Video not found"` - 動画が存在しない
- `"FFmpeg not found"` - FFmpegが見つからない
- `"Thumbnail generation failed: ..."` - 生成失敗

**Behavior:**
- 既にサムネイルが存在する場合は再生成せずパスを返す
- 16枚のサムネイルを4x4タイルで生成
- ハッシュベースのファイル名でキャッシュディレクトリに保存

**Example:**
```typescript
try {
  const thumbnailPath = await generateThumbnail(123);
  console.log(`Thumbnail: ${thumbnailPath}`);
} catch (error) {
  console.error("Thumbnail generation failed:", error);
}
```

---

### 3.2 `batch_generate_thumbnails`

複数動画のサムネイルをバッチ生成します。

**Rust Signature:**
```rust
#[tauri::command]
pub async fn batch_generate_thumbnails(
    video_ids: Vec<i64>,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<BatchResult, String>
```

**TypeScript Wrapper:**
```typescript
async function batchGenerateThumbnails(
  videoIds: number[],
  onProgress?: (progress: ThumbnailProgress) => void
): Promise<BatchResult>
```

**Parameters:**
| Name | Type | Description |
|------|------|-------------|
| videoIds | number[] | 動画IDの配列 |
| onProgress | function | 進捗コールバック（オプション） |

**Returns:**
- Success: `BatchResult`
- Error: `string` - エラーメッセージ

**Progress Events:**
イベント名: `thumbnail_progress`
Payload: `ThumbnailProgress`

**Example:**
```typescript
const result = await batchGenerateThumbnails([1, 2, 3, 4, 5], (progress) => {
  console.log(`Generated ${progress.completed}/${progress.total}`);
});

console.log(`Success: ${result.success_count}, Failed: ${result.failed_count}`);
```

---

## 4. System Commands

### 4.1 `check_ffmpeg`

FFmpeg/FFprobeの利用可能性を確認します。

**Rust Signature:**
```rust
#[tauri::command]
pub async fn check_ffmpeg() -> Result<FFmpegStatus, String>
```

**TypeScript Wrapper:**
```typescript
async function checkFFmpeg(): Promise<FFmpegStatus>
```

**Parameters:** None

**Returns:**
- Success: `FFmpegStatus`
- Error: `string` - エラーメッセージ

**Example:**
```typescript
const status = await checkFFmpeg();
if (status.ffmpeg_available && status.ffprobe_available) {
  console.log("FFmpeg is ready:", status.ffmpeg_version);
} else {
  console.error("FFmpeg not found");
}
```

---

## 5. Data Types

### 5.1 Folder

```typescript
interface Folder {
  id: number;
  path: string;
  recursive: boolean;
  created_at: string;  // ISO 8601 format
  updated_at: string;  // ISO 8601 format
}
```

**Rust:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Folder {
    pub id: i64,
    pub path: String,
    pub recursive: bool,
    pub created_at: String,
    pub updated_at: String,
}
```

---

### 5.2 Video

```typescript
interface Video {
  id: number;
  folder_id: number;
  path: string;
  filename: string;
  hash: string;
  duration: number;           // seconds
  width: number;              // pixels
  height: number;             // pixels
  size: number;               // bytes
  codec: string | null;
  framerate: number | null;   // fps
  thumbnail_path: string | null;
  thumbnail_count: number;
  rating: number;             // 0-5
  created_at: string;         // ISO 8601
  updated_at: string;         // ISO 8601
  scanned_at: string;         // ISO 8601
}
```

**Rust:**
```rust
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

---

### 5.3 ScanResult

```typescript
interface ScanResult {
  videos_found: number;    // Total videos found in directory
  videos_added: number;    // Newly added videos
  videos_updated: number;  // Updated existing videos
  errors: string[];        // List of errors encountered
}
```

**Rust:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub videos_found: usize,
    pub videos_added: usize,
    pub videos_updated: usize,
    pub errors: Vec<String>,
}
```

---

### 5.4 ScanProgress

```typescript
interface ScanProgress {
  current: number;       // Current item number
  total: number;         // Total items
  current_file: string;  // Currently processing file
}
```

**Rust:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub current: usize,
    pub total: usize,
    pub current_file: String,
}
```

---

### 5.5 BatchResult

```typescript
interface BatchResult {
  success_count: number;
  failed_count: number;
  failed_ids: number[];
}
```

**Rust:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    pub success_count: usize,
    pub failed_count: usize,
    pub failed_ids: Vec<i64>,
}
```

---

### 5.6 ThumbnailProgress

```typescript
interface ThumbnailProgress {
  completed: number;
  total: number;
  current_video_id: number;
}
```

**Rust:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThumbnailProgress {
    pub completed: usize,
    pub total: usize,
    pub current_video_id: i64,
}
```

---

### 5.7 FFmpegStatus

```typescript
interface FFmpegStatus {
  ffmpeg_available: boolean;
  ffprobe_available: boolean;
  ffmpeg_version: string | null;
  ffprobe_version: string | null;
}
```

**Rust:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FFmpegStatus {
    pub ffmpeg_available: bool,
    pub ffprobe_available: bool,
    pub ffmpeg_version: Option<String>,
    pub ffprobe_version: Option<String>,
}
```

---

## 6. Error Codes

### 6.1 Error Messages

| Error Message | Description | HTTP Equivalent |
|---------------|-------------|-----------------|
| `"Folder already exists"` | フォルダが既に登録済み | 409 Conflict |
| `"Folder not found"` | フォルダが存在しない | 404 Not Found |
| `"Video not found"` | 動画が存在しない | 404 Not Found |
| `"Invalid path"` | パスが無効または存在しない | 400 Bad Request |
| `"Database error: ..."` | データベース操作エラー | 500 Internal Server Error |
| `"FFmpeg not found"` | FFmpegが見つからない | 503 Service Unavailable |
| `"FFmpeg execution failed: ..."` | FFmpeg実行エラー | 500 Internal Server Error |
| `"Invalid video file: ..."` | 動画ファイルが不正 | 422 Unprocessable Entity |

### 6.2 Error Handling Pattern

**Frontend:**
```typescript
try {
  const result = await someCommand();
  // Handle success
} catch (error) {
  if (typeof error === 'string') {
    // Parse error message
    if (error.includes("not found")) {
      // Handle not found
    } else if (error.includes("Database error")) {
      // Handle database error
    } else {
      // Generic error
    }
  }
}
```

**Backend:**
```rust
match some_operation() {
    Ok(result) => Ok(result),
    Err(e) => Err(format!("Operation failed: {}", e)),
}
```

---

## 7. Events

### 7.1 Event Types

| Event Name | Payload Type | Description |
|------------|--------------|-------------|
| `scan_progress` | `ScanProgress` | スキャン進捗 |
| `thumbnail_progress` | `ThumbnailProgress` | サムネイル生成進捗 |

### 7.2 Listening to Events

**TypeScript:**
```typescript
import { listen } from '@tauri-apps/api/event';

// Scan progress
const unlisten = await listen<ScanProgress>('scan_progress', (event) => {
  console.log(`Progress: ${event.payload.current}/${event.payload.total}`);
  console.log(`File: ${event.payload.current_file}`);
});

// Don't forget to unlisten when done
unlisten();
```

**Rust (Emitting):**
```rust
window.emit("scan_progress", ScanProgress {
    current: idx,
    total: total_count,
    current_file: path.to_string(),
})?;
```

---

## 8. API Usage Examples

### 8.1 Complete Workflow: Add Folder and Scan

```typescript
async function addAndScanFolder(path: string) {
  try {
    // 1. Add folder
    const folderId = await addFolder(path, true);
    console.log(`Added folder with ID: ${folderId}`);
    
    // 2. Scan folder with progress
    const result = await scanFolder(folderId, (progress) => {
      const percentage = (progress.current / progress.total) * 100;
      console.log(`Scanning: ${percentage.toFixed(1)}%`);
    });
    
    console.log(`Scan complete!`);
    console.log(`Found: ${result.videos_found}`);
    console.log(`Added: ${result.videos_added}`);
    console.log(`Updated: ${result.videos_updated}`);
    
    // 3. Load videos
    const videos = await getVideos(folderId, 100, 0);
    console.log(`Loaded ${videos.length} videos`);
    
    // 4. Generate thumbnails (first 10)
    const videoIds = videos.slice(0, 10).map(v => v.id);
    await batchGenerateThumbnails(videoIds, (progress) => {
      console.log(`Thumbnails: ${progress.completed}/${progress.total}`);
    });
    
  } catch (error) {
    console.error("Workflow failed:", error);
  }
}
```

### 8.2 Search and Display

```typescript
async function searchAndDisplay(query: string) {
  try {
    const videos = await searchVideos(query);
    
    for (const video of videos) {
      console.log(`${video.filename} (${formatDuration(video.duration)})`);
      
      // Generate thumbnail if not exists
      if (!video.thumbnail_path) {
        const thumbnailPath = await generateThumbnail(video.id);
        console.log(`  Thumbnail: ${thumbnailPath}`);
      }
    }
  } catch (error) {
    console.error("Search failed:", error);
  }
}
```

---

## 9. Rate Limiting and Performance

### 9.1 Recommendations

- **Pagination**: Always use limit/offset for large result sets
- **Debouncing**: Debounce search input (300-500ms)
- **Batch Operations**: Use batch commands for multiple items
- **Lazy Loading**: Generate thumbnails on-demand, not all at once
- **Caching**: Cache results on frontend where appropriate

### 9.2 Limits

| Operation | Recommended Limit |
|-----------|------------------|
| `get_videos` limit | 100-1000 per request |
| `search_videos` | No explicit limit, but DB indexed |
| `batch_generate_thumbnails` | 100 videos max per batch |

---

## 10. Versioning

Current API Version: **v0.1.0** (Phase 1)

### 10.1 Future API Changes (Phase 2)

Planned additions:
- Tag management commands
- Rating commands
- Advanced search with filters
- Duplicate detection commands

---

## Summary

This API provides a complete interface for:
- ✅ Folder management
- ✅ Video scanning and metadata extraction
- ✅ Thumbnail generation
- ✅ Search and pagination
- ✅ Progress tracking via events
- ✅ Error handling

All commands are async and type-safe on both Rust and TypeScript sides.
