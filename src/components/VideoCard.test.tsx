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
    created_at: '2026-01-01T00:00:00Z',
    updated_at: '2026-01-01T00:00:00Z',
    scanned_at: '2026-01-01T00:00:00Z',
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('calls invoke with opener plugin when clicked', async () => {
    const user = userEvent.setup();
    const invokeMock = vi.mocked(invoke);
    invokeMock.mockResolvedValue(undefined);

    render(<VideoCard video={mockVideo} />);

    const card = screen.getByText('test-video.mp4').closest('.video-card');
    expect(card).toBeInTheDocument();

    await user.click(card!);

    await waitFor(() => {
      expect(invokeMock).toHaveBeenCalledWith('plugin:opener|open_path', {
        path: '/test/video.mp4',
      });
    });
  });

  it('shows alert when opening video fails', async () => {
    const user = userEvent.setup();
    const invokeMock = vi.mocked(invoke);
    const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});

    invokeMock.mockRejectedValue(new Error('Permission denied'));

    render(<VideoCard video={mockVideo} />);

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
    render(<VideoCard video={mockVideo} />);

    expect(screen.getByText('test-video.mp4')).toBeInTheDocument();
    expect(screen.getByText('1920x1080')).toBeInTheDocument();
    expect(screen.getByText('100.00 MB')).toBeInTheDocument();
    expect(screen.getByText('2:00')).toBeInTheDocument(); // 120 seconds = 2:00
  });

  it('formats duration correctly for hours', () => {
    const longVideo = { ...mockVideo, duration: 7265 }; // 2:01:05
    render(<VideoCard video={longVideo} />);

    expect(screen.getByText('2:01:05')).toBeInTheDocument();
  });

  it('logs click events to console', async () => {
    const user = userEvent.setup();
    const consoleLogMock = vi.spyOn(console, 'log').mockImplementation(() => {});
    const invokeMock = vi.mocked(invoke);
    invokeMock.mockResolvedValue(undefined);

    render(<VideoCard video={mockVideo} />);

    const card = screen.getByText('test-video.mp4').closest('.video-card');
    await user.click(card!);

    await waitFor(() => {
      expect(consoleLogMock).toHaveBeenCalledWith(
        'Video card clicked, path:',
        '/test/video.mp4'
      );
      expect(consoleLogMock).toHaveBeenCalledWith('Opening video...');
    });

    consoleLogMock.mockRestore();
  });
});
