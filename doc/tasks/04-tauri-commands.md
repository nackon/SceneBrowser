# Task: Tauri Commands Implementation

## Description
フロントエンドから呼び出せるTauriコマンドを実装します。

## Objectives
- [ ] Folder Commands の実装
- [ ] Video Commands の実装
- [ ] Thumbnail Commands の実装
- [ ] AppState の設定
- [ ] コマンドの登録

## Steps

### 1. Folder Commands (`src-tauri/src/commands/folder.rs`)
```rust
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

#[tauri::command]
pub async fn remove_folder(
    folder_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String>
```

### 2. Video Commands (`src-tauri/src/commands/video.rs`)
```rust
#[tauri::command]
pub async fn scan_folder(
    folder_id: i64,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<ScanResult, String>

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

#[tauri::command]
pub async fn get_video_by_id(
    video_id: i64,
    state: State<'_, AppState>,
) -> Result<Video, String>
```

### 3. Thumbnail Commands (`src-tauri/src/commands/thumbnail.rs`)
```rust
#[tauri::command]
pub async fn generate_thumbnail(
    video_id: i64,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<String, String>

#[tauri::command]
pub async fn batch_generate_thumbnails(
    video_ids: Vec<i64>,
    window: tauri::Window,
    state: State<'_, AppState>,
) -> Result<BatchResult, String>
```

### 4. AppState Setup (`src-tauri/src/main.rs`)
```rust
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub db: Arc<Mutex<Database>>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let db = Database::new().await?;
            app.manage(AppState {
                db: Arc::new(Mutex::new(db)),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::folder::add_folder,
            commands::folder::get_folders,
            commands::folder::remove_folder,
            commands::video::scan_folder,
            commands::video::get_videos,
            commands::video::search_videos,
            commands::video::get_video_by_id,
            commands::thumbnail::generate_thumbnail,
            commands::thumbnail::batch_generate_thumbnails,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 5. Progress Events
スキャンやサムネイル生成の進捗をフロントエンドに送信:
```rust
#[derive(Clone, Serialize)]
struct ScanProgress {
    current: usize,
    total: usize,
    current_file: String,
}

window.emit("scan_progress", ScanProgress {
    current: idx,
    total: videos.len(),
    current_file: path.to_string(),
})?;
```

## Acceptance Criteria
- [ ] 全コマンドが正常にコンパイルできる
- [ ] フロントエンドから呼び出せる
- [ ] エラーが適切にフロントエンドに返される
- [ ] 進捗イベントが正しく発火する

## Testing
```bash
# Tauri devモードで起動してブラウザコンソールから
invoke('get_folders')
invoke('add_folder', { path: '/test', recursive: true })
```

## Estimated Time
5 hours

## Dependencies
- Task #03 (Backend Services)

## Labels
- backend
- commands
- phase-1
