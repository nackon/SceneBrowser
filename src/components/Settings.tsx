import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import type { VideoPlayerSetting } from '../types/settings';
import './Settings.css';

export function Settings() {
  const [settings, setSettings] = useState<VideoPlayerSetting[]>([]);
  const [loading, setLoading] = useState(true);
  const [newExtension, setNewExtension] = useState('');
  const [newPlayerPath, setNewPlayerPath] = useState('');
  const [editingId, setEditingId] = useState<number | null>(null);

  const commonExtensions = ['mp4', 'mov', 'avi', 'mkv', 'wmv', 'flv', 'webm', 'm4v'];

  useEffect(() => {
    loadSettings();
  }, []);

  async function loadSettings() {
    setLoading(true);
    try {
      const result = await invoke<VideoPlayerSetting[]>('get_video_player_settings');
      setSettings(result);
    } catch (err) {
      console.error('Failed to load settings:', err);
      alert(`Failed to load settings: ${err}`);
    } finally {
      setLoading(false);
    }
  }

  async function handleSelectPlayer(extension?: string) {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        filters: [
          {
            name: 'Applications',
            extensions: ['app', 'exe'],
          },
        ],
      });

      if (selected) {
        const path = typeof selected === 'string' ? selected : String(selected);
        if (extension) {
          // Update existing setting
          await updateSetting(extension, path);
        } else {
          // Set for new extension
          setNewPlayerPath(path);
        }
      }
    } catch (err) {
      console.error('Failed to select player:', err);
      alert(`Failed to select player: ${err}`);
    }
  }

  async function updateSetting(extension: string, playerPath: string) {
    try {
      await invoke('set_video_player_setting', {
        fileExtension: extension,
        playerPath,
      });
      await loadSettings();
      setEditingId(null);
    } catch (err) {
      console.error('Failed to update setting:', err);
      alert(`Failed to update setting: ${err}`);
    }
  }

  async function handleAddSetting() {
    if (!newExtension) {
      alert('Please enter a file extension');
      return;
    }

    try {
      await invoke('set_video_player_setting', {
        fileExtension: newExtension.toLowerCase(),
        playerPath: newPlayerPath,
      });
      setNewExtension('');
      setNewPlayerPath('');
      await loadSettings();
    } catch (err) {
      console.error('Failed to add setting:', err);
      alert(`Failed to add setting: ${err}`);
    }
  }

  async function handleDeleteSetting(extension: string) {
    if (!confirm(`Delete setting for .${extension} files?`)) {
      return;
    }

    try {
      await invoke('delete_video_player_setting', {
        fileExtension: extension,
      });
      await loadSettings();
    } catch (err) {
      console.error('Failed to delete setting:', err);
      alert(`Failed to delete setting: ${err}`);
    }
  }

  function getPlayerName(path: string): string {
    if (!path) return 'System Default';
    const parts = path.split('/');
    const appName = parts[parts.length - 1];
    return appName.replace('.app', '').replace('.exe', '');
  }

  if (loading) {
    return (
      <div className="settings-container">
        <div className="spinner"></div>
      </div>
    );
  }

  const defaultSetting = settings.find((s) => s.file_extension === '*');
  const extensionSettings = settings.filter((s) => s.file_extension !== '*');

  return (
    <div className="settings-container">
      <h1>Video Player Settings</h1>

      {/* Default Player */}
      <div className="settings-section">
        <h2>Default Player</h2>
        <div className="setting-item default-setting">
          <div className="setting-info">
            <span className="extension-label">All video files</span>
            <span className="player-path">{getPlayerName(defaultSetting?.player_path || '')}</span>
          </div>
          <button
            className="btn-select-player"
            onClick={() => handleSelectPlayer('*')}
          >
            Change Player
          </button>
        </div>
      </div>

      {/* Extension-specific Players */}
      <div className="settings-section">
        <h2>Extension-specific Players</h2>
        <div className="settings-list">
          {extensionSettings.map((setting) => (
            <div key={setting.id} className="setting-item">
              <div className="setting-info">
                <span className="extension-label">.{setting.file_extension}</span>
                <span className="player-path">{getPlayerName(setting.player_path)}</span>
              </div>
              <div className="setting-actions">
                <button
                  className="btn-select-player"
                  onClick={() => handleSelectPlayer(setting.file_extension)}
                >
                  Change
                </button>
                <button
                  className="btn-delete"
                  onClick={() => handleDeleteSetting(setting.file_extension)}
                >
                  Delete
                </button>
              </div>
            </div>
          ))}
        </div>

        {/* Add New Setting */}
        <div className="add-setting">
          <h3>Add Extension-specific Player</h3>
          <div className="add-setting-form">
            <div className="form-row">
              <label>
                File Extension:
                <select
                  value={newExtension}
                  onChange={(e) => setNewExtension(e.target.value)}
                >
                  <option value="">-- Select Extension --</option>
                  {commonExtensions
                    .filter(
                      (ext) =>
                        !extensionSettings.some((s) => s.file_extension === ext)
                    )
                    .map((ext) => (
                      <option key={ext} value={ext}>
                        .{ext}
                      </option>
                    ))}
                  <option value="custom">Custom...</option>
                </select>
              </label>
              {newExtension === 'custom' && (
                <input
                  type="text"
                  placeholder="Enter extension (e.g., mp4)"
                  onChange={(e) => setNewExtension(e.target.value.toLowerCase())}
                />
              )}
            </div>
            <div className="form-row">
              <label>
                Player:
                <input
                  type="text"
                  value={newPlayerPath}
                  placeholder="Click 'Select Player' to choose..."
                  readOnly
                />
              </label>
              <button
                className="btn-select-player"
                onClick={() => handleSelectPlayer()}
              >
                Select Player
              </button>
            </div>
            <button
              className="btn-add"
              onClick={handleAddSetting}
              disabled={!newExtension || !newPlayerPath}
            >
              Add Setting
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
