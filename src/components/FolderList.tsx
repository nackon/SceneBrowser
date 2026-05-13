import { useState, useEffect } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { getFolders, addFolder, scanFolder, generateThumbnailsBatch } from '../services/commands';
import { useVideoStore } from '../store/videoStore';
import type { Folder } from '../types/video';
import './FolderList.css';

export function FolderList() {
  const [folders, setFolders] = useState<Folder[]>([]);
  const [scanning, setScanning] = useState(false);
  const [generatingThumbnails, setGeneratingThumbnails] = useState(false);
  const [thumbnailProgress, setThumbnailProgress] = useState<{ current: number; total: number } | null>(null);
  const { selectedFolder, setSelectedFolder, setScanProgress, setVideos } = useVideoStore();

  useEffect(() => {
    loadFolders();
  }, []);

  async function loadFolders() {
    try {
      const data = await getFolders();
      setFolders(data);
    } catch (error) {
      console.error('Failed to load folders:', error);
    }
  }

  async function handleAddFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select video folder',
      });

      if (selected && typeof selected === 'string') {
        await addFolder(selected, true);
        await loadFolders();
      }
    } catch (error) {
      console.error('Failed to add folder:', error);
    }
  }

  async function handleScanFolder(folderId: number) {
    setScanning(true);
    try {
      const result = await scanFolder(folderId, (progress) => {
        setScanProgress({
          current: progress.current,
          total: progress.total,
          file: progress.current_file,
        });
      });

      console.log('Scan complete:', result);
      setScanProgress(null);

      // Select this folder and refresh video list
      setSelectedFolder(folderId);
    } catch (error) {
      console.error('Failed to scan folder:', error);
    } finally {
      setScanning(false);
    }
  }

  async function handleGenerateThumbnails(folderId: number) {
    setGeneratingThumbnails(true);
    setThumbnailProgress(null);
    try {
      const generated = await generateThumbnailsBatch(folderId, (progress) => {
        setThumbnailProgress({
          current: progress.current,
          total: progress.total,
        });
      });

      console.log(`Generated ${generated} thumbnails`);
      setThumbnailProgress(null);

      // Refresh video list to show new thumbnails
      if (selectedFolder === folderId) {
        setVideos([]); // Force reload
        setSelectedFolder(folderId);
      }
    } catch (error) {
      console.error('Failed to generate thumbnails:', error);
    } finally {
      setGeneratingThumbnails(false);
    }
  }

  return (
    <div className="folder-list">
      <div className="folder-list-header">
        <h2>Folders</h2>
        <button onClick={handleAddFolder} className="btn-add" title="Add folder">
          +
        </button>
      </div>

      {folders.length === 0 ? (
        <div className="empty-folders">
          <p>No folders added yet</p>
        </div>
      ) : (
        <ul className="folders">
          {folders.map((folder) => (
            <li
              key={folder.id}
              className={`folder-item ${selectedFolder === folder.id ? 'selected' : ''}`}
            >
              <div
                className="folder-info"
                onClick={() => setSelectedFolder(folder.id)}
              >
                <div className="folder-icon">📁</div>
                <div className="folder-details">
                  <div className="folder-path" title={folder.path}>
                    {folder.path.split('/').pop() || folder.path}
                  </div>
                  <div className="folder-full-path">{folder.path}</div>
                </div>
              </div>
              <div className="folder-actions">
                <button
                  onClick={() => handleScanFolder(folder.id)}
                  disabled={scanning}
                  className="btn-scan"
                  title="Scan folder"
                >
                  🔄
                </button>
                <button
                  onClick={() => handleGenerateThumbnails(folder.id)}
                  disabled={generatingThumbnails}
                  className="btn-thumbnail"
                  title="Generate thumbnails"
                >
                  🖼️
                </button>
              </div>
            </li>
          ))}
        </ul>
      )}
      {thumbnailProgress && (
        <div className="thumbnail-progress">
          <div className="progress-text">
            Generating thumbnails... {thumbnailProgress.current}/{thumbnailProgress.total} (
            {Math.round((thumbnailProgress.current / thumbnailProgress.total) * 100)}%)
          </div>
          <div className="progress-bar">
            <div
              className="progress-fill"
              style={{ width: `${(thumbnailProgress.current / thumbnailProgress.total) * 100}%` }}
            />
          </div>
        </div>
      )}
    </div>
  );
}
