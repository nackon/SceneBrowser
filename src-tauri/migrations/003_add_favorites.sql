-- Add favorites support to videos table
-- This migration adds is_favorite column to track favorite videos

-- Note: This migration needs to be applied to per-folder databases
-- It will be executed by database.rs when opening folder databases

-- Add is_favorite column (0 = not favorite, 1 = favorite)
ALTER TABLE videos ADD COLUMN is_favorite INTEGER NOT NULL DEFAULT 0;

-- Create index for faster favorite filtering
CREATE INDEX IF NOT EXISTS idx_videos_favorite ON videos(is_favorite);
