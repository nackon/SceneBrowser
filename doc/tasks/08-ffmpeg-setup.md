# Task: FFmpeg/FFprobe Setup and Integration

## Description
ffmpeg/ffprobeのダウンロード、配置、Tauriへの統合を行います。

## Objectives
- [ ] ffmpeg/ffprobeバイナリの取得
- [ ] sidecar設定
- [ ] 実行権限の設定
- [ ] システムパスでのフォールバック実装

## Steps

### 1. Download FFmpeg Binaries

#### macOS (Apple Silicon)
```bash
# https://evermeet.cx/ffmpeg/ から取得
cd src-tauri/binaries
curl -O https://evermeet.cx/ffmpeg/getrelease/ffmpeg/zip
curl -O https://evermeet.cx/ffmpeg/getrelease/ffprobe/zip

unzip ffmpeg.zip -d ffmpeg-aarch64-apple-darwin
unzip ffprobe.zip -d ffprobe-aarch64-apple-darwin

mv ffmpeg-aarch64-apple-darwin/ffmpeg ./ffmpeg-aarch64-apple-darwin
mv ffprobe-aarch64-apple-darwin/ffprobe ./ffprobe-aarch64-apple-darwin

rm -rf ffmpeg.zip ffprobe.zip ffmpeg-aarch64-apple-darwin ffprobe-aarch64-apple-darwin
```

#### macOS (Intel)
```bash
# Intel版も同様に取得
# ユニバーサルバイナリを使用する場合は1つでOK
```

### 2. Configure Tauri (`src-tauri/tauri.conf.json`)
```json
{
  "bundle": {
    "externalBin": [
      "binaries/ffmpeg-aarch64-apple-darwin",
      "binaries/ffmpeg-x86_64-apple-darwin",
      "binaries/ffprobe-aarch64-apple-darwin",
      "binaries/ffprobe-x86_64-apple-darwin"
    ]
  },
  "security": {
    "dangerousAllowExternalBin": true
  }
}
```

### 3. Sidecar Utility (`src-tauri/src/utils/ffmpeg.rs`)
```rust
use std::path::PathBuf;
use std::process::Command;

pub enum FFmpegSource {
    Sidecar(PathBuf),
    System(PathBuf),
}

pub fn find_ffmpeg() -> Result<FFmpegSource, String> {
    // Try sidecar first
    if let Ok(sidecar_path) = tauri::api::process::sidecar_path("ffmpeg") {
        if sidecar_path.exists() {
            return Ok(FFmpegSource::Sidecar(sidecar_path));
        }
    }
    
    // Fallback to system PATH
    if let Ok(path) = which::which("ffmpeg") {
        return Ok(FFmpegSource::System(path));
    }
    
    Err("ffmpeg not found".to_string())
}

pub fn find_ffprobe() -> Result<FFmpegSource, String> {
    // Same logic for ffprobe
    if let Ok(sidecar_path) = tauri::api::process::sidecar_path("ffprobe") {
        if sidecar_path.exists() {
            return Ok(FFmpegSource::Sidecar(sidecar_path));
        }
    }
    
    if let Ok(path) = which::which("ffprobe") {
        return Ok(FFmpegSource::System(path));
    }
    
    Err("ffprobe not found".to_string())
}

pub fn check_ffmpeg_availability() -> Result<(), String> {
    find_ffmpeg()?;
    find_ffprobe()?;
    Ok(())
}
```

### 4. Update Metadata Service
`src-tauri/src/services/metadata.rs`:
```rust
use crate::utils::ffmpeg::find_ffprobe;

pub async fn extract(video_path: &Path) -> Result<VideoMetadata> {
    let ffprobe_path = match find_ffprobe()? {
        FFmpegSource::Sidecar(path) | FFmpegSource::System(path) => path,
    };
    
    let output = Command::new(&ffprobe_path)
        .args(&[
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            "-show_streams",
            video_path.to_str().unwrap(),
        ])
        .output()?;
    
    // Parse output...
}
```

### 5. Update Thumbnail Service
`src-tauri/src/services/thumbnail_gen.rs`:
```rust
use crate::utils::ffmpeg::find_ffmpeg;

pub async fn generate(video_path: &Path, duration: f64) -> Result<PathBuf> {
    let ffmpeg_path = match find_ffmpeg()? {
        FFmpegSource::Sidecar(path) | FFmpegSource::System(path) => path,
    };
    
    Command::new(&ffmpeg_path)
        .args(&[
            "-i", video_path.to_str().unwrap(),
            "-vf", "fps=1/60,scale=320:-1,tile=4x4",
            "-frames:v", "1",
            output_path.to_str().unwrap(),
        ])
        .output()?;
    
    // Check result...
}
```

### 6. Add Dependency
`src-tauri/Cargo.toml`:
```toml
[dependencies]
which = "6.0"
```

### 7. Verify Installation Command
```rust
#[tauri::command]
pub async fn check_ffmpeg() -> Result<FFmpegStatus, String> {
    let ffmpeg = find_ffmpeg().is_ok();
    let ffprobe = find_ffprobe().is_ok();
    
    Ok(FFmpegStatus {
        ffmpeg_available: ffmpeg,
        ffprobe_available: ffprobe,
        ffmpeg_version: get_version("ffmpeg")?,
        ffprobe_version: get_version("ffprobe")?,
    })
}
```

### 8. .gitignore Update
```
# FFmpeg binaries
src-tauri/binaries/ffmpeg*
src-tauri/binaries/ffprobe*
```

### 9. Download Script
`scripts/download-ffmpeg.sh`:
```bash
#!/bin/bash
set -e

ARCH=$(uname -m)
OS=$(uname -s)

if [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "arm64" ]; then
        TARGET="aarch64-apple-darwin"
    else
        TARGET="x86_64-apple-darwin"
    fi
    
    echo "Downloading ffmpeg for $TARGET..."
    # Download and extract
fi
```

## Acceptance Criteria
- [ ] ffmpeg/ffprobeがバンドルされている
- [ ] sidecarとして実行できる
- [ ] システムインストール版へのフォールバックが動作する
- [ ] バージョン確認コマンドが動作する

## Testing
```bash
# ビルドして確認
npm run tauri build

# アプリ起動
open src-tauri/target/release/bundle/macos/SceneBrowser.app

# FFmpegが正しくバンドルされているか確認
ls -la src-tauri/target/release/bundle/macos/SceneBrowser.app/Contents/MacOS/
```

## Estimated Time
3 hours

## Dependencies
- Task #01 (Project Setup)

## Labels
- setup
- ffmpeg
- phase-1
