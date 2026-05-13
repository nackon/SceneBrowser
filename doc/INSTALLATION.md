# SceneBrowser インストールガイド

## 安定版リリースのインストール

1. [GitHub Releases](https://github.com/nackon/SceneBrowser/releases/latest) から最新版をダウンロード
2. DMGファイルを開く
3. SceneBrowser.appをApplicationsフォルダにドラッグ
4. アプリを起動

安定版は公式に署名されているため、Gatekeeperの警告は表示されません。

## 開発ビルドのインストール

開発ビルドはad-hoc署名されているため、追加の手順が必要です。

### 方法1: 右クリックから開く（推奨）

1. DMGをマウントし、SceneBrowser.appをApplicationsにコピー
2. Finderで `/Applications/SceneBrowser.app` を探す
3. **右クリック** → **開く** を選択
4. 「開発元を確認できません」の警告が表示されたら **開く** をクリック
5. 以降は通常通りダブルクリックで起動可能

### 方法2: ターミナルから quarantine 属性を削除

```bash
# アプリをApplicationsにコピー後
xattr -cr /Applications/SceneBrowser.app

# または、DMGファイル自体の属性を削除してからインストール
xattr -cr ~/Downloads/SceneBrowser-dev.dmg
```

### 方法3: システム設定から許可（macOS Ventura以降）

1. アプリを開こうとして警告が出る
2. システム設定 → プライバシーとセキュリティ を開く
3. 「"SceneBrowser"は開発元を確認できないため、使用がブロックされました」の横の **このまま開く** をクリック

## トラブルシューティング

### 「マルウェアが含まれていないことを検証できませんでした」エラー

これはmacOSのGatekeeperによる警告で、開発ビルドがAppleの公証（Notarization）を受けていないために表示されます。**アプリは安全です。**

**解決方法1: ターミナルコマンド（最も確実）:**

```bash
# アプリをApplicationsにコピー後、ターミナルで実行
xattr -cr /Applications/SceneBrowser.app

# その後、Finderでダブルクリックして起動
```

**解決方法2: 右クリックから開く:**

1. Finderで `/Applications/SceneBrowser.app` を探す
2. **右クリック** → **開く** を選択
3. 警告ダイアログで **開く** をクリック

**解決方法3: システム設定から許可（macOS Ventura以降）:**

1. アプリを開こうとして警告が出る
2. システム設定 → プライバシーとセキュリティ を開く
3. **このまま開く** をクリック

### 「SceneBrowserは壊れているため開けません」エラー

これもGatekeeperによるもので、アプリが壊れているわけではありません。

**解決方法:**

```bash
# quarantine属性を削除
xattr -cr /Applications/SceneBrowser.app

# または、すべての拡張属性を確認
xattr -l /Applications/SceneBrowser.app

# 特定の属性だけ削除
xattr -d com.apple.quarantine /Applications/SceneBrowser.app
```

### 「開発元が未確認のため開けません」エラー

開発ビルドは個人の開発者証明書で署名されていないため、この警告が出ます。

**解決方法:**
1. 右クリック → 開く（初回のみ）
2. または上記の `xattr` コマンドを使用

### DMGがマウントできない

```bash
# DMGファイルの quarantine 属性を削除
xattr -cr ~/Downloads/SceneBrowser-dev.dmg

# その後、DMGをダブルクリック
```

## セキュリティについて

開発ビルドはad-hoc署名（`-`署名）されています。これは：
- Apple Developer IDで署名されていない
- Notarization（公証）されていない
- 開発・テスト目的のビルド

**本番環境では安定版リリースの使用を推奨します。**

安定版リリース（git tagからビルド）は将来的に正式な署名を追加する予定です。

## FFmpegのインストール

SceneBrowserはFFmpegが必要です：

```bash
brew install ffmpeg
```

FFmpegがインストールされていない場合、アプリ起動時にエラーが表示されます。
