-- SceneBrowser Global Database Schema
-- Created: 2026-05-13
-- Updated: 2026-05-13 (Refactored to per-folder architecture)
--
-- This is the GLOBAL database that stores only the folder list.
-- Video metadata is stored in per-folder databases at <folder>/.scenebrowser/db.sqlite

-- folders: 登録フォルダ管理
CREATE TABLE IF NOT EXISTS folders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    recursive BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
