# SceneBrowser Phase 1 Tasks

このファイルは、Phase 1実装のタスク一覧です。各タスクの詳細は `doc/tasks/` ディレクトリを参照してください。

## Task Overview

| # | Task | Est. Time | Dependencies | Status |
|---|------|-----------|--------------|--------|
| 1 | [Project Setup](doc/tasks/01-project-setup.md) | 2h | - | 🔲 Todo |
| 2 | [Database Schema](doc/tasks/02-database-schema.md) | 3h | #1 | 🔲 Todo |
| 3 | [Backend Services](doc/tasks/03-backend-services.md) | 8h | #2 | 🔲 Todo |
| 4 | [Tauri Commands](doc/tasks/04-tauri-commands.md) | 5h | #3 | 🔲 Todo |
| 5 | [Frontend Types](doc/tasks/05-frontend-types.md) | 3h | #4 | 🔲 Todo |
| 6 | [Frontend UI](doc/tasks/06-frontend-ui.md) | 8h | #5 | 🔲 Todo |
| 7 | [Integration Testing](doc/tasks/07-integration-testing.md) | 6h | #6 | 🔲 Todo |
| 8 | [FFmpeg Setup](doc/tasks/08-ffmpeg-setup.md) | 3h | #1 | 🔲 Todo |

**Total Estimated Time**: 38 hours

## Task Details

### Task #1: Project Setup and Initialization
- Initialize Tauri project
- Install dependencies (npm + Rust)
- Create directory structure
- Verify build process

### Task #2: Database Schema and Migrations
- Create SQLite migration files
- Define data models (Video, Folder)
- Implement database initialization
- Add indexes

### Task #3: Backend Core Services Implementation
- Database Service (CRUD operations)
- Video Scanner Service
- Metadata Extractor (ffprobe)
- Thumbnail Generator (ffmpeg)
- Utility functions

### Task #4: Tauri Commands Implementation
- Folder commands (add/get/remove)
- Video commands (scan/get/search)
- Thumbnail commands (generate/batch)
- AppState setup

### Task #5: Frontend TypeScript Types and Services
- Type definitions
- Tauri command wrappers
- State management (Zustand)
- Custom hooks

### Task #6: Frontend UI Components
- App layout
- VideoGrid (virtual scrolling)
- VideoCard
- Sidebar
- FolderList
- SearchBar

### Task #7: Integration and Testing
- End-to-end testing
- Performance testing
- Bug fixes
- Documentation

### Task #8: FFmpeg/FFprobe Setup and Integration
- Download binaries
- Configure as sidecars
- Integration with services
- Fallback to system binaries

## GitHub Issue Creation

各タスクをGitHub Issueとして作成するには、以下のコマンドを実行してください：

```bash
# まずGitHub CLIで認証
gh auth login

# 各タスクをIssueとして作成
gh issue create --title "Task #1: Project Setup" --body-file doc/tasks/01-project-setup.md --label "setup,phase-1"
gh issue create --title "Task #2: Database Schema" --body-file doc/tasks/02-database-schema.md --label "database,phase-1"
gh issue create --title "Task #3: Backend Services" --body-file doc/tasks/03-backend-services.md --label "backend,services,phase-1"
gh issue create --title "Task #4: Tauri Commands" --body-file doc/tasks/04-tauri-commands.md --label "backend,commands,phase-1"
gh issue create --title "Task #5: Frontend Types" --body-file doc/tasks/05-frontend-types.md --label "frontend,typescript,phase-1"
gh issue create --title "Task #6: Frontend UI" --body-file doc/tasks/06-frontend-ui.md --label "frontend,ui,react,phase-1"
gh issue create --title "Task #7: Integration Testing" --body-file doc/tasks/07-integration-testing.md --label "testing,integration,phase-1"
gh issue create --title "Task #8: FFmpeg Setup" --body-file doc/tasks/08-ffmpeg-setup.md --label "setup,ffmpeg,phase-1"
```

または、手動でGitHub UIから作成する場合は、各タスクファイルの内容をコピー&ペーストしてください。

## Progress Tracking

- [ ] Task #1: Project Setup (2h)
- [ ] Task #2: Database Schema (3h)
- [ ] Task #3: Backend Services (8h)
- [ ] Task #4: Tauri Commands (5h)
- [ ] Task #5: Frontend Types (3h)
- [ ] Task #6: Frontend UI (8h)
- [ ] Task #7: Integration Testing (6h)
- [ ] Task #8: FFmpeg Setup (3h)

## Documentation

詳細な設計書は以下を参照：
- [Phase 1 Implementation Plan](doc/phase1-implementation-plan.md)
- [Architecture Design](doc/design/architecture.md)
- [API Specification](doc/design/api-specification.md)
