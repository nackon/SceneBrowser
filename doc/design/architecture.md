# SceneBrowser Architecture Design Document

## 1. System Overview

SceneBrowserは、Tauri、Rust、React、TypeScriptを使用したデスクトップアプリケーションです。大量の動画ファイルを効率的に管理し、複数サムネイルによる視覚的なブラウジング体験を提供します。

### 1.1 Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    Frontend (React)                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ Components   │  │ State (Zustand)│ │ Services     │  │
│  │ - VideoGrid  │  │ - videoStore  │  │ - commands.ts│  │
│  │ - VideoCard  │  │               │  │              │  │
│  │ - Sidebar    │  │               │  │              │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
                           │ Tauri IPC
                           ▼
┌─────────────────────────────────────────────────────────┐
│                  Backend (Rust/Tauri)                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │  Commands    │  │   Services   │  │   Models     │  │
│  │ - folder.rs  │  │ - database   │  │ - video.rs   │  │
│  │ - video.rs   │  │ - scanner    │  │ - folder.rs  │  │
│  │ - thumbnail  │  │ - metadata   │  │              │  │
│  │              │  │ - thumbnail  │  │              │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│                           │                              │
│                           ▼                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   SQLite     │  │  FFmpeg      │  │  File System │  │
│  │   Database   │  │  FFprobe     │  │              │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
```

## 2. Technology Stack

### 2.1 Frontend
- **Framework**: React 18+
- **Language**: TypeScript 5+
- **State Management**: Zustand
- **Virtual Scrolling**: react-window
- **Auto Sizing**: react-virtualized-auto-sizer
- **Build Tool**: Vite 5+
- **Styling**: CSS Modules

### 2.2 Backend
- **Framework**: Tauri 2.0
- **Language**: Rust 1.75+
- **Database**: SQLite via sqlx
- **Async Runtime**: Tokio
- **Serialization**: serde + serde_json
- **File Walking**: walkdir
- **Hashing**: sha2 + hex

### 2.3 External Dependencies
- **Video Processing**: FFmpeg 6.0+
- **Metadata Extraction**: FFprobe 6.0+

## 3. Data Flow

### 3.1 Folder Registration Flow
```
User Action (Add Folder)
    ↓
Frontend: addFolder() call
    ↓
Tauri IPC: add_folder command
    ↓
Backend: FolderCommand::add_folder
    ↓
Database: INSERT INTO folders
    ↓
Return folder_id
    ↓
Frontend: Update folder list
```

### 3.2 Video Scan Flow
```
User Action (Scan Folder)
    ↓
Frontend: scanFolder() with progress callback
    ↓
Tauri IPC: scan_folder command
    ↓
Backend: VideoCommand::scan_folder
    ↓
VideoScanner: Scan directory recursively
    ↓
For each video file:
    ├─ MetadataExtractor: Extract metadata (ffprobe)
    ├─ Hashing: Compute SHA256 hash
    ├─ Database: INSERT/UPDATE video record
    └─ Progress Event: Emit to frontend
    ↓
Return ScanResult
    ↓
Frontend: Update video list
```

### 3.3 Thumbnail Generation Flow
```
Frontend: Display VideoCard
    ↓
Check if thumbnail_path exists
    ↓ No
Frontend: generateThumbnail() call
    ↓
Tauri IPC: generate_thumbnail command
    ↓
Backend: ThumbnailCommand::generate_thumbnail
    ↓
ThumbnailGenerator:
    ├─ Check cache (hash-based filename)
    ├─ If not exists: Generate with ffmpeg
    └─ Return thumbnail path
    ↓
Database: UPDATE video.thumbnail_path
    ↓
Frontend: Display thumbnail
```

## 4. Database Schema

### 4.1 ER Diagram
```
┌──────────────┐         ┌──────────────┐
│   folders    │1      N │    videos    │
│──────────────│◄────────│──────────────│
│ id (PK)      │         │ id (PK)      │
│ path         │         │ folder_id(FK)│
│ recursive    │         │ path         │
│ created_at   │         │ filename     │
│ updated_at   │         │ hash         │
└──────────────┘         │ duration     │
                         │ width        │
                         │ height       │
                         │ size         │
                         │ codec        │
                         │ framerate    │
                         │ thumbnail_*  │
                         │ rating       │
                         │ *_at         │
                         └──────────────┘
                                │
                                │M
                                │
                                │N
                         ┌──────────────┐
                         │ video_tags   │
                         │──────────────│
                         │ video_id(FK) │
                         │ tag_id (FK)  │
                         └──────────────┘
                                │N
                                │
                                │1
                         ┌──────────────┐
                         │     tags     │
                         │──────────────│
                         │ id (PK)      │
                         │ name         │
                         │ color        │
                         └──────────────┘
