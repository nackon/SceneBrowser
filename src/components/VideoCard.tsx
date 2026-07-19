import { useState, useEffect, useRef } from 'react';
import { createPortal } from 'react-dom';
import { invoke } from '@tauri-apps/api/core';
import { readThumbnail, regenerateThumbnail, toggleFavorite } from '../services/commands';
import { useVideoStore } from '../store/videoStore';
import type { Video } from '../types/video';
import './VideoCard.css';

interface VideoCardProps {
  video: Video;
  folderId: number | null;
  onThumbnailRegenerated?: () => void;
  onFavoriteToggled?: () => void;
}

export function VideoCard({ video, folderId, onThumbnailRegenerated, onFavoriteToggled }: VideoCardProps) {
  const updateVideoFavorite = useVideoStore((state) => state.updateVideoFavorite);
  const updateVideoThumbnail = useVideoStore((state) => state.updateVideoThumbnail);
  const [thumbnailUrl, setThumbnailUrl] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);
  const [regenerating, setRegenerating] = useState(false);
  const [isFavorite, setIsFavorite] = useState(video.is_favorite === 1);
  // The grid virtualizer recycles VideoCard instances across different videos as it
  // scrolls/filters, so an in-flight thumbnail read for a previous video must not be
  // allowed to overwrite state after this instance has been reassigned to a new one.
  // video.id alone isn't a safe guard: ids are only unique within a folder, so a
  // recycled card can be reassigned to a same-numbered video in a different folder -
  // folderId must match too.
  const videoIdRef = useRef(video.id);
  videoIdRef.current = video.id;
  const folderIdRef = useRef(folderId);
  folderIdRef.current = folderId;
  const [contextMenuPos, setContextMenuPos] = useState<{ x: number; y: number } | null>(null);
  const contextMenuRef = useRef<HTMLDivElement>(null);

  // Reset per-video, non-thumbnail state whenever a recycled card is reassigned
  // to a different video, so stale flags/values from the previous video can't leak in.
  useEffect(() => {
    setIsFavorite(video.is_favorite === 1);
    setRegenerating(false);
    setContextMenuPos(null);
  }, [video.id]);

  useEffect(() => {
    if (!contextMenuPos) return;

    function handleOutsideEvent(e: MouseEvent | KeyboardEvent) {
      if (e instanceof KeyboardEvent) {
        if (e.key === 'Escape') setContextMenuPos(null);
        return;
      }
      if (contextMenuRef.current && !contextMenuRef.current.contains(e.target as Node)) {
        setContextMenuPos(null);
      }
    }

    document.addEventListener('mousedown', handleOutsideEvent);
    document.addEventListener('keydown', handleOutsideEvent);
    return () => {
      document.removeEventListener('mousedown', handleOutsideEvent);
      document.removeEventListener('keydown', handleOutsideEvent);
    };
  }, [contextMenuPos]);

  useEffect(() => {
    let cancelled = false;

    async function load(thumbnailPath: string) {
      setLoading(true);
      setError(false);
      try {
        const dataUrl = await readThumbnail(thumbnailPath);
        if (!cancelled) setThumbnailUrl(dataUrl);
      } catch (err) {
        if (!cancelled) {
          console.error('Failed to load thumbnail:', err);
          setError(true);
        }
      } finally {
        if (!cancelled) setLoading(false);
      }
    }

    if (video.thumbnail_path) {
      load(video.thumbnail_path);
    } else {
      setThumbnailUrl(null);
      setLoading(false);
      setError(false);
    }

    return () => {
      cancelled = true;
    };
  }, [video.id, video.thumbnail_path]);

  async function handleRegenerateThumbnail(e: React.MouseEvent) {
    e.stopPropagation();
    if (folderId === null) {
      alert('Please select a folder first');
      return;
    }
    const videoId = video.id;
    const requestFolderId = folderId;
    const isSameCard = () =>
      videoIdRef.current === videoId && folderIdRef.current === requestFolderId;
    setRegenerating(true);
    setError(false);
    try {
      const thumbnailPath = await regenerateThumbnail(requestFolderId, videoId);
      const dataUrl = await readThumbnail(thumbnailPath);
      if (isSameCard()) {
        updateVideoThumbnail(videoId, thumbnailPath);
        setThumbnailUrl(dataUrl);
      }
      if (onThumbnailRegenerated) {
        onThumbnailRegenerated();
      }
    } catch (err) {
      console.error('Failed to regenerate thumbnail:', err);
      if (isSameCard()) {
        setError(true);
      }
    } finally {
      if (isSameCard()) {
        setRegenerating(false);
      }
    }
  }

  const formatDuration = (seconds: number) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    if (hours > 0) {
      return `${hours}:${minutes.toString().padStart(2, '0')}:${secs
        .toString()
        .padStart(2, '0')}`;
    }
    return `${minutes}:${secs.toString().padStart(2, '0')}`;
  };

  const formatSize = (bytes: number) => {
    const gb = bytes / 1024 ** 3;
    if (gb >= 1) return `${gb.toFixed(2)} GB`;
    const mb = bytes / 1024 ** 2;
    return `${mb.toFixed(2)} MB`;
  };

  const handleClick = async () => {
    console.log('Video card clicked, path:', video.path);
    try {
      console.log('Opening video with configured player...');
      await invoke('open_video_with_player', { videoPath: video.path });
      console.log('Video opened successfully');
    } catch (err) {
      console.error('Failed to open video:', err);
      alert(`Failed to open video: ${err}`);
    }
  };

  const handleContextMenu = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setContextMenuPos({ x: e.clientX, y: e.clientY });
  };

  const handleCopyPath = async (e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      await navigator.clipboard.writeText(video.path);
    } catch (err) {
      console.error('Failed to copy path:', err);
    }
    setContextMenuPos(null);
  };

  const handleFavoriteClick = async (e: React.MouseEvent) => {
    e.stopPropagation();
    if (folderId === null) {
      alert('Please select a folder first');
      return;
    }
    try {
      const newStatus = await toggleFavorite(folderId, video.id);
      setIsFavorite(newStatus);
      updateVideoFavorite(video.id, newStatus);
      if (onFavoriteToggled) {
        onFavoriteToggled();
      }
    } catch (err) {
      console.error('Failed to toggle favorite:', err);
      alert(`Failed to toggle favorite: ${err}`);
    }
  };

  return (
    <div className="video-card" onClick={handleClick}>
      <div className="thumbnail-container" onContextMenu={handleContextMenu}>
        {loading ? (
          <div className="thumbnail-loading">
            <div className="spinner-small"></div>
          </div>
        ) : error ? (
          <div className="thumbnail-error">
            <span>❌</span>
            <p>Failed to load</p>
          </div>
        ) : thumbnailUrl ? (
          <img
            src={thumbnailUrl}
            alt={video.filename}
            loading="lazy"
            className="thumbnail"
          />
        ) : (
          <div className="thumbnail-placeholder">🎬</div>
        )}
        {regenerating && (
          <div className="thumbnail-regenerating-overlay">
            <div className="spinner-small"></div>
            <p>Regenerating...</p>
          </div>
        )}
        <div className="duration-badge">{formatDuration(video.duration)}</div>
        <button
          className={`favorite-button ${isFavorite ? 'favorite' : ''}`}
          onClick={handleFavoriteClick}
          title={isFavorite ? 'Remove from favorites' : 'Add to favorites'}
        >
          {isFavorite ? '★' : '☆'}
        </button>
        <button
          className="regenerate-button"
          onClick={handleRegenerateThumbnail}
          disabled={regenerating}
          title="Regenerate thumbnail"
          aria-label="Regenerate thumbnail"
        >
          🔄
        </button>
      </div>
      <div className="video-info">
        <h3 className="filename" title={video.filename}>
          {video.filename}
        </h3>
        <div className="metadata">
          <span className="resolution">
            {video.width}x{video.height}
          </span>
          <span className="size">{formatSize(video.size)}</span>
          {video.codec && <span className="codec">{video.codec}</span>}
        </div>
      </div>
      {contextMenuPos &&
        createPortal(
          <div
            ref={contextMenuRef}
            className="context-menu"
            style={{ left: contextMenuPos.x, top: contextMenuPos.y }}
          >
            <button className="context-menu-item" onClick={handleCopyPath}>
              Copy Path
            </button>
          </div>,
          document.body
        )}
    </div>
  );
}
