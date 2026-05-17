import { describe, it, expect, beforeEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useVideoStore } from './videoStore';
import type { Video } from '../types/video';

describe('videoStore', () => {
  const mockVideo1: Video = {
    id: 1,
    folder_id: 1,
    path: '/test/video1.mp4',
    filename: 'video1.mp4',
    hash: 'hash1',
    duration: 120,
    width: 1920,
    height: 1080,
    size: 1024 * 1024 * 100,
    codec: 'h264',
    framerate: 30,
    thumbnail_path: null,
    thumbnail_count: 9,
    rating: 0,
    created_at: '2026-01-01T00:00:00Z',
    updated_at: '2026-01-01T00:00:00Z',
    scanned_at: '2026-01-01T00:00:00Z',
  };

  const mockVideo2: Video = {
    ...mockVideo1,
    id: 2,
    path: '/test/video2.mp4',
    filename: 'video2.mp4',
    hash: 'hash2',
  };

  beforeEach(() => {
    // Reset store state before each test
    act(() => {
      const state = useVideoStore.getState();
      state.clearVideos();
      state.setSelectedFolder(null);
      state.setSearchQuery('');
      state.setIsLoading(false);
      state.setError(null);
      state.setScanProgress(null);
    });
  });

  describe('setVideos', () => {
    it('should set videos array', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setVideos([mockVideo1, mockVideo2]);
      });

      expect(result.current.videos).toEqual([mockVideo1, mockVideo2]);
    });

    it('should replace existing videos', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setVideos([mockVideo1]);
      });
      act(() => {
        result.current.setVideos([mockVideo2]);
      });

      expect(result.current.videos).toEqual([mockVideo2]);
      expect(result.current.videos).toHaveLength(1);
    });
  });

  describe('addVideos', () => {
    it('should append videos to existing array', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setVideos([mockVideo1]);
        result.current.addVideos([mockVideo2]);
      });

      expect(result.current.videos).toEqual([mockVideo1, mockVideo2]);
    });

    it('should work with empty initial array', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.addVideos([mockVideo1]);
      });

      expect(result.current.videos).toEqual([mockVideo1]);
    });
  });

  describe('updateVideoThumbnail', () => {
    it('should update thumbnail_path for matching video', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setVideos([mockVideo1, mockVideo2]);
        result.current.updateVideoThumbnail(1, '/cache/thumbnail1.jpg');
      });

      expect(result.current.videos[0].thumbnail_path).toBe('/cache/thumbnail1.jpg');
      expect(result.current.videos[1].thumbnail_path).toBeNull();
    });

    it('should not modify other video properties', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setVideos([mockVideo1]);
        result.current.updateVideoThumbnail(1, '/cache/thumbnail1.jpg');
      });

      expect(result.current.videos[0]).toEqual({
        ...mockVideo1,
        thumbnail_path: '/cache/thumbnail1.jpg',
      });
    });

    it('should handle non-existent video id gracefully', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setVideos([mockVideo1]);
        result.current.updateVideoThumbnail(999, '/cache/thumbnail999.jpg');
      });

      expect(result.current.videos[0].thumbnail_path).toBeNull();
    });

    it('should update only the matching video when multiple videos exist', () => {
      const { result } = renderHook(() => useVideoStore());
      const video3 = { ...mockVideo1, id: 3 };

      act(() => {
        result.current.setVideos([mockVideo1, mockVideo2, video3]);
        result.current.updateVideoThumbnail(2, '/cache/thumbnail2.jpg');
      });

      expect(result.current.videos[0].thumbnail_path).toBeNull();
      expect(result.current.videos[1].thumbnail_path).toBe('/cache/thumbnail2.jpg');
      expect(result.current.videos[2].thumbnail_path).toBeNull();
    });
  });

  describe('setSelectedFolder', () => {
    it('should set selected folder and clear videos', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setVideos([mockVideo1]);
        result.current.setSelectedFolder(5);
      });

      expect(result.current.selectedFolder).toBe(5);
      expect(result.current.videos).toEqual([]);
    });

    it('should handle null folder id', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setSelectedFolder(1);
        result.current.setSelectedFolder(null);
      });

      expect(result.current.selectedFolder).toBeNull();
    });
  });

  describe('setSearchQuery', () => {
    it('should update search query', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setSearchQuery('test query');
      });

      expect(result.current.searchQuery).toBe('test query');
    });
  });

  describe('setIsLoading', () => {
    it('should update loading state', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setIsLoading(true);
      });

      expect(result.current.isLoading).toBe(true);
    });
  });

  describe('setError', () => {
    it('should set error message', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setError('Test error');
      });

      expect(result.current.error).toBe('Test error');
    });

    it('should clear error when set to null', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setError('Test error');
        result.current.setError(null);
      });

      expect(result.current.error).toBeNull();
    });
  });

  describe('setScanProgress', () => {
    it('should set scan progress', () => {
      const { result } = renderHook(() => useVideoStore());
      const progress = { current: 5, total: 10, file: 'test.mp4' };

      act(() => {
        result.current.setScanProgress(progress);
      });

      expect(result.current.scanProgress).toEqual(progress);
    });

    it('should clear scan progress when set to null', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setScanProgress({ current: 5, total: 10, file: 'test.mp4' });
        result.current.setScanProgress(null);
      });

      expect(result.current.scanProgress).toBeNull();
    });
  });

  describe('clearVideos', () => {
    it('should clear all videos', () => {
      const { result } = renderHook(() => useVideoStore());

      act(() => {
        result.current.setVideos([mockVideo1, mockVideo2]);
        result.current.clearVideos();
      });

      expect(result.current.videos).toEqual([]);
    });
  });
});
