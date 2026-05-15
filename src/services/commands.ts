import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { Video, Folder, ScanResult, ScanProgress } from '../types/video';

// --- Folder Commands ---

export async function addFolder(path: string, recursive: boolean): Promise<number> {
  return await invoke<number>('add_folder', { path, recursive });
}

export async function getFolders(): Promise<Folder[]> {
  return await invoke<Folder[]>('get_folders');
}

export async function removeFolder(folderId: number): Promise<void> {
  await invoke('remove_folder', { folderId });
}

// --- Video Commands ---

export async function scanFolder(
  folderId: number,
  onProgress?: (progress: ScanProgress) => void
): Promise<ScanResult> {
  let unlisten: UnlistenFn | null = null;

  if (onProgress) {
    unlisten = await listen<ScanProgress>('scan_progress', (event) => {
      onProgress(event.payload);
    });
  }

  try {
    const result = await invoke<ScanResult>('scan_folder', { folderId });
    return result;
  } finally {
    if (unlisten) {
      unlisten();
    }
  }
}

export async function getVideos(
  folderId: number | null,
  limit: number,
  offset: number
): Promise<Video[]> {
  return await invoke<Video[]>('get_videos', { folderId, limit, offset });
}

export async function searchVideos(query: string): Promise<Video[]> {
  return await invoke<Video[]>('search_videos', { query });
}

export async function getVideoById(videoId: number): Promise<Video> {
  return await invoke<Video>('get_video_by_id', { videoId });
}

// --- Thumbnail Commands ---

export interface ThumbnailProgress {
  current: number;
  total: number;
  current_file: string;
}

export async function generateThumbnail(videoId: number): Promise<string> {
  return await invoke<string>('generate_thumbnail', { videoId });
}

export async function generateThumbnailsBatch(
  folderId: number,
  onProgress?: (progress: ThumbnailProgress) => void
): Promise<number> {
  let unlisten: UnlistenFn | null = null;

  if (onProgress) {
    unlisten = await listen<ThumbnailProgress>('thumbnail_progress', (event) => {
      onProgress(event.payload);
    });
  }

  try {
    const result = await invoke<number>('generate_thumbnails_batch', { folderId });
    return result;
  } finally {
    if (unlisten) {
      unlisten();
    }
  }
}

export async function readThumbnail(thumbnailPath: string): Promise<string> {
  return await invoke<string>('read_thumbnail', { thumbnailPath });
}

// --- System Commands ---

export async function checkFFmpeg(): Promise<void> {
  await invoke('check_ffmpeg');
}
