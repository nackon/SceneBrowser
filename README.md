# SceneBrowser

[![CI](https://github.com/nackon/SceneBrowser/actions/workflows/ci.yml/badge.svg)](https://github.com/nackon/SceneBrowser/actions/workflows/ci.yml)
[![Release](https://github.com/nackon/SceneBrowser/actions/workflows/release.yml/badge.svg)](https://github.com/nackon/SceneBrowser/actions/workflows/release.yml)

A macOS video collection management application with multiple thumbnail preview.

## Download

### Stable Release
Download the latest stable version from [GitHub Releases](https://github.com/nackon/SceneBrowser/releases/latest).

### Development Builds
Development builds are automatically created for every PR and push to main. These include the latest features but may be unstable.

To download a development build:
1. Go to [Actions](https://github.com/nackon/SceneBrowser/actions/workflows/ci.yml)
2. Click on a successful workflow run
3. Download the `scenebrowser-macos-dev-*` artifact from the "Artifacts" section
4. Unzip and install the DMG file

**Note:** Development builds are retained for 30 days.

## Quick Start

### Prerequisites

1. **Rust** - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. **FFmpeg** - `brew install ffmpeg`
3. **Node.js 18+**

📖 **詳細なセットアップ手順は [SETUP.md](doc/SETUP.md) を参照してください**

### Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Build

```bash
# Build for production
npm run tauri build
```

Output: `src-tauri/target/release/bundle/dmg/SceneBrowser_*.dmg`

## Project Structure

```
SceneBrowser/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── services/           # Tauri command wrappers
│   ├── types/              # TypeScript types
│   ├── hooks/              # Custom React hooks
│   └── store/              # Zustand state management
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri commands
│   │   ├── services/       # Business logic
│   │   ├── models/         # Data models
│   │   └── utils/          # Utilities
│   └── migrations/         # Database migrations
└── doc/                    # Documentation
```

## Features (Phase 1 MVP)

- ✅ Folder registration and scanning
- ✅ Video metadata extraction (ffprobe)
- ✅ 4x4 thumbnail grid generation (ffmpeg)
- ✅ SQLite database with connection pooling
- ✅ Virtual scrolling for large collections
- ✅ Progress tracking for long operations
- ✅ Type-safe frontend-backend communication

## Technology Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri 2.0
- **Database**: SQLite + sqlx
- **State Management**: Zustand
- **Virtual Scrolling**: react-window
- **Video Processing**: FFmpeg/FFprobe

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
- [Tauri Extension](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint)

## License

See [LICENSE](LICENSE) file for details.
