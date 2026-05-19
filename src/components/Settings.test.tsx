import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { Settings } from './Settings';

vi.mock('@tauri-apps/api/core');
vi.mock('@tauri-apps/plugin-dialog');

describe('Settings', () => {
  const mockSettings = [
    {
      id: 1,
      file_extension: '*',
      player_path: '',
      created_at: '2026-01-01T00:00:00Z',
      updated_at: '2026-01-01T00:00:00Z',
    },
    {
      id: 2,
      file_extension: 'mp4',
      player_path: '/Applications/VLC.app',
      created_at: '2026-01-01T00:00:00Z',
      updated_at: '2026-01-01T00:00:00Z',
    },
  ];

  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(invoke).mockImplementation((cmd) => {
      if (cmd === 'get_video_player_settings') {
        return Promise.resolve(mockSettings);
      }
      return Promise.resolve(undefined);
    });
  });

  it('renders settings title', async () => {
    render(<Settings />);
    await waitFor(() => {
      expect(screen.getByText('Video Player Settings')).toBeInTheDocument();
    });
  });

  it('loads and displays existing settings', async () => {
    render(<Settings />);

    await waitFor(() => {
      expect(screen.getByText('All video files')).toBeInTheDocument();
      expect(screen.getByText('.mp4')).toBeInTheDocument();
      expect(screen.getByText('VLC')).toBeInTheDocument();
    });
  });

  it('shows "System Default" when player_path is empty', async () => {
    render(<Settings />);

    await waitFor(() => {
      const defaultSetting = screen.getByText('All video files').closest('.setting-item');
      expect(defaultSetting).toHaveTextContent('System Default');
    });
  });

  it('calls invoke to load settings on mount', async () => {
    render(<Settings />);

    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith('get_video_player_settings');
    });
  });

  it('opens file picker when change player button is clicked', async () => {
    const user = userEvent.setup();
    vi.mocked(open).mockResolvedValue('/Applications/IINA.app');

    render(<Settings />);

    await waitFor(() => {
      expect(screen.getByText('All video files')).toBeInTheDocument();
    });

    const changeButton = screen.getByText('Change Player');
    await user.click(changeButton);

    await waitFor(() => {
      expect(open).toHaveBeenCalledWith({
        multiple: false,
        directory: false,
        filters: [{ name: 'Applications', extensions: ['app', 'exe'] }],
      });
    });
  });

  it('updates setting when player is selected', async () => {
    const user = userEvent.setup();
    vi.mocked(open).mockResolvedValue('/Applications/IINA.app');
    vi.mocked(invoke).mockImplementation((cmd) => {
      if (cmd === 'get_video_player_settings') {
        return Promise.resolve(mockSettings);
      }
      if (cmd === 'set_video_player_setting') {
        return Promise.resolve(undefined);
      }
      return Promise.resolve(undefined);
    });

    render(<Settings />);

    await waitFor(() => {
      expect(screen.getByText('All video files')).toBeInTheDocument();
    });

    const changeButton = screen.getByText('Change Player');
    await user.click(changeButton);

    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith('set_video_player_setting', {
        fileExtension: '*',
        playerPath: '/Applications/IINA.app',
      });
    });
  });

  it('can add new extension-specific setting', async () => {
    const user = userEvent.setup();
    vi.mocked(open).mockResolvedValue('/Applications/VLC.app');

    render(<Settings />);

    await waitFor(() => {
      expect(screen.getByText('Add Extension-specific Player')).toBeInTheDocument();
    });

    // Select extension
    const select = screen.getByRole('combobox');
    await user.selectOptions(select, 'mkv');

    // Click "Select Player" button to open file picker
    const selectPlayerButton = screen.getByText('Select Player');
    await user.click(selectPlayerButton);

    // File picker sets newPlayerPath, now click "Add Setting"
    await waitFor(() => {
      const addButton = screen.getByText('Add Setting');
      expect(addButton).not.toBeDisabled();
    });

    const addButton = screen.getByText('Add Setting');
    await user.click(addButton);

    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith('set_video_player_setting', {
        fileExtension: 'mkv',
        playerPath: '/Applications/VLC.app',
      });
    });
  });

  it('deletes setting when delete button is clicked', async () => {
    const user = userEvent.setup();
    const confirmMock = vi.spyOn(window, 'confirm').mockReturnValue(true);

    render(<Settings />);

    await waitFor(() => {
      expect(screen.getByText('.mp4')).toBeInTheDocument();
    });

    const deleteButtons = screen.getAllByText('Delete');
    await user.click(deleteButtons[0]);

    await waitFor(() => {
      expect(confirmMock).toHaveBeenCalled();
      expect(invoke).toHaveBeenCalledWith('delete_video_player_setting', {
        fileExtension: 'mp4',
      });
    });

    confirmMock.mockRestore();
  });

  it('does not delete when confirmation is cancelled', async () => {
    const user = userEvent.setup();
    const confirmMock = vi.spyOn(window, 'confirm').mockReturnValue(false);

    render(<Settings />);

    await waitFor(() => {
      expect(screen.getByText('.mp4')).toBeInTheDocument();
    });

    const deleteButtons = screen.getAllByText('Delete');
    await user.click(deleteButtons[0]);

    expect(confirmMock).toHaveBeenCalled();
    expect(invoke).not.toHaveBeenCalledWith('delete_video_player_setting', expect.any(Object));

    confirmMock.mockRestore();
  });

  it('handles file picker cancellation gracefully', async () => {
    const user = userEvent.setup();
    vi.mocked(open).mockResolvedValue(null);

    render(<Settings />);

    await waitFor(() => {
      expect(screen.getByText('All video files')).toBeInTheDocument();
    });

    const changeButton = screen.getByText('Change Player');
    const invokeCallCount = vi.mocked(invoke).mock.calls.length;

    await user.click(changeButton);

    await waitFor(() => {
      // Should not call set_video_player_setting when cancelled
      expect(vi.mocked(invoke).mock.calls.length).toBe(invokeCallCount);
    });
  });

  it('displays all supported extensions in dropdown', async () => {
    render(<Settings />);

    await waitFor(() => {
      const select = screen.getByRole('combobox');
      expect(select).toBeInTheDocument();
    });

    const options = screen.getAllByRole('option');
    const extensionTexts = options.map(opt => opt.textContent);

    // Extensions in the dropdown have leading dots
    expect(extensionTexts).toContain('.mov');
    expect(extensionTexts).toContain('.avi');
    expect(extensionTexts).toContain('.mkv');
    expect(extensionTexts).toContain('.wmv');
    expect(extensionTexts).toContain('.flv');
    expect(extensionTexts).toContain('.webm');
    expect(extensionTexts).toContain('.m4v');
    // mp4 is already in use so it's filtered out
  });

  it('does not show delete button for default setting', async () => {
    render(<Settings />);

    await waitFor(() => {
      const defaultItem = screen.getByText('All video files').closest('.setting-item');
      const deleteButton = defaultItem?.querySelector('.btn-delete');

      // Default setting should not have a delete button
      expect(deleteButton).toBeNull();
    });
  });
});
