# Contributing to SceneBrowser

Thank you for your interest in contributing to SceneBrowser! This document provides guidelines and information for contributors.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Development Workflow](#development-workflow)
3. [Code Standards](#code-standards)
4. [Testing](#testing)
5. [CI/CD](#cicd)
6. [Pull Request Process](#pull-request-process)

## Getting Started

### Prerequisites

See [SETUP.md](doc/SETUP.md) for detailed setup instructions.

Quick setup:
```bash
# Install dependencies
brew install rust ffmpeg node

# Clone and setup
git clone https://github.com/nackon/SceneBrowser.git
cd SceneBrowser
npm install
```

### Running Locally

```bash
npm run tauri:dev
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

Branch naming conventions:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test additions/updates

### 2. Make Changes

- Write clear, concise commit messages
- Keep commits focused and atomic
- Add tests for new functionality
- Update documentation as needed

### 3. Run Tests

```bash
# Rust tests
cd src-tauri && cargo test

# Rust linting
cargo clippy
cargo fmt --check

# Frontend build
npm run build

# TypeScript check
npx tsc --noEmit
```

## Code Standards

### Rust

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

```rust
// Use descriptive names
pub async fn scan_folder(folder_id: i64) -> Result<ScanResult>

// Document public APIs
/// Scans a folder for video files and extracts metadata
pub async fn scan_folder(folder_id: i64) -> Result<ScanResult> {
    // ...
}

// Handle errors properly
match operation() {
    Ok(result) => Ok(result),
    Err(e) => Err(AppError::from(e)),
}
```

**Formatting:**
```bash
cargo fmt
```

**Linting:**
```bash
cargo clippy -- -D warnings
```

### TypeScript

Follow standard TypeScript best practices:

```typescript
// Use interfaces for object shapes
interface Video {
  id: number;
  path: string;
  // ...
}

// Use async/await
async function fetchVideos(): Promise<Video[]> {
  return await invoke('get_videos');
}

// Type function parameters
function processVideo(video: Video): void {
  // ...
}
```

**Type checking:**
```bash
npx tsc --noEmit
```

### React

- Use functional components with hooks
- Keep components small and focused
- Extract reusable logic into custom hooks
- Use meaningful component names

```tsx
// Good
export function VideoCard({ video }: VideoCardProps) {
  const [thumbnail, setThumbnail] = useState<string | null>(null);
  // ...
}

// Custom hooks for shared logic
export function useVideos(folderId: number | null) {
  // ...
}
```

## Testing

### Unit Tests

Write unit tests for:
- Utility functions
- Services
- Business logic

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_scanner_detects_videos() {
        // Arrange
        let temp_dir = create_test_directory();
        
        // Act
        let videos = VideoScanner::scan_directory(&temp_dir, true)?;
        
        // Assert
        assert_eq!(videos.len(), 3);
    }
}
```

### Integration Tests

Test command integration:
```rust
#[tokio::test]
async fn test_add_and_get_folders() {
    let state = setup_test_db().await;
    let folder_id = add_folder("/test".to_string(), true, State::from(&state))
        .await
        .expect("Failed to add folder");
    assert!(folder_id > 0);
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Watch mode (requires cargo-watch)
cargo watch -x test
```

## CI/CD

### Continuous Integration

Every push and pull request triggers CI checks:

1. **Rust Tests** - All tests must pass
2. **Rust Linting** - `cargo clippy` with no warnings
3. **Rust Formatting** - `cargo fmt --check`
4. **TypeScript Type Check** - `tsc --noEmit`
5. **Frontend Build** - Must build successfully
6. **Integration Build** - Full Tauri app build

**CI runs on**: macOS (for Tauri compatibility)

**Build time**: ~10 minutes (first run), ~5 minutes (cached)

### Continuous Deployment

Tags trigger automated releases:

```bash
# Create and push a tag
git tag v0.2.0
git push origin v0.2.0
```

This will:
1. Build for macOS (Universal, Intel, Apple Silicon)
2. Create GitHub Release
3. Upload DMG artifacts
4. Generate checksums

### Checking CI Status

- View CI status on your PR
- Click "Details" to see logs
- Fix any failing checks before requesting review

## Pull Request Process

### 1. Before Submitting

- [ ] All tests pass locally
- [ ] Code is formatted (`cargo fmt`)
- [ ] No linting warnings (`cargo clippy`)
- [ ] TypeScript type checks pass
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (if applicable)

### 2. Create Pull Request

- Use a descriptive title
- Reference related issues (`Fixes #123`)
- Describe what changed and why
- Include screenshots for UI changes
- List any breaking changes

**PR Template:**

```markdown
## Description
Brief description of changes

## Related Issues
Fixes #123

## Changes
- Added feature X
- Fixed bug Y
- Updated documentation

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests pass
- [ ] Manually tested on macOS

## Screenshots (if applicable)
[Add screenshots here]

## Checklist
- [ ] Tests pass
- [ ] Code formatted
- [ ] Documentation updated
- [ ] CHANGELOG updated
```

### 3. Review Process

- Address review feedback promptly
- Keep discussions focused and professional
- Update PR based on feedback
- Request re-review after changes

### 4. Merging

- PRs require passing CI checks
- Squash commits when merging (if many small commits)
- Use merge commits for feature branches
- Delete branch after merge

## Communication

- **GitHub Issues** - Bug reports, feature requests
- **Pull Requests** - Code review, discussion
- **Discussions** - General questions, ideas

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on the code, not the person
- Help others learn and grow

## Questions?

- Check [SETUP.md](doc/SETUP.md) for setup help
- Check [API Documentation](doc/design/api-specification.md)
- Check [Architecture Documentation](doc/design/architecture.md)
- Open a GitHub Discussion for questions

---

**Thank you for contributing to SceneBrowser!** 🎬
