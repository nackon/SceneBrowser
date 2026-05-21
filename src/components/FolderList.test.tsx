import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor, act } from '@testing-library/react';
import { FolderList } from './FolderList';
import * as commands from '../services/commands';
import { listen } from '@tauri-apps/api/event';
import { useVideoStore } from '../store/videoStore';

// Mock the commands module
vi.mock('../services/commands', () => ({
  getFolders: vi.fn(),
  addFolder: vi.fn(),
  scanFolder: vi.fn(),
  removeFolder: vi.fn(),
  generateThumbnailsBatch: vi.fn(),
  getVideos: vi.fn(),
}));

// Mock the dialog plugin
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(),
  confirm: vi.fn(),
}));

// Mock the event API
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(),
}));

describe('FolderList', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset store
    act(() => {
      const state = useVideoStore.getState();
      state.clearVideos();
      state.setSelectedFolder(null);
    });

    // Default mock for getFolders
    vi.mocked(commands.getFolders).mockResolvedValue([]);

    // Default mock for listen - returns a promise with unlisten function
    vi.mocked(listen).mockResolvedValue((() => {}) as any);
  });

  it('renders folder list header', async () => {
    render(<FolderList />);

    expect(screen.getByText('Folders')).toBeInTheDocument();
    expect(screen.getByTitle('Add folder')).toBeInTheDocument();
  });

  it('loads folders on mount', async () => {
    const mockFolders = [
      { id: 1, path: '/test/folder1', recursive: true, created_at: '2026-01-01', updated_at: '2026-01-01' },
      { id: 2, path: '/test/folder2', recursive: true, created_at: '2026-01-01', updated_at: '2026-01-01' },
    ];

    vi.mocked(commands.getFolders).mockResolvedValue(mockFolders);

    render(<FolderList />);

    await waitFor(() => {
      expect(commands.getFolders).toHaveBeenCalled();
    });
  });

  it('shows empty state when no folders', async () => {
    vi.mocked(commands.getFolders).mockResolvedValue([]);

    render(<FolderList />);

    await waitFor(() => {
      expect(screen.getByText('No folders added yet')).toBeInTheDocument();
    });
  });

  it('displays folder list when folders exist', async () => {
    const mockFolders = [
      { id: 1, path: '/test/videos', recursive: true, created_at: '2026-01-01', updated_at: '2026-01-01' },
    ];

    vi.mocked(commands.getFolders).mockResolvedValue(mockFolders);

    render(<FolderList />);

    await waitFor(() => {
      expect(screen.getByText('videos')).toBeInTheDocument();
      expect(screen.getByText('/test/videos')).toBeInTheDocument();
    });
  });

  describe('thumbnail_generated event listener', () => {
    it('sets up event listener on mount', async () => {
      const mockFolders = [
        { id: 1, path: '/test/folder1', recursive: true, created_at: '2026-01-01', updated_at: '2026-01-01' },
      ];

      vi.mocked(commands.getFolders).mockResolvedValue(mockFolders);

      // Mock listen to capture the event handler
      let eventHandler: ((event: { payload: { video_id: number; thumbnail_path: string } }) => void) | null = null;
      vi.mocked(listen).mockImplementation(async (eventName, handler) => {
        if (eventName === 'thumbnail_generated') {
          eventHandler = handler as typeof eventHandler;
        }
        return (() => {}) as any; // Return unlisten function
      });

      render(<FolderList />);

      await waitFor(() => {
        expect(listen).toHaveBeenCalledWith('thumbnail_generated', expect.any(Function));
      });

      // Verify the handler was set
      expect(eventHandler).toBeTruthy();
    });

    it('updates video thumbnail when event is received', async () => {
      const mockFolders = [
        { id: 1, path: '/test/folder1', recursive: true, created_at: '2026-01-01', updated_at: '2026-01-01' },
      ];

      vi.mocked(commands.getFolders).mockResolvedValue(mockFolders);

      // Setup mock videos in store
      const mockVideo = {
        id: 1,
        folder_id: 1,
        path: '/test/video.mp4',
        filename: 'video.mp4',
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
        is_favorite: 0,
        created_at: '2026-01-01T00:00:00Z',
        updated_at: '2026-01-01T00:00:00Z',
        scanned_at: '2026-01-01T00:00:00Z',
      };

      // Capture event handler
      let eventHandler: ((event: { payload: { video_id: number; thumbnail_path: string } }) => void) | null = null;
      vi.mocked(listen).mockImplementation(async (eventName, handler) => {
        if (eventName === 'thumbnail_generated') {
          eventHandler = handler as typeof eventHandler;
        }
        return (() => {}) as any;
      });

      render(<FolderList />);

      await waitFor(() => {
        expect(listen).toHaveBeenCalled();
      });

      // Set initial video state
      act(() => {
        useVideoStore.getState().setVideos([mockVideo]);
      });

      // Simulate thumbnail_generated event
      act(() => {
        eventHandler?.({
          payload: {
            video_id: 1,
            thumbnail_path: '/cache/thumbnail1.jpg'
          }
        });
      });

      // Check that the store was updated
      await waitFor(() => {
        const videos = useVideoStore.getState().videos;
        expect(videos[0].thumbnail_path).toBe('/cache/thumbnail1.jpg');
      });
    });

    it('cleans up event listener on unmount', async () => {
      const mockFolders = [
        { id: 1, path: '/test/folder1', recursive: true, created_at: '2026-01-01', updated_at: '2026-01-01' },
      ];

      vi.mocked(commands.getFolders).mockResolvedValue(mockFolders);

      const unlistenMock = vi.fn();
      vi.mocked(listen).mockResolvedValue(unlistenMock as any);

      const { unmount } = render(<FolderList />);

      await waitFor(() => {
        expect(listen).toHaveBeenCalled();
      });

      unmount();

      // Note: The cleanup happens asynchronously, so we need to wait a bit
      await waitFor(() => {
        expect(unlistenMock).toHaveBeenCalled();
      }, { timeout: 100 });
    });
  });

  describe('folder actions', () => {
    it('shows scan button for each folder', async () => {
      const mockFolders = [
        { id: 1, path: '/test/folder1', recursive: true, created_at: '2026-01-01', updated_at: '2026-01-01' },
      ];

      vi.mocked(commands.getFolders).mockResolvedValue(mockFolders);

      render(<FolderList />);

      await waitFor(() => {
        expect(screen.getByTitle('Scan folder')).toBeInTheDocument();
      });
    });

    it('shows thumbnail generation button for each folder', async () => {
      const mockFolders = [
        { id: 1, path: '/test/folder1', recursive: true, created_at: '2026-01-01', updated_at: '2026-01-01' },
      ];

      vi.mocked(commands.getFolders).mockResolvedValue(mockFolders);

      render(<FolderList />);

      await waitFor(() => {
        expect(screen.getByTitle('Generate thumbnails')).toBeInTheDocument();
      });
    });

    it('shows delete button for each folder', async () => {
      const mockFolders = [
        { id: 1, path: '/test/folder1', recursive: true, created_at: '2026-01-01', updated_at: '2026-01-01' },
      ];

      vi.mocked(commands.getFolders).mockResolvedValue(mockFolders);

      render(<FolderList />);

      await waitFor(() => {
        expect(screen.getByTitle('Remove folder')).toBeInTheDocument();
      });
    });
  });
});
