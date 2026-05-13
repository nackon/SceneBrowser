# SceneBrowser セットアップガイド

このドキュメントでは、SceneBrowserの開発環境セットアップから実行までの手順を詳しく説明します。

## 目次

1. [前提条件](#前提条件)
2. [環境構築](#環境構築)
3. [プロジェクトのクローンとセットアップ](#プロジェクトのクローンとセットアップ)
4. [開発サーバーの起動](#開発サーバーの起動)
5. [ビルド](#ビルド)
6. [トラブルシューティング](#トラブルシューティング)

---

## 前提条件

### macOS バージョン
- macOS 11.0 (Big Sur) 以降

### 必要なツール
- Rust 1.75+
- Node.js 22+
- FFmpeg 6.0+
- Xcode Command Line Tools

---

## 環境構築

### 1. Xcode Command Line Tools

```bash
xcode-select --install
```

### 2. Homebrew（未インストールの場合）

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### 3. Rust のインストール

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

インストール後、シェルを再起動するか、以下を実行：

```bash
source $HOME/.cargo/env
```

バージョン確認：

```bash
rustc --version
cargo --version
```

### 4. Node.js のインストール

#### Option A: Homebrew を使用

```bash
brew install node@22
```

#### Option B: nvm を使用（推奨）

```bash
# nvm のインストール
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

# シェルを再起動後
nvm install 22
nvm use 22
```

バージョン確認：

```bash
node --version  # v22.x.x 以上
npm --version   # 10.x.x 以上
```

### 5. FFmpeg のインストール

#### Option A: Homebrew を使用（推奨）

```bash
brew install ffmpeg
```

#### Option B: バイナリを直接ダウンロード

```bash
# Intel Mac
curl -O https://evermeet.cx/ffmpeg/ffmpeg-6.0.zip
curl -O https://evermeet.cx/ffmpeg/ffprobe-6.0.zip

# Apple Silicon (M1/M2)
# 同じファイルがユニバーサルバイナリとして動作

unzip ffmpeg-6.0.zip
unzip ffprobe-6.0.zip

# /usr/local/bin に移動（要管理者権限）
sudo mv ffmpeg /usr/local/bin/
sudo mv ffprobe /usr/local/bin/
sudo chmod +x /usr/local/bin/ffmpeg
sudo chmod +x /usr/local/bin/ffprobe
```

バージョン確認：

```bash
ffmpeg -version
ffprobe -version
```

---

## プロジェクトのクローンとセットアップ

### 1. リポジトリのクローン

```bash
git clone https://github.com/nackon/SceneBrowser.git
cd SceneBrowser
```

### 2. Node.js 依存関係のインストール

```bash
npm install
```

これにより以下がインストールされます：
- React + React DOM
- TypeScript
- Vite
- Zustand (状態管理)
- react-window (仮想スクロール)
- Tauri プラグイン

### 3. Rust 依存関係のダウンロード

初回起動時に自動的にダウンロードされますが、事前に確認する場合：

```bash
cd src-tauri
cargo check
cd ..
```

---

## 開発サーバーの起動

### 通常起動

```bash
npm run tauri dev
```

このコマンドは以下を行います：
1. Vite 開発サーバーを起動（ポート 5173）
2. Rust バックエンドをコンパイル
3. Tauri アプリケーションウィンドウを開く

### 初回起動時

初回起動は Rust のコンパイルに時間がかかります（5-10分程度）。  
2回目以降は増分コンパイルにより高速化されます（30秒程度）。

### 開発サーバーが起動したら

1. アプリケーションウィンドウが表示されます
2. 左サイドバーの「+」ボタンをクリックしてフォルダを追加
3. フォルダを追加したら「🔄」ボタンでスキャン
4. 動画のサムネイルが自動生成されます

---

## ビルド

### 開発ビルド（デバッグ情報付き）

```bash
npm run tauri build -- --debug
```

### 本番ビルド

```bash
npm run tauri build
```

ビルド成果物の場所：
- **dmgファイル**: `src-tauri/target/release/bundle/dmg/SceneBrowser_0.1.0_*.dmg`
- **アプリバンドル**: `src-tauri/target/release/bundle/macos/SceneBrowser.app`

### インストール

```bash
# dmg を開いてアプリケーションフォルダにドラッグ
open src-tauri/target/release/bundle/dmg/SceneBrowser_*.dmg
```

---

## データの保存場所

SceneBrowser は以下のディレクトリにデータを保存します：

### macOS

```
~/Library/Application Support/SceneBrowser/
├── scenebrowser.db          # SQLite データベース
└── thumbnails/              # 生成されたサムネイル画像
    └── [hash].jpg
```

### データのクリア

```bash
rm -rf ~/Library/Application\ Support/SceneBrowser/
```

---

## トラブルシューティング

### Rust がインストールされていない

**症状**: `cargo: command not found`

**解決策**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### FFmpeg が見つからない

**症状**: `ffmpeg not found` エラー

**解決策**:
```bash
# インストール確認
which ffmpeg
which ffprobe

# インストールされていない場合
brew install ffmpeg

# PATH の確認
echo $PATH | grep /usr/local/bin
```

### npm install でエラーが出る

**症状**: `EACCES` や権限エラー

**解決策**:
```bash
# npm のグローバルディレクトリを変更
mkdir ~/.npm-global
npm config set prefix '~/.npm-global'
echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.zshrc
source ~/.zshrc

# 再度インストール
npm install
```

### Tauri のビルドが失敗する

**症状**: `error: linking with 'cc' failed`

**解決策**:
```bash
# Xcode Command Line Tools の再インストール
sudo rm -rf /Library/Developer/CommandLineTools
xcode-select --install
```

### ポート 5173 が既に使用されている

**症状**: `Port 5173 is in use`

**解決策**:
```bash
# 使用中のプロセスを確認
lsof -i :5173

# プロセスを終了
kill -9 [PID]

# または別のポートを使用
# vite.config.ts の server.port を変更
```

### データベースがロックされている

**症状**: `database is locked`

**解決策**:
```bash
# アプリを完全に終了
killall SceneBrowser

# データベースファイルを削除（データが消えます）
rm ~/Library/Application\ Support/SceneBrowser/scenebrowser.db
```

### サムネイルが生成されない

**症状**: 動画カードにサムネイルが表示されない

**確認事項**:
1. FFmpeg が正しくインストールされているか確認
   ```bash
   ffmpeg -version
   ```

2. 動画ファイルが対応フォーマットか確認（mp4, mkv, avi, mov, webm, m4v）

3. 動画ファイルが破損していないか確認
   ```bash
   ffprobe /path/to/video.mp4
   ```

4. ディスク容量が十分にあるか確認
   ```bash
   df -h ~/Library/Application\ Support/SceneBrowser/thumbnails/
   ```

### 仮想スクロールが動作しない

**症状**: 動画一覧がスクロールできない

**解決策**:
- ブラウザのコンソールを開いてエラーを確認
- React Developer Tools でコンポーネントツリーを確認
- `npm run tauri dev` を再起動

---

## テストの実行

### Rust ユニットテスト

```bash
cd src-tauri
cargo test
```

### Rust 統合テスト

```bash
cd src-tauri
cargo test --test '*'
```

### 特定のテストのみ実行

```bash
cargo test test_add_and_get_folders
```

### テストの詳細出力

```bash
cargo test -- --nocapture
```

---

## 開発のヒント

### ホットリロード

- **フロントエンド**: ファイル保存時に自動リロード
- **バックエンド**: Rust ファイル保存時に自動再コンパイル＆再起動

### デバッグ

#### フロントエンド

ブラウザの開発者ツールを開く：
- macOS: `Cmd + Option + I`

#### バックエンド

Rust のログ出力：
```rust
println!("Debug: {:?}", variable);
```

または `env_logger` を使用：
```bash
RUST_LOG=debug npm run tauri dev
```

### パフォーマンス最適化

開発ビルドは遅いため、パフォーマンステストは本番ビルドで行ってください：
```bash
npm run tauri build
# ビルドされた .app を実行
open src-tauri/target/release/bundle/macos/SceneBrowser.app
```

---

## 次のステップ

セットアップが完了したら：

1. [API仕様書](design/api-specification.md)を確認
2. [アーキテクチャ設計書](design/architecture.md)を確認
3. [Phase 1実装計画](phase1-implementation-plan.md)を確認
4. [タスク一覧](../TASKS.md)で進捗確認

---

## サポート

問題が発生した場合：
1. このドキュメントのトラブルシューティングを確認
2. [GitHub Issues](https://github.com/nackon/SceneBrowser/issues) を検索
3. 新しいIssueを作成

---

**Happy Coding! 🎬**
