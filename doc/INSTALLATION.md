# SceneBrowser Installation Guide

## Installing the Stable Release

1. Download the latest version from [GitHub Releases](https://github.com/nackon/SceneBrowser/releases/latest)
2. Open the DMG file
3. Drag SceneBrowser.app to the Applications folder
4. Launch the app

The stable release is officially signed, so no Gatekeeper warning will appear.

## Installing a Development Build

Development builds are ad-hoc signed, so some extra steps are required.

### Method 1: Open via right-click (Recommended)

1. Mount the DMG and copy SceneBrowser.app to Applications
2. Locate `/Applications/SceneBrowser.app` in Finder
3. **Right-click** it and select **Open**
4. When the "developer cannot be verified" warning appears, click **Open**
5. After this, you can launch it normally by double-clicking

### Method 2: Remove the quarantine attribute via Terminal

```bash
# After copying the app to Applications
xattr -cr /Applications/SceneBrowser.app

# Or remove the attribute from the DMG file itself before installing
xattr -cr ~/Downloads/SceneBrowser-dev.dmg
```

### Method 3: Allow via System Settings (macOS Ventura and later)

1. Try to open the app; a warning appears
2. Open System Settings → Privacy & Security
3. Click **Open Anyway** next to "'SceneBrowser' was blocked because it is from an unidentified developer"

## Troubleshooting

### "Apple could not verify... is free of malware" error

This is a macOS Gatekeeper warning shown because the development build is not notarized by Apple. **The app is safe.**

**Solution 1: Terminal command (most reliable):**

```bash
# After copying the app to Applications, run this in Terminal
xattr -cr /Applications/SceneBrowser.app

# Then double-click to launch it in Finder
```

**Solution 2: Open via right-click:**

1. Locate `/Applications/SceneBrowser.app` in Finder
2. **Right-click** it and select **Open**
3. Click **Open** in the warning dialog

**Solution 3: Allow via System Settings (macOS Ventura and later):**

1. Try to open the app; a warning appears
2. Open System Settings → Privacy & Security
3. Click **Open Anyway**

### "SceneBrowser is damaged and can't be opened" error

This is also caused by Gatekeeper — the app is not actually damaged.

**Solution:**

```bash
# Remove the quarantine attribute
xattr -cr /Applications/SceneBrowser.app

# Or list all extended attributes
xattr -l /Applications/SceneBrowser.app

# Or remove just the quarantine attribute
xattr -d com.apple.quarantine /Applications/SceneBrowser.app
```

### "Cannot be opened because the developer cannot be verified" error

This warning appears because development builds are not signed with a personal developer certificate.

**Solution:**
1. Right-click → Open (first time only)
2. Or use the `xattr` command above

### DMG won't mount

```bash
# Remove the quarantine attribute from the DMG file
xattr -cr ~/Downloads/SceneBrowser-dev.dmg

# Then double-click the DMG
```

## About Security

Development builds are ad-hoc signed (signed with `-`). This means:
- Not signed with an Apple Developer ID
- Not notarized
- Intended for development/testing purposes

**Using the stable release is recommended for production use.**

Official signing is planned to be added to stable releases (built from git tags) in the future.

## Installing FFmpeg

SceneBrowser requires FFmpeg:

```bash
brew install ffmpeg
```

If FFmpeg is not installed, an error will be shown when the app starts.
