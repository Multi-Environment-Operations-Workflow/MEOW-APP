# GitHub Actions Workflows

This directory contains GitHub Actions workflows to test the MEOW-APP on different platforms.

## 📁 Workflow Files

### 1. `desktop-test.yml` - Desktop Platform Testing

**Triggers**: Push to main branches, Pull Requests  
**Platforms**: Linux (Ubuntu), Windows  
**What it does**:

- ✅ Installs system dependencies (Linux WebKit, ALSA, etc.)
- ✅ Runs Rust tests with `cargo test --no-default-features`
- ✅ Builds Tauri desktop applications
- ✅ Uploads build artifacts (AppImage, DEB, MSI, EXE)

### 2. `android-test.yml` - Android Platform Testing

**Triggers**: Push to main branches, Pull Requests  
**Platforms**: Android (ARM64, ARMv7)  
**What it does**:

- ✅ Sets up Android SDK and NDK
- ✅ Adds Rust Android targets
- ✅ Tests Android compilation compatibility
- ✅ Builds Android APK (if configured)
- ⚠️ Includes optional emulator testing (disabled by default)

### 3. `full-platform-test.yml` - Complete Cross-Platform Suite

**Triggers**: Push to main/develop, Pull Requests, Manual dispatch  
**Platforms**: Linux, Windows, Android  
**What it does**:

- ✅ Runs all desktop and mobile tests in parallel
- ✅ Generates comprehensive test summary
- ✅ Shows overall platform compatibility status

## 🚀 How to Use

### Automatic Triggers

The workflows automatically run when you:

- Push code to `main`, `develop`, `mobile-desktop`, or `Making-test-balls` branches
- Create a Pull Request to `main` or `develop`

### Manual Triggers

You can manually run the full test suite:

1. Go to your GitHub repository
2. Click "Actions" tab
3. Select "Full Platform Test Suite"
4. Click "Run workflow"

## 📊 What Gets Tested

### Rust Backend

- ✅ WebSocket server functionality
- ✅ Bridge communication
- ✅ Cross-platform compilation
- ✅ Audio/media dependencies (Linux)

### Frontend

- ✅ Node.js dependencies install
- ✅ TypeScript compilation
- ✅ React component functionality

### Platform-Specific

- **Linux**: WebKit, GTK, ALSA dependencies
- **Windows**: WebView2, native Windows APIs
- **Android**: NDK compilation, ARM targets

## 🔧 Configuration

### Required Secrets

No secrets required for basic testing.

### Optional Configuration

- **Android Emulator**: Set `android-emulator-test.if` to `true` for device testing
- **Build Caching**: Automatically enabled for faster subsequent runs
- **Artifact Retention**: Build outputs kept for 90 days

## 📝 Workflow Status Badges

Add these to your README.md:

```markdown
![Desktop Tests](<https://github.com/Multi-Environment-Operations-Workflow/MEOW-APP/workflows/Desktop%20Tests%20(Windows%20&%20Linux)/badge.svg>)
![Android Tests](https://github.com/Multi-Environment-Operations-Workflow/MEOW-APP/workflows/Android%20Tests/badge.svg)
![Full Platform Tests](https://github.com/Multi-Environment-Operations-Workflow/MEOW-APP/workflows/Full%20Platform%20Test%20Suite/badge.svg)
```

## 🐛 Troubleshooting

### Common Issues

**Linux Build Fails**:

- Check if all system dependencies are installed
- Verify WebKit version compatibility

**Windows Build Fails**:

- Ensure WebView2 runtime compatibility
- Check Windows SDK version

**Android Build Fails**:

- Verify NDK version (25.1.8937393)
- Check Rust target installation
- Ensure Java 17 compatibility

### Debugging Steps

1. Check workflow logs in GitHub Actions tab
2. Look for specific error messages in failed jobs
3. Test locally with same commands used in workflows
4. Verify all dependencies in `Cargo.toml` and `package.json`

## 📈 Performance

- **Desktop Tests**: ~5-10 minutes per platform
- **Android Tests**: ~8-15 minutes
- **Full Suite**: ~15-20 minutes (parallel execution)

Caching reduces subsequent run times by ~50%.
