# Task: Frontend TypeScript Types and Services

## Description
TypeScript型定義とTauriコマンドラッパーを実装します。

## Objectives
- [ ] TypeScript型定義の作成
- [ ] Tauriコマンドラッパーの実装
- [ ] 状態管理の設定

## Steps

### 1. Type Definitions (`src/types/video.ts`)
```typescript
export interface Video {
  id: number;
  folder_id: number;
  path: string;
  filename: string;
  hash: string;
  duration: number;
  width: number;
  height: number;
  size: number;
  codec: string | null;
  framerate: number | null;
  thumbnail_path: string | null;
  thumbnail_count: number;
  rating: number;
  created_at: string;
  updated_at: string;
  scanned_at: string;
}

export interface Folder {
  id: number;
  path: string;
  recursive: boolean;
  created_at: string;
  updated_at: string;
}

export interface ScanResult {
  videos_found: number;
  videos_added: number;
  videos_updated: number;
}

export interface ScanProgress {
  current: number;
  total: number;
  current_file: string;
}
```

### 2. Command Wrappers (`src/services/commands.ts`)
```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { Video, Folder, ScanResult, ScanProgress } from '../types/video';

// Folder Commands
export async function addFolder(path: string, recursive: boolean): Promise<number> {
  return await invoke<number>('add_folder', { path, recursive });
}

export async function getFolders(): Promise<Folder[]> {
  return await invoke<Folder[]>('get_folders');
}

export async function removeFolder(folderId: number): Promise<void> {
  await invoke('remove_folder', { folderId });
}

// Video Commands
export async function scanFolder(
  folderId: number,
  onProgress?: (progress: ScanProgress) => void
): Promise<ScanResult> {
  if (onProgress) {
    const unlisten = await listen<ScanProgress>('scan_progress', (event) => {
      onProgress(event.payload);
    });
    
    try {
      const result = await invoke<ScanResult>('scan_folder', { folderId });
      unlisten();
      return result;
    } catch (error) {
      unlisten();
      throw error;
    }
  }
  
  return await invoke<ScanResult>('scan_folder', { folderId });
}

export async function getVideos(
  folderId: number | null,
  limit: number,
  offset: number
): Promise<Video[]> {
  return await invoke<Video[]>('get_videos', { folderId, limit, offset });
}

export async function searchVideos(query: string): Promise<Video[]> {
  return await invoke<Video[]>('search_videos', { query });
}

export async function getVideoById(videoId: number): Promise<Video> {
  return await invoke<Video>('get_video_by_id', { videoId });
}

// Thumbnail Commands
export async function generateThumbnail(videoId: number): Promise<string> {
  return await invoke<string>('generate_thumbnail', { videoId });
}
```

### 3. State Management (`src/store/videoStore.ts`)
```typescript
import { create } from 'zustand';
import type { Video, Folder } from '../types/video';

interface VideoStore {
  videos: Video[];
  folders: Folder[];
  selectedFolder: number | null;
  isLoading: boolean;
  searchQuery: string;
  
  setVideos: (videos: Video[]) => void;
  addVideos: (videos: Video[]) => void;
  setFolders: (folders: Folder[]) => void;
  setSelectedFolder: (folderId: number | null) => void;
  setIsLoading: (loading: boolean) => void;
  setSearchQuery: (query: string) => void;
}

export const useVideoStore = create<VideoStore>((set) => ({
  videos: [],
  folders: [],
  selectedFolder: null,
  isLoading: false,
  searchQuery: '',
  
  setVideos: (videos) => set({ videos }),
  addVideos: (videos) => set((state) => ({ 
    videos: [...state.videos, ...videos] 
  })),
  setFolders: (folders) => set({ folders }),
  setSelectedFolder: (folderId) => set({ selectedFolder: folderId }),
  setIsLoading: (loading) => set({ isLoading: loading }),
  setSearchQuery: (query) => set({ searchQuery: query }),
}));
```

### 4. Custom Hooks (`src/hooks/useVideos.ts`)
```typescript
import { useEffect } from 'react';
import { useVideoStore } from '../store/videoStore';
import { getVideos } from '../services/commands';

export function useVideos(folderId: number | null, limit = 100) {
  const { setVideos, setIsLoading } = useVideoStore();
  
  useEffect(() => {
    let cancelled = false;
    
    async function fetchVideos() {
      setIsLoading(true);
      try {
        const videos = await getVideos(folderId, limit, 0);
        if (!cancelled) {
          setVideos(videos);
        }
      } catch (error) {
        console.error('Failed to fetch videos:', error);
      } finally {
        if (!cancelled) {
          setIsLoading(false);
        }
      }
    }
    
    fetchVideos();
    
    return () => {
      cancelled = true;
    };
  }, [folderId, limit, setVideos, setIsLoading]);
}
```

## Acceptance Criteria
- [ ] 全型定義がRustの構造体と一致している
- [ ] コマンドラッパーが正常に動作する
- [ ] 状態管理が正しく機能する
- [ ] TypeScriptコンパイルエラーがない

## Estimated Time
3 hours

## Dependencies
- Task #04 (Tauri Commands)

## Labels
- frontend
- typescript
- phase-1
