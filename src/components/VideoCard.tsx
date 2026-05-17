import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { readThumbnail, regenerateThumbnail } from '../services/commands';
import type { Video } from '../types/video';
import './VideoCard.css';

interface VideoCardProps {
  video: Video;
  onThumbnailRegenerated?: () => void;
}

export function VideoCard({ video, onThumbnailRegenerated }: VideoCardProps) {
  const [thumbnailUrl, setThumbnailUrl] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);
  const [regenerating, setRegenerating] = useState(false);

  useEffect(() => {
    if (video.thumbnail_path) {
      // Read thumbnail as base64 data URL
      loadThumbnail(video.thumbnail_path);
    }
  }, [video.id, video.thumbnail_path]);

  async function loadThumbnail(thumbnailPath: string) {
    setLoading(true);
    setError(false);
    try {
      const dataUrl = await readThumbnail(thumbnailPath);
      setThumbnailUrl(dataUrl);
    } catch (err) {
      console.error('Failed to load thumbnail:', err);
      setError(true);
    } finally {
      setLoading(false);
    }
  }

  async function handleRegenerateThumbnail(e: React.MouseEvent) {
    e.stopPropagation();
    setRegenerating(true);
    setError(false);
    try {
      const thumbnailPath = await regenerateThumbnail(video.id);
      const dataUrl = await readThumbnail(thumbnailPath);
      setThumbnailUrl(dataUrl);
      if (onThumbnailRegenerated) {
        onThumbnailRegenerated();
      }
    } catch (err) {
      console.error('Failed to regenerate thumbnail:', err);
      setError(true);
    } finally {
      setRegenerating(false);
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
      console.log('Opening video...');
      await invoke('plugin:opener|open_path', { path: video.path });
      console.log('Video opened successfully');
    } catch (err) {
      console.error('Failed to open video:', err);
      alert(`Failed to open video: ${err}`);
    }
  };

  return (
    <div className="video-card" onClick={handleClick}>
      <div className="thumbnail-container">
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
    </div>
  );
}
