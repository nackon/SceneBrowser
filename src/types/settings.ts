export interface VideoPlayerSetting {
  id: number;
  file_extension: string; // "*" for default, or specific extension like "mp4"
  player_path: string; // Empty string means system default
  created_at: string;
  updated_at: string;
}

export interface VideoPlayerSettingInput {
  file_extension: string;
  player_path: string;
}
