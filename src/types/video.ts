export interface Video {
  id: number;
  folder_id: number;
  path: string;
  filename: string;
  hash: string;
  duration: number; // seconds
  width: number; // pixels
  height: number; // pixels
  size: number; // bytes
  codec: string | null;
  framerate: number | null; // fps
  thumbnail_path: string | null;
  thumbnail_count: number;
  rating: number; // 0-5
  is_favorite: number; // 0 or 1
  created_at: string; // ISO 8601
  updated_at: string; // ISO 8601
  scanned_at: string; // ISO 8601
}

export interface Folder {
  id: number;
  path: string;
  recursive: boolean;
  created_at: string; // ISO 8601
  updated_at: string; // ISO 8601
}

export interface ScanResult {
  videos_found: number;
  videos_added: number;
  videos_updated: number;
  errors: string[];
}

export interface ScanProgress {
  current: number;
  total: number;
  current_file: string;
}

export interface VideoMetadata {
  duration: number;
  width: number;
  height: number;
  codec: string;
  framerate: number;
}
