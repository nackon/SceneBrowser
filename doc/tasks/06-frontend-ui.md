# Task: Frontend UI Components

## Description
React UIコンポーネント（グリッド表示、サイドバー、動画カードなど）を実装します。

## Objectives
- [ ] レイアウトコンポーネントの実装
- [ ] VideoGridコンポーネント（仮想スクロール）
- [ ] VideoCardコンポーネント
- [ ] Sidebarコンポーネント
- [ ] FolderListコンポーネント
- [ ] SearchBarコンポーネント

## Steps

### 1. App Layout (`src/App.tsx`)
```tsx
import { Sidebar } from './components/Sidebar';
import { VideoGrid } from './components/VideoGrid';
import { useVideoStore } from './store/videoStore';
import './App.css';

function App() {
  const { videos, isLoading } = useVideoStore();
  
  return (
    <div className="app">
      <Sidebar />
      <main className="main-content">
        {isLoading ? (
          <div className="loading">Loading...</div>
        ) : (
          <VideoGrid videos={videos} />
        )}
      </main>
    </div>
  );
}

export default App;
```

### 2. Video Grid (`src/components/VideoGrid.tsx`)
```tsx
import { FixedSizeGrid } from 'react-window';
import AutoSizer from 'react-virtualized-auto-sizer';
import { VideoCard } from './VideoCard';
import type { Video } from '../types/video';

const CARD_WIDTH = 360;
const CARD_HEIGHT = 280;
const GUTTER = 16;

interface VideoGridProps {
  videos: Video[];
}

export function VideoGrid({ videos }: VideoGridProps) {
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
            itemData={{ videos, columnCount }}
          >
            {Cell}
          </FixedSizeGrid>
        );
      }}
    </AutoSizer>
  );
}

function Cell({ columnIndex, rowIndex, style, data }) {
  const { videos, columnCount } = data;
  const index = rowIndex * columnCount + columnIndex;
  const video = videos[index];
  
  if (!video) return null;
  
  return (
    <div style={style}>
      <VideoCard video={video} />
    </div>
  );
}
```

### 3. Video Card (`src/components/VideoCard.tsx`)
```tsx
import { convertFileSrc } from '@tauri-apps/api/core';
import type { Video } from '../types/video';
import './VideoCard.css';

interface VideoCardProps {
  video: Video;
}

export function VideoCard({ video }: VideoCardProps) {
  const thumbnailUrl = video.thumbnail_path 
    ? convertFileSrc(video.thumbnail_path)
    : '/placeholder.jpg';
  
  const formatDuration = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    
    if (hours > 0) {
      return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
    }
    return `${minutes}:${secs.toString().padStart(2, '0')}`;
  };
  
  const formatSize = (bytes: number) => {
    const gb = bytes / (1024 ** 3);
    if (gb >= 1) return `${gb.toFixed(2)} GB`;
    const mb = bytes / (1024 ** 2);
    return `${mb.toFixed(2)} MB`;
  };
  
  return (
    <div className="video-card">
      <div className="thumbnail-container">
        <img 
          src={thumbnailUrl} 
          alt={video.filename}
          loading="lazy"
          className="thumbnail"
        />
        <div className="duration-badge">
          {formatDuration(video.duration)}
        </div>
      </div>
      <div className="video-info">
        <h3 className="filename" title={video.filename}>
          {video.filename}
        </h3>
        <div className="metadata">
          <span>{video.width}x{video.height}</span>
          <span>{formatSize(video.size)}</span>
        </div>
      </div>
    </div>
  );
}
```

### 4. Sidebar (`src/components/Sidebar.tsx`)
```tsx
import { FolderList } from './FolderList';
import { SearchBar } from './SearchBar';
import './Sidebar.css';

export function Sidebar() {
  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <h1>SceneBrowser</h1>
      </div>
      <SearchBar />
      <FolderList />
    </aside>
  );
}
```

### 5. Folder List (`src/components/FolderList.tsx`)
```tsx
import { useEffect, useState } from 'react';
import { getFolders, addFolder, scanFolder } from '../services/commands';
import { useVideoStore } from '../store/videoStore';
import type { Folder, ScanProgress } from '../types/video';
import './FolderList.css';

export function FolderList() {
  const [folders, setFolders] = useState<Folder[]>([]);
  const [scanning, setScanning] = useState(false);
  const [progress, setProgress] = useState<ScanProgress | null>(null);
  const { selectedFolder, setSelectedFolder } = useVideoStore();
  
  useEffect(() => {
    loadFolders();
  }, []);
  
  async function loadFolders() {
    const data = await getFolders();
    setFolders(data);
  }
  
  async function handleAddFolder() {
    // TODO: Implement folder picker dialog
    const path = await window.prompt('Enter folder path:');
    if (path) {
      await addFolder(path, true);
      await loadFolders();
    }
  }
  
  async function handleScanFolder(folderId: number) {
    setScanning(true);
    try {
      await scanFolder(folderId, (p) => setProgress(p));
      // Refresh video list
    } finally {
      setScanning(false);
      setProgress(null);
    }
  }
  
  return (
    <div className="folder-list">
      <div className="folder-list-header">
        <h2>Folders</h2>
        <button onClick={handleAddFolder}>+ Add</button>
      </div>
      
      {scanning && progress && (
        <div className="scan-progress">
          Scanning: {progress.current}/{progress.total}
        </div>
      )}
      
      <ul>
        {folders.map((folder) => (
          <li 
            key={folder.id}
            className={selectedFolder === folder.id ? 'selected' : ''}
            onClick={() => setSelectedFolder(folder.id)}
          >
            <span className="folder-path">{folder.path}</span>
            <button onClick={() => handleScanFolder(folder.id)}>
              Scan
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}
```

### 6. Search Bar (`src/components/SearchBar.tsx`)
```tsx
import { useState } from 'react';
import { searchVideos } from '../services/commands';
import { useVideoStore } from '../store/videoStore';
import './SearchBar.css';

export function SearchBar() {
  const [query, setQuery] = useState('');
  const { setVideos, setIsLoading } = useVideoStore();
  
  async function handleSearch(e: React.FormEvent) {
    e.preventDefault();
    if (!query.trim()) return;
    
    setIsLoading(true);
    try {
      const results = await searchVideos(query);
      setVideos(results);
    } finally {
      setIsLoading(false);
    }
  }
  
  return (
    <form className="search-bar" onSubmit={handleSearch}>
      <input
        type="text"
        placeholder="Search videos..."
        value={query}
        onChange={(e) => setQuery(e.target.value)}
      />
      <button type="submit">Search</button>
    </form>
  );
}
```

### 7. CSS Styling
各コンポーネントに対応する `.css` ファイルを作成

## Acceptance Criteria
- [ ] 全コンポーネントが正常にレンダリングされる
- [ ] 仮想スクロールが滑らかに動作する
- [ ] フォルダ追加・スキャン機能が動作する
- [ ] 検索機能が動作する
- [ ] レスポンシブデザインが適用されている

## Estimated Time
8 hours

## Dependencies
- Task #05 (Frontend Types)

## Labels
- frontend
- ui
- react
- phase-1
