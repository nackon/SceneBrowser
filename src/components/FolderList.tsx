import { useState, useEffect } from 'react';
import { open, confirm } from '@tauri-apps/plugin-dialog';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { getFolders, addFolder, scanFolder, removeFolder, generateThumbnailsBatch, getVideos } from '../services/commands';
import { useVideoStore } from '../store/videoStore';
import type { Folder } from '../types/video';
import './FolderList.css';

interface DeleteProgress {
  phase: string;
  current: number;
  total: number;
}

export function FolderList() {
  const [folders, setFolders] = useState<Folder[]>([]);
  const [scanning, setScanning] = useState(false);
  const [generatingThumbnails, setGeneratingThumbnails] = useState<number | null>(null);
  const [thumbnailProgress, setThumbnailProgress] = useState<{ current: number; total: number } | null>(null);
  const [deleting, setDeleting] = useState<number | null>(null);
  const [deleteProgress, setDeleteProgress] = useState<DeleteProgress | null>(null);
  const { selectedFolder, setSelectedFolder, setScanProgress, setVideos, setIsLoading } = useVideoStore();

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
        const folderId = await addFolder(selected, true);
        await loadFolders();

        // Automatically scan and generate thumbnails for newly added folder
        await handleScanFolder(folderId);
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

      // Select folder first (this will trigger useVideos hook to fetch videos)
      setSelectedFolder(folderId);

      // Automatically generate thumbnails after scan completes
      // Always attempt generation - the backend will skip videos that already have thumbnails
      await handleGenerateThumbnails(folderId);
    } catch (error) {
      console.error('Failed to scan folder:', error);
      setIsLoading(false);
    } finally {
      setScanning(false);
    }
  }

  async function handleGenerateThumbnails(folderId: number) {
    console.log('handleGenerateThumbnails called with folderId:', folderId);
    setGeneratingThumbnails(folderId);
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
      setGeneratingThumbnails(null);
    }
  }

  async function handleRemoveFolder(folderId: number, folderPath: string) {
    const confirmed = await confirm(
      `Are you sure you want to remove this folder?\n\n${folderPath}\n\nThis will delete all thumbnails and database entries.`,
      { title: 'Remove Folder', kind: 'warning' }
    );

    if (!confirmed) {
      return;
    }

    setDeleting(folderId);
    setDeleteProgress(null);

    let unlisten: UnlistenFn | null = null;

    try {
      unlisten = await listen<DeleteProgress>('delete_progress', (event) => {
        setDeleteProgress(event.payload);
      });

      await removeFolder(folderId);

      // Clear selection if deleting the selected folder
      if (selectedFolder === folderId) {
        setSelectedFolder(null);
        setVideos([]);
      }

      await loadFolders();
    } catch (error) {
      console.error('Failed to remove folder:', error);
    } finally {
      if (unlisten) {
        unlisten();
      }
      setDeleting(null);
      setDeleteProgress(null);
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
                  disabled={scanning || deleting !== null}
                  className="btn-scan"
                  title="Scan folder"
                >
                  🔄
                </button>
                <button
                  onClick={() => handleGenerateThumbnails(folder.id)}
                  disabled={generatingThumbnails !== null || deleting !== null}
                  className="btn-thumbnail"
                  title="Generate thumbnails"
                >
                  {generatingThumbnails === folder.id ? '⏳' : '🖼️'}
                </button>
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    handleRemoveFolder(folder.id, folder.path);
                  }}
                  disabled={scanning || deleting !== null}
                  className="btn-delete"
                  title="Remove folder"
                >
                  🗑️
                </button>
              </div>
              {generatingThumbnails === folder.id && thumbnailProgress && (
                <div className="folder-thumbnail-progress">
                  <div className="folder-progress-text">
                    Generating thumbnails... {thumbnailProgress.current}/{thumbnailProgress.total}
                  </div>
                  <div className="folder-progress-bar">
                    <div
                      className="folder-progress-fill"
                      style={{
                        width: `${(thumbnailProgress.current / thumbnailProgress.total) * 100}%`,
                      }}
                    />
                  </div>
                </div>
              )}
              {deleting === folder.id && deleteProgress && (
                <div className="delete-progress">
                  <div className="delete-progress-text">
                    {deleteProgress.phase}
                    {deleteProgress.total > 0 && (
                      <span className="delete-progress-count">
                        {' '}
                        ({deleteProgress.current}/{deleteProgress.total})
                      </span>
                    )}
                  </div>
                  {deleteProgress.total > 0 && (
                    <div className="delete-progress-bar">
                      <div
                        className="delete-progress-fill"
                        style={{
                          width: `${(deleteProgress.current / deleteProgress.total) * 100}%`,
                        }}
                      />
                    </div>
                  )}
                </div>
              )}
            </li>
          ))}
        </ul>
      )}

      {generatingThumbnails !== null && thumbnailProgress && (
        <div className="thumbnail-progress">
          <div className="progress-text">
            Generating thumbnails... {thumbnailProgress.current}/{thumbnailProgress.total}
          </div>
          <div className="progress-bar">
            <div
              className="progress-fill"
              style={{
                width: `${(thumbnailProgress.current / thumbnailProgress.total) * 100}%`,
              }}
            />
          </div>
        </div>
      )}
    </div>
  );
}
