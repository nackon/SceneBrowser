# お気に入り機能仕様書

## 概要
動画をお気に入りとしてマークし、お気に入りの動画のみをフィルタリング表示できる機能を追加します。

## 機能要件

### 1. お気に入りマーク機能
- 各動画カードに「お気に入りボタン」（★アイコン）を表示
- クリックでお気に入り状態をトグル
  - お気に入り: ★（塗りつぶし、黄色）
  - 非お気に入り: ☆（白抜き、グレー）
- お気に入り状態はデータベースに永続化

### 2. お気に入りフィルター機能
- ビデオリスト上部にフィルターボタンを配置
- フィルター状態:
  - **All Videos**: 全ての動画を表示（デフォルト）
  - **Favorites Only**: お気に入りの動画のみを表示
- 現在のフィルター状態を視覚的に表示
- フィルター状態はセッション間で保持（localStorage）

### 3. UI/UX要件
- お気に入りボタンはホバー時に表示が強調
- お気に入り状態の変更は即座に反映
- お気に入りの数をフィルターボタンに表示（例: "Favorites (5)"）
- アニメーションでスムーズな状態遷移

## データベース設計

### 既存テーブルの拡張
`videos` テーブルに以下のカラムを追加:
- `is_favorite` INTEGER NOT NULL DEFAULT 0 (0: 非お気に入り, 1: お気に入り)

### マイグレーション
新しいマイグレーションファイル `003_add_favorites.sql` を作成:
```sql
-- Add is_favorite column to videos table
ALTER TABLE videos ADD COLUMN is_favorite INTEGER NOT NULL DEFAULT 0;

-- Create index for faster favorite filtering
CREATE INDEX idx_videos_favorite ON videos(is_favorite);
```

## API設計

### Tauri Commands

#### 1. `toggle_favorite`
動画のお気に入り状態をトグル

**入力:**
```typescript
{
  videoId: number
}
```

**出力:**
```typescript
{
  is_favorite: boolean
}
```

#### 2. `get_favorite_videos`
お気に入りの動画のみを取得

**入力:**
```typescript
{
  folderId: number,
  limit: number,
  offset: number
}
```

**出力:**
```typescript
Video[]
```

#### 3. `get_favorite_count`
お気に入りの動画数を取得

**入力:**
```typescript
{
  folderId: number
}
```

**出力:**
```typescript
number
```

## フロントエンド設計

### 1. コンポーネント

#### VideoCard コンポーネント拡張
- お気に入りボタンを右上に配置
- クリックイベントで `toggle_favorite` を呼び出し
- アニメーション付きで状態変更を表示

#### FilterBar コンポーネント（新規）
- フィルターボタンを表示
- 現在のフィルター状態を管理
- お気に入り数を表示

### 2. 状態管理
Zustand storeに以下を追加:
```typescript
interface VideoStore {
  filterMode: 'all' | 'favorites';
  setFilterMode: (mode: 'all' | 'favorites') => void;
  favoriteCount: number;
  updateFavoriteCount: (count: number) => void;
}
```

### 3. LocalStorage
フィルター状態を保持:
```typescript
{
  "video_filter_mode": "all" | "favorites"
}
```

## 実装順序

1. **データベース層**
   - マイグレーションファイル作成
   - データベースメソッド追加 (`toggle_favorite`, `get_favorite_videos`, `get_favorite_count`)

2. **バックエンドAPI**
   - Tauri commandハンドラー実装
   - 既存の `get_videos` に is_favorite を含める

3. **フロントエンド - データ層**
   - TypeScript型定義更新（Video型に `is_favorite` 追加）
   - Tauri commandラッパー作成

4. **フロントエンド - UI**
   - FilterBar コンポーネント作成
   - VideoCard にお気に入りボタン追加
   - アニメーション実装

5. **テスト**
   - データベース層のユニットテスト
   - フロントエンドコンポーネントテスト
   - E2Eテスト

## テストケース

### バックエンド
- [ ] お気に入り状態のトグルが正常に動作する
- [ ] お気に入りの動画のみが取得できる
- [ ] お気に入り数が正確にカウントされる
- [ ] 存在しない動画IDでエラーが返る

### フロントエンド
- [ ] お気に入りボタンが表示される
- [ ] クリックで状態がトグルされる
- [ ] フィルターボタンが動作する
- [ ] フィルター状態が保持される
- [ ] お気に入り数が正しく表示される

## 非機能要件

### パフォーマンス
- お気に入りトグルは500ms以内に完了
- インデックスにより大量の動画でも高速フィルタリング

### ユーザビリティ
- 直感的なアイコン（★）使用
- ホバーとアクティブ状態で視覚的フィードバック
- ローディング中の適切な状態表示

### 保守性
- 既存コードへの影響を最小限に
- 型安全性を維持
- テストカバレッジ80%以上

## 今後の拡張案
- お気に入りのエクスポート/インポート
- お気に入りリストの並び替え
- スマートプレイリスト（お気に入り + その他の条件）
- お気に入りに追加した日時の記録
