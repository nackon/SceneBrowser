import { create } from 'zustand';
import type { Video, Folder } from '../types/video';

interface VideoStore {
  // Data
  videos: Video[];
  folders: Folder[];
  selectedFolder: number | null;
  searchQuery: string;

  // UI State
  isLoading: boolean;
  error: string | null;
  scanProgress: { current: number; total: number; file: string } | null;

  // Actions
  setVideos: (videos: Video[]) => void;
  addVideos: (videos: Video[]) => void;
  setFolders: (folders: Folder[]) => void;
  setSelectedFolder: (folderId: number | null) => void;
  setSearchQuery: (query: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  setScanProgress: (progress: { current: number; total: number; file: string } | null) => void;
  clearVideos: () => void;
  updateVideoThumbnail: (videoId: number, thumbnailPath: string) => void;
}

export const useVideoStore = create<VideoStore>((set) => ({
  // Initial state
  videos: [],
  folders: [],
  selectedFolder: null,
  searchQuery: '',
  isLoading: false,
  error: null,
  scanProgress: null,

  // Actions
  setVideos: (videos) => set({ videos }),

  addVideos: (videos) =>
    set((state) => ({
      videos: [...state.videos, ...videos],
    })),

  setFolders: (folders) => set({ folders }),

  setSelectedFolder: (folderId) =>
    set({
      selectedFolder: folderId,
      videos: [], // Clear videos when changing folder
    }),

  setSearchQuery: (query) => set({ searchQuery: query }),

  setIsLoading: (loading) => set({ isLoading: loading }),

  setError: (error) => set({ error }),

  setScanProgress: (progress) => set({ scanProgress: progress }),

  clearVideos: () => set({ videos: [] }),

  updateVideoThumbnail: (videoId, thumbnailPath) =>
    set((state) => ({
      videos: state.videos.map((video) =>
        video.id === videoId ? { ...video, thumbnail_path: thumbnailPath } : video
      ),
    })),
}));
