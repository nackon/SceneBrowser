# Task: Integration and Testing

## Description
フロントエンドとバックエンドの統合、エンドツーエンドテスト、バグ修正を行います。

## Objectives
- [ ] フォルダ登録フローの動作確認
- [ ] 動画スキャン機能の動作確認
- [ ] サムネイル生成の動作確認
- [ ] UI/UX改善
- [ ] パフォーマンステスト
- [ ] バグ修正

## Steps

### 1. End-to-End Testing Flow
```
1. アプリ起動
2. フォルダ追加
3. フォルダスキャン実行
4. 動画一覧表示確認
5. サムネイル表示確認
6. 検索機能確認
7. 仮想スクロール動作確認
```

### 2. Test Scenarios

#### シナリオ1: 初回セットアップ
- [ ] アプリを初めて起動
- [ ] データベースが自動作成される
- [ ] キャッシュディレクトリが作成される

#### シナリオ2: フォルダ登録とスキャン
- [ ] 動画が含まれるフォルダを登録
- [ ] スキャンを実行
- [ ] 進捗が表示される
- [ ] 動画がデータベースに登録される
- [ ] メタデータが正しく取得される

#### シナリオ3: サムネイル生成
- [ ] サムネイル未生成の動画を表示
- [ ] バックグラウンドでサムネイル生成
- [ ] 生成完了後に表示更新
- [ ] 次回起動時はキャッシュから読み込み

#### シナリオ4: 大量データ
- [ ] 1000件以上の動画を登録
- [ ] スクロールが滑らか
- [ ] メモリ使用量が許容範囲内
- [ ] 検索が高速

#### シナリオ5: エラーハンドリング
- [ ] 存在しないフォルダを登録しようとする
- [ ] 破損した動画ファイル
- [ ] ディスク容量不足
- [ ] ffmpeg/ffprobeが見つからない

### 3. Performance Testing

#### メトリクス計測
```rust
// バックエンド
use std::time::Instant;

let start = Instant::now();
// 処理
let duration = start.elapsed();
println!("Time: {:?}", duration);
```

#### 目標値
- 初回起動: 3秒以内
- サムネイル表示: 100ms以内
- 1000件スクロール: 60fps維持
- 動画スキャン: 1000件/分以上

### 4. Bug Fixes Checklist
- [ ] メモリリーク確認
- [ ] ファイルハンドルのクローズ
- [ ] エラーメッセージの改善
- [ ] UI不具合の修正
- [ ] データ不整合の修正

### 5. User Experience Improvements
- [ ] ローディング状態の表示改善
- [ ] エラーメッセージの表示
- [ ] プログレスバーの実装
- [ ] キーボードショートカット（将来）
- [ ] ダークモード対応（将来）

### 6. Documentation
- [ ] READMEの更新
- [ ] ビルド手順の記載
- [ ] 使用方法の記載
- [ ] トラブルシューティング

## Acceptance Criteria
- [ ] 全ての機能が正常に動作する
- [ ] パフォーマンス目標を達成している
- [ ] 既知のバグがない
- [ ] ドキュメントが整備されている

## Testing Checklist

### Functional Testing
- [ ] フォルダ追加
- [ ] フォルダ削除
- [ ] 動画スキャン
- [ ] 動画一覧表示
- [ ] サムネイル生成
- [ ] サムネイル表示
- [ ] 検索機能
- [ ] ページネーション

### Non-Functional Testing
- [ ] パフォーマンス
- [ ] メモリ使用量
- [ ] ディスク使用量
- [ ] CPU使用率
- [ ] 起動時間

### Cross-Platform Testing (macOS)
- [ ] macOS Monterey (12.x)
- [ ] macOS Ventura (13.x)
- [ ] macOS Sonoma (14.x)
- [ ] Apple Silicon (M1/M2)
- [ ] Intel Mac

## Estimated Time
6 hours

## Dependencies
- Task #06 (Frontend UI)

## Labels
- testing
- integration
- phase-1