```

### 4.2 Index Strategy
- **Primary Keys**: All tables have auto-increment INTEGER primary key
- **Foreign Keys**: CASCADE DELETE for data integrity
- **Search Optimization**:
  - `idx_videos_folder_id`: Folder filtering
  - `idx_videos_path`: Duplicate detection
  - `idx_videos_hash`: Duplicate detection
  - `idx_videos_created_at`: Sorting by date

### 4.3 Data Integrity Rules
1. **Unique Constraints**:
   - `folders.path`: No duplicate folder paths
   - `videos.path`: No duplicate video paths
   - `tags.name`: No duplicate tag names

2. **Cascading Deletes**:
   - Delete folder → Delete all associated videos
   - Delete video → Delete all video_tags associations
   - Delete tag → Delete all video_tags associations

## 5. Component Architecture

### 5.1 Frontend Component Tree
```
App
├── Sidebar
│   ├── SearchBar
│   └── FolderList
│       └── FolderItem (repeat)
└── MainContent
    └── VideoGrid (react-window)
        └── VideoCard (repeat, virtualized)
```

### 5.2 Backend Module Structure
```
src-tauri/src/
├── main.rs              # Entry point, app setup
├── commands/
│   ├── mod.rs           # Module exports
│   ├── folder.rs        # Folder commands
│   ├── video.rs         # Video commands
│   └── thumbnail.rs     # Thumbnail commands
├── services/
│   ├── mod.rs           # Module exports
│   ├── database.rs      # SQLite operations
│   ├── video_scanner.rs # Directory scanning
│   ├── metadata.rs      # FFprobe integration
│   └── thumbnail_gen.rs # FFmpeg thumbnail gen
├── models/
│   ├── mod.rs           # Module exports
│   ├── folder.rs        # Folder model
│   └── video.rs         # Video model
├── utils/
│   ├── mod.rs           # Module exports
│   ├── paths.rs         # Path utilities
│   ├── ffmpeg.rs        # FFmpeg helpers
│   └── hashing.rs       # Hash computation
└── error.rs             # Error types
```

## 6. State Management

### 6.1 Frontend State (Zustand)
```typescript
interface VideoStore {
  // Data
  videos: Video[];
  folders: Folder[];
  selectedFolder: number | null;
  searchQuery: string;
  
  // UI State
  isLoading: boolean;
  error: string | null;
  
  // Actions
  setVideos: (videos: Video[]) => void;
  addVideos: (videos: Video[]) => void;
  setFolders: (folders: Folder[]) => void;
  setSelectedFolder: (id: number | null) => void;
  setSearchQuery: (query: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}
```

### 6.2 Backend State (Tauri Managed State)
```rust
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
}
```

State is initialized in `main.rs` setup hook and injected into commands via Tauri's state management.

## 7. Performance Optimization

### 7.1 Frontend Optimizations
1. **Virtual Scrolling**: Only render visible items (react-window)
2. **Lazy Loading**: Load thumbnails on demand
3. **Image Loading**: Use `loading="lazy"` attribute
4. **Pagination**: Load videos in batches (1000 items)
5. **Debouncing**: Debounce search input

### 7.2 Backend Optimizations
1. **Database Indexing**: Indexes on frequently queried columns
2. **Connection Pooling**: SQLx connection pool
3. **Async Operations**: All I/O operations are async
4. **Batch Processing**: Process videos in parallel where possible
5. **Caching**: Thumbnail cache prevents regeneration

### 7.3 Memory Management
- **Frontend**: Virtual scrolling limits DOM nodes
- **Backend**: Stream large result sets instead of loading all at once
- **Database**: Use LIMIT/OFFSET for pagination
- **Thumbnails**: Generate on-demand, not all upfront

## 8. Error Handling

### 8.1 Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("FFmpeg not found")]
    FFmpegNotFound,
    
    #[error("FFmpeg execution failed: {0}")]
    FFmpegExecution(String),
    
    #[error("Invalid video file: {0}")]
    InvalidVideo(String),
    
    #[error("Video not found: {0}")]
    VideoNotFound(i64),
    
    #[error("Folder not found: {0}")]
    FolderNotFound(i64),
}
```

