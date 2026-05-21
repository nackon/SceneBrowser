import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { invoke } from '@tauri-apps/api/core';
import { VideoCard } from './VideoCard';
import type { Video } from '../types/video';

// Mock the commands module
vi.mock('../services/commands', () => ({
  generateThumbnail: vi.fn(),
  readThumbnail: vi.fn(),
  regenerateThumbnail: vi.fn(),
  toggleFavorite: vi.fn(),
}));

describe('VideoCard', () => {
  const mockVideo: Video = {
    id: 1,
    folder_id: 1,
    path: '/test/video.mp4',
    filename: 'test-video.mp4',
    hash: 'test-hash',
    duration: 120,
    width: 1920,
    height: 1080,
    size: 1024 * 1024 * 100, // 100MB
    codec: 'h264',
    framerate: 30,
    thumbnail_path: '/test/thumbnail.jpg',
    thumbnail_count: 9,
    rating: 0,
    is_favorite: 0,
    created_at: '2026-01-01T00:00:00Z',
    updated_at: '2026-01-01T00:00:00Z',
    scanned_at: '2026-01-01T00:00:00Z',
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('calls invoke with open_video_with_player when clicked', async () => {
    const user = userEvent.setup();
    const invokeMock = vi.mocked(invoke);
    invokeMock.mockResolvedValue(undefined);

    render(<VideoCard video={mockVideo} folderId={1} />);

    const card = screen.getByText('test-video.mp4').closest('.video-card');
    expect(card).toBeInTheDocument();

    await user.click(card!);

    await waitFor(() => {
      expect(invokeMock).toHaveBeenCalledWith('open_video_with_player', {
        videoPath: '/test/video.mp4',
      });
    });
  });

  it('shows alert when opening video fails', async () => {
    const user = userEvent.setup();
    const invokeMock = vi.mocked(invoke);
    const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});

    invokeMock.mockRejectedValue(new Error('Permission denied'));

    render(<VideoCard video={mockVideo} folderId={1} />);

    const card = screen.getByText('test-video.mp4').closest('.video-card');
    await user.click(card!);

    await waitFor(() => {
      expect(alertMock).toHaveBeenCalledWith(
        expect.stringContaining('Failed to open video')
      );
    });

    alertMock.mockRestore();
  });

  it('displays video metadata correctly', () => {
    render(<VideoCard video={mockVideo} folderId={1} />);

    expect(screen.getByText('test-video.mp4')).toBeInTheDocument();
    expect(screen.getByText('1920x1080')).toBeInTheDocument();
    expect(screen.getByText('100.00 MB')).toBeInTheDocument();
    expect(screen.getByText('2:00')).toBeInTheDocument(); // 120 seconds = 2:00
  });

  it('formats duration correctly for hours', () => {
    const longVideo = { ...mockVideo, duration: 7265 }; // 2:01:05
    render(<VideoCard video={longVideo} folderId={1} />);

    expect(screen.getByText('2:01:05')).toBeInTheDocument();
  });

  it('logs click events to console', async () => {
    const user = userEvent.setup();
    const consoleLogMock = vi.spyOn(console, 'log').mockImplementation(() => {});
    const invokeMock = vi.mocked(invoke);
    invokeMock.mockResolvedValue(undefined);

    render(<VideoCard video={mockVideo} folderId={1} />);

    const card = screen.getByText('test-video.mp4').closest('.video-card');
    await user.click(card!);

    await waitFor(() => {
      expect(consoleLogMock).toHaveBeenCalledWith(
        'Video card clicked, path:',
        '/test/video.mp4'
      );
      expect(consoleLogMock).toHaveBeenCalledWith('Opening video with configured player...');
    });

    consoleLogMock.mockRestore();
  });

  describe('Favorites', () => {
    it('displays favorite button', () => {
      render(<VideoCard video={mockVideo} folderId={1} />);

      const favoriteButton = document.querySelector('.favorite-button');
      expect(favoriteButton).toBeInTheDocument();
    });

    it('shows empty star for non-favorite video', () => {
      const nonFavoriteVideo = { ...mockVideo, is_favorite: 0 };
      render(<VideoCard video={nonFavoriteVideo} folderId={1} />);

      const favoriteButton = document.querySelector('.favorite-button');
      expect(favoriteButton?.textContent).toBe('☆');
      expect(favoriteButton).not.toHaveClass('favorite');
    });

    it('shows filled star for favorite video', () => {
      const favoriteVideo = { ...mockVideo, is_favorite: 1 };
      render(<VideoCard video={favoriteVideo} folderId={1} />);

      const favoriteButton = document.querySelector('.favorite-button');
      expect(favoriteButton?.textContent).toBe('★');
      expect(favoriteButton).toHaveClass('favorite');
    });

    it('toggles favorite when button is clicked', async () => {
      const user = userEvent.setup();
      const { toggleFavorite } = await import('../services/commands');
      const toggleFavoriteMock = vi.mocked(toggleFavorite);
      toggleFavoriteMock.mockResolvedValue(true);

      render(<VideoCard video={mockVideo} folderId={1} />);

      const favoriteButton = document.querySelector('.favorite-button');
      expect(favoriteButton?.textContent).toBe('☆');

      await user.click(favoriteButton!);

      await waitFor(() => {
        expect(toggleFavoriteMock).toHaveBeenCalledWith(1, 1);
        expect(favoriteButton?.textContent).toBe('★');
      });
    });

    it('shows alert when folderId is null', async () => {
      const user = userEvent.setup();
      const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});

      render(<VideoCard video={mockVideo} folderId={null} />);

      const favoriteButton = document.querySelector('.favorite-button');
      await user.click(favoriteButton!);

      expect(alertMock).toHaveBeenCalledWith('Please select a folder first');
      alertMock.mockRestore();
    });

    it('shows alert when toggle fails', async () => {
      const user = userEvent.setup();
      const { toggleFavorite } = await import('../services/commands');
      const toggleFavoriteMock = vi.mocked(toggleFavorite);
      const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});

      toggleFavoriteMock.mockRejectedValue(new Error('Network error'));

      render(<VideoCard video={mockVideo} folderId={1} />);

      const favoriteButton = document.querySelector('.favorite-button');
      await user.click(favoriteButton!);

      await waitFor(() => {
        expect(alertMock).toHaveBeenCalledWith(
          expect.stringContaining('Failed to toggle favorite')
        );
      });

      alertMock.mockRestore();
    });

    it('calls onFavoriteToggled callback when toggled', async () => {
      const user = userEvent.setup();
      const { toggleFavorite } = await import('../services/commands');
      const toggleFavoriteMock = vi.mocked(toggleFavorite);
      const onFavoriteToggled = vi.fn();

      toggleFavoriteMock.mockResolvedValue(true);

      render(
        <VideoCard
          video={mockVideo}
          folderId={1}
          onFavoriteToggled={onFavoriteToggled}
        />
      );

      const favoriteButton = document.querySelector('.favorite-button');
      await user.click(favoriteButton!);

      await waitFor(() => {
        expect(onFavoriteToggled).toHaveBeenCalled();
      });
    });

    it('does not trigger video opening when favorite button is clicked', async () => {
      const user = userEvent.setup();
      const { toggleFavorite } = await import('../services/commands');
      const toggleFavoriteMock = vi.mocked(toggleFavorite);
      const invokeMock = vi.mocked(invoke);

      toggleFavoriteMock.mockResolvedValue(true);

      render(<VideoCard video={mockVideo} folderId={1} />);

      const favoriteButton = document.querySelector('.favorite-button');
      await user.click(favoriteButton!);

      await waitFor(() => {
        expect(toggleFavoriteMock).toHaveBeenCalled();
      });

      // Should not have called open_video_with_player
      expect(invokeMock).not.toHaveBeenCalledWith('open_video_with_player', expect.any(Object));
    });
  });
});
