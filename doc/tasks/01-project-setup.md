# Task: Project Setup and Initialization

## Description
Tauri + React + TypeScript プロジェクトの初期セットアップを行います。

## Objectives
- [ ] Tauriプロジェクトの初期化
- [ ] npm依存関係のインストール
- [ ] Rust依存関係の設定
- [ ] プロジェクト構成の確認

## Steps

### 1. Tauri Project Initialization
```bash
npm create tauri-app@latest -- \
  --yes \
  --manager npm \
  --frontend-template react-ts \
  --app-name scenebrowser \
  --window-title SceneBrowser \
  --identifier com.scenebrowser.app
```

### 2. Frontend Dependencies
```bash
npm install zustand react-window react-virtualized-auto-sizer
npm install -D @types/react-window
```

### 3. Rust Dependencies
`src-tauri/Cargo.toml` に以下を追加:
```toml
[dependencies]
tauri = { version = "2.0", features = ["protocol-asset"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }
tokio = { version = "1", features = ["full"] }
walkdir = "2"
sha2 = "0.10"
hex = "0.4"
```

### 4. Directory Structure Creation
```bash
# Backend directories
mkdir -p src-tauri/src/commands
mkdir -p src-tauri/src/models
mkdir -p src-tauri/src/services
mkdir -p src-tauri/src/utils
mkdir -p src-tauri/migrations
mkdir -p src-tauri/binaries

# Frontend directories
mkdir -p src/components
mkdir -p src/services
mkdir -p src/types
mkdir -p src/hooks
mkdir -p src/store
```

## Acceptance Criteria
- [ ] プロジェクトが正常にビルドできる (`npm run tauri dev`)
- [ ] 全ての依存関係がインストールされている
- [ ] ディレクトリ構造が完成している
- [ ] 開発サーバーが起動できる

## Estimated Time
2 hours

## Dependencies
None

## Labels
- setup
- phase-1