### 8.2 Error Propagation
```
Backend Error
    ↓
Convert to String (Tauri IPC requirement)
    ↓
Return Err(error_message)
    ↓
Frontend catch
    ↓
Display user-friendly message
```

## 9. Security Considerations

### 9.1 Path Validation
- Validate all file paths to prevent path traversal
- Use canonical paths
- Restrict access to registered folders only

### 9.2 Command Injection Prevention
- Never construct FFmpeg commands with string concatenation
- Use Command builder with separate arguments
- Validate all user inputs

### 9.3 Database Security
- Use parameterized queries (SQLx prevents SQL injection)
- No raw SQL string concatenation
- Validate foreign key constraints

### 9.4 Tauri Security
- Minimal CSP policy
- Only expose necessary commands
- Validate all command parameters

## 10. Testing Strategy

### 10.1 Frontend Testing
- **Unit Tests**: Component logic testing
- **Integration Tests**: State management + commands
- **E2E Tests**: User flow testing (future)

### 10.2 Backend Testing
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_database_operations() { }
    
    #[test]
    fn test_video_scanner() { }
    
    #[test]
    fn test_hash_computation() { }
}
```

### 10.3 Integration Testing
- Manual testing scenarios (see doc/tasks/07-integration-testing.md)
- Performance benchmarking
- Memory leak detection

## 11. Deployment

### 11.1 Build Process
```bash
# Development
npm run tauri dev

# Production build
npm run tauri build
```

### 11.2 Platform-Specific
- **macOS**: `.dmg` installer
- **Bundle**: Application bundle with embedded FFmpeg
- **Signing**: Code signing for distribution (future)

### 11.3 Distribution
- Phase 1: Manual distribution
- Future: GitHub Releases, Auto-update

## 12. Monitoring and Logging

### 12.1 Backend Logging
```rust
use log::{info, warn, error, debug};

info!("Scanning folder: {}", path);
warn!("FFmpeg not found, using system version");
error!("Failed to generate thumbnail: {}", err);
```

### 12.2 Frontend Logging
```typescript
console.log('[SceneBrowser] Loading videos...');
console.error('[SceneBrowser] Failed to scan folder:', error);
```

### 12.3 Performance Metrics
- Track scan duration
- Track thumbnail generation time
- Monitor memory usage
- FPS monitoring for virtual scroll

## 13. Future Extensions (Phase 2+)

### 13.1 Tag Management
- Tag CRUD operations
- Tag filtering
- Tag color customization

### 13.2 Rating System
- 5-star rating
- Sorting by rating
- Quick rating shortcuts

### 13.3 Advanced Features
- AI-based scene detection
- OCR for text extraction
- Smart thumbnail selection
- NAS server mode
- Mobile viewer

## 14. Configuration

### 14.1 App Configuration (`tauri.conf.json`)
```json
{
  "productName": "SceneBrowser",
  "version": "0.1.0",
  "identifier": "com.scenebrowser.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  }
}
```

### 14.2 User Settings (Future)
```json
{
  "thumbnail_count": 16,
  "cache_size_limit_mb": 10000,
  "auto_scan_on_startup": false,
  "theme": "light"
}
```

## 15. Maintenance

### 15.1 Database Migrations
- Use SQLx migrations system
- Version-controlled migration files
- Auto-run on app startup

### 15.2 Cache Management
- LRU eviction policy (future)
- Manual cache clear option
- Cache size monitoring

### 15.3 Updates
- Check for updates (future)
- Auto-update mechanism (future)
- Changelog display

---

## Summary

This architecture provides:
- **Scalability**: Handles 100k+ videos
- **Performance**: Virtual scrolling, caching, indexing
- **Maintainability**: Clear separation of concerns
- **Extensibility**: Easy to add features (tags, ratings, AI)
- **Security**: Input validation, safe command execution
- **User Experience**: Fast, responsive, intuitive

The architecture follows Tauri best practices and leverages Rust's performance with React's flexibility for a robust desktop application.
