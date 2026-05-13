# Task: CI/CD Setup with GitHub Actions

## Description
Implement Continuous Integration and Continuous Deployment pipelines using GitHub Actions for automated testing, building, and releasing.

## Objectives
- [ ] CI: Automated testing on pull requests and pushes
- [ ] CI: Code linting and formatting checks
- [ ] CD: Automated builds for macOS (DMG)
- [ ] CD: Automated GitHub Releases with artifacts
- [ ] Cache optimization for faster builds

## CI Workflow Requirements

### On Pull Request / Push to main
1. **Rust Tests**
   - Run `cargo test`
   - Run `cargo clippy` for linting
   - Run `cargo fmt --check` for formatting

2. **Frontend Tests**
   - Run `npm install`
   - Run `npm run build` to verify build
   - TypeScript type checking

3. **Platform**: macOS (for Tauri compatibility)

## CD Workflow Requirements

### On Git Tag (e.g., v0.1.0)
1. **Build for macOS**
   - Intel (x86_64)
   - Apple Silicon (aarch64)
   - Universal binary

2. **Artifacts**
   - DMG installer
   - App bundle (.app)
   - Checksums (SHA256)

3. **GitHub Release**
   - Auto-create release from tag
   - Upload build artifacts
   - Generate release notes

## Implementation Steps

### Step 1: CI Workflow (.github/workflows/ci.yml)
- Trigger on: push, pull_request
- Jobs:
  - rust-test: Run Rust tests and linting
  - frontend-build: Build frontend and type check
- Cache: Cargo dependencies, npm modules

### Step 2: CD Workflow (.github/workflows/release.yml)
- Trigger on: tag push (v*)
- Jobs:
  - build-macos: Build DMG for macOS
  - create-release: Create GitHub Release
  - upload-artifacts: Upload DMG and checksums
- Use: tauri-action for building

### Step 3: Configuration Files
- .github/workflows/ci.yml
- .github/workflows/release.yml
- Update CODEOWNERS (optional)
- Update CONTRIBUTING.md with CI/CD info

## Testing Strategy

### CI Testing
1. Create a test branch
2. Push changes to trigger CI
3. Verify all checks pass
4. Create a PR to test PR checks

### CD Testing
1. Create a test tag (e.g., v0.0.1-test)
2. Push tag to trigger CD
3. Verify build completes
4. Verify release is created with artifacts
5. Delete test tag and release

## Cache Strategy

### Rust Cache
```yaml
- uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/bin/
      ~/.cargo/registry/index/
      ~/.cargo/registry/cache/
      ~/.cargo/git/db/
      src-tauri/target/
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

### npm Cache
```yaml
- uses: actions/cache@v3
  with:
    path: ~/.npm
    key: ${{ runner.os }}-node-${{ hashFiles('**/package-lock.json') }}
```

## Required GitHub Secrets

None required for basic setup. All operations use `GITHUB_TOKEN` automatically.

For signing (optional, future):
- `APPLE_CERTIFICATE` - Developer ID certificate
- `APPLE_CERTIFICATE_PASSWORD`
- `APPLE_ID` - Apple ID for notarization
- `APPLE_PASSWORD` - App-specific password

## Success Criteria

### CI
- [ ] Tests run automatically on every push
- [ ] PR checks show pass/fail status
- [ ] Build completes in under 10 minutes
- [ ] Cache reduces subsequent builds to under 5 minutes

### CD
- [ ] Tag push triggers release build
- [ ] DMG is generated and uploaded
- [ ] Release notes are auto-generated
- [ ] Artifacts include checksums

## Performance Targets

| Workflow | First Run | Cached Run |
|----------|-----------|------------|
| CI Tests | < 10 min  | < 5 min    |
| CD Build | < 15 min  | < 10 min   |

## Documentation Updates

- [ ] Update README.md with build status badge
- [ ] Update CONTRIBUTING.md with CI/CD guidelines
- [ ] Add RELEASE.md with release process

## Estimated Time
4 hours

## Dependencies
None

## Labels
- ci-cd
- automation
- infrastructure

## References
- [Tauri GitHub Action](https://github.com/tauri-apps/tauri-action)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [cargo-cache Action](https://github.com/actions/cache)
