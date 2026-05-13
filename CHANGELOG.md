# Changelog

All notable changes to SceneBrowser will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- CI/CD pipelines with GitHub Actions
- Automated testing on pull requests
- Automated release builds for macOS

## [0.1.0] - 2026-05-13

### Added
- Initial release
- Folder registration and scanning
- Video metadata extraction (ffprobe)
- 4x4 thumbnail grid generation (ffmpeg)
- SQLite database with connection pooling
- Virtual scrolling for large video collections
- Progress tracking for long operations
- Type-safe frontend-backend communication
- Dark theme UI optimized for video browsing
- macOS support (Intel and Apple Silicon)

### Features
- Register multiple folders with recursive scanning
- Automatic video metadata extraction
- Lazy thumbnail generation with caching
- Search videos by filename
- Support for mp4, mkv, avi, mov, webm, m4v formats
- Handles 10,000+ videos with virtual scrolling

### Technical
- Tauri 2.0 framework
- React + TypeScript frontend
- Rust backend
- SQLite database
- FFmpeg/FFprobe integration
- Comprehensive test coverage

[Unreleased]: https://github.com/nackon/SceneBrowser/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/nackon/SceneBrowser/releases/tag/v0.1.0
