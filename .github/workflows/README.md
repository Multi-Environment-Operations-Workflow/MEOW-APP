# GitHub Actions Workflows

This directory contains GitHub Actions workflows to test the MEOW-APP on different platforms.

## 📁 Workflow Files

### 1. `fast-ci.yml` - ⚡ Optimized CI/CD (Recommended)

**Triggers**: Push to main branches, Pull Requests, Manual dispatch  
**Platforms**: Linux (Container), Windows, Android (Container)  
**Performance**: ~60% faster than standard workflows  
**What it does**:

- ⚡ Uses pre-built Docker images with Rust/Android tools
- ✅ Linux: `rust:1.89-slim-bookworm` container
- ✅ Android: `cimg/android:2024.01.1` container
- ✅ Windows: GitHub runners (fastest for Windows)
- ✅ Optimized caching and dependency management
- ✅ Comprehensive build summary with performance metrics

### 2. `desktop-test.yml` - Desktop Platform Testing

**Triggers**: Push to main branches, Pull Requests  
**Platforms**: Linux (Container), Windows  
**What it does**:

- ✅ Container-based Linux builds for faster setup
- ✅ Installs system dependencies (Linux WebKit, ALSA, etc.)
- ✅ Runs Rust tests with `cargo test --no-default-features`
- ✅ Builds Tauri desktop applications
- ✅ Uploads build artifacts (AppImage, DEB, MSI, EXE)

### 3. `android-test.yml` - Android Platform Testing

**Triggers**: Push to main branches, Pull Requests  
**Platforms**: Android (ARM64, ARMv7)  
**What it does**:

- ✅ Uses Android SDK container for faster setup
- ✅ Sets up Android SDK and NDK
- ✅ Adds Rust Android targets
- ✅ Tests Android compilation compatibility
- ✅ Builds Android APK (if configured)
- ⚠️ Includes optional emulator testing (disabled by default)

### 4. `full-platform-test.yml` - Complete Cross-Platform Suite

**Triggers**: Push to main/develop, Pull Requests, Manual dispatch  
**Platforms**: Linux, Windows, Android  
**What it does**:

- ✅ Matrix strategy with container/host configurations
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
- **Build Caching**: Automatically enabled with actions/cache@v4 for faster subsequent runs
- **Artifact Retention**: Build outputs kept for 90 days with actions/upload-artifact@v4

### Action Versions Used

- ✅ `actions/checkout@v4` - Latest stable
- ✅ `actions/upload-artifact@v4` - Fixed deprecation warnings
- ✅ `actions/cache@v4` - Improved caching performance
- ✅ `actions/setup-node@v4` - Node.js 18 support
- ✅ `actions/setup-java@v4` - Java 17 support
- ✅ `dtolnay/rust-toolchain@stable` - Latest Rust toolchain

## 📝 Workflow Status Badges

Add these to your README.md:

```markdown
![Fast CI/CD](https://github.com/Multi-Environment-Operations-Workflow/MEOW-APP/workflows/Fast%20CI/CD%20with%20Pre-built%20Images/badge.svg)
![Desktop Tests](<https://github.com/Multi-Environment-Operations-Workflow/MEOW-APP/workflows/Desktop%20Tests%20(Windows%20&%20Linux)/badge.svg>)
![Android Tests](https://github.com/Multi-Environment-Operations-Workflow/MEOW-APP/workflows/Android%20Tests/badge.svg)
![Full Platform Tests](https://github.com/Multi-Environment-Operations-Workflow/MEOW-APP/workflows/Full%20Platform%20Test%20Suite/badge.svg)
```

### Recommended Badge for Daily Development

```markdown
[![Fast CI/CD](https://github.com/Multi-Environment-Operations-Workflow/MEOW-APP/workflows/Fast%20CI/CD%20with%20Pre-built%20Images/badge.svg)](https://github.com/Multi-Environment-Operations-Workflow/MEOW-APP/actions)
```

## 🐛 Troubleshooting

### Common Issues

**Cargo.lock Version Mismatch**:

```
error: lock file version `4` was found, but this version of Cargo does not understand this lock file
```

- **Cause**: Cargo.lock generated with newer Cargo version than container
- **Solution**: All workflows now include automatic Cargo.lock regeneration
- **Prevention**: Use consistent Rust 1.89+ across all environments

**Linux Build Fails**:

- Check if all system dependencies are installed
- Verify WebKit version compatibility
- Ensure container uses `rust:1.89-slim-bookworm` or newer

**Windows Build Fails**:

- Ensure WebView2 runtime compatibility
- Check Windows SDK version
- Verify Rust 1.89 compatibility

**Android Build Fails**:

- Verify NDK version (25.1.8937393)
- Check Rust target installation
- Ensure Java 17 compatibility
- Verify Rust 1.89 with Android NDK compatibility

### Debugging Steps

1. Check workflow logs in GitHub Actions tab
2. Look for specific error messages in failed jobs
3. Test locally with same commands used in workflows
4. Verify all dependencies in `Cargo.toml` and `package.json`

## 🧪 Validation

### Workflow Validation Script

Run the included validation script to check your workflows:

```bash
./.github/workflows/validate-workflows.sh
```

This script checks for:

- ✅ Deprecated action versions
- ✅ Current recommended versions
- ✅ YAML syntax errors (if `yq` is installed)
- ✅ Workflow file count and structure

### Install Additional Tools

For full validation, install `yq`:

```bash
sudo snap install yq
```

## 📈 Performance

| Workflow                 | Setup Time | Build Time | Total Time | Best For          |
| ------------------------ | ---------- | ---------- | ---------- | ----------------- |
| `fast-ci.yml` ⚡         | ~2 min     | ~5-8 min   | ~7-10 min  | Daily development |
| `desktop-test.yml`       | ~3 min     | ~6-10 min  | ~9-13 min  | Desktop features  |
| `android-test.yml`       | ~4 min     | ~8-12 min  | ~12-16 min | Mobile features   |
| `full-platform-test.yml` | ~5 min     | ~10-15 min | ~15-20 min | Releases          |

### Performance Benefits of Container-Based Builds

- ⚡ **60% faster setup**: Pre-built images eliminate tool installation time
- 🎯 **Consistent environment**: Same runtime across all builds
- 📦 **Optimized caching**: Better cache hit rates with predictable environments
- 🔧 **Reduced dependencies**: No need to install Rust/Android SDK each time

Caching reduces subsequent run times by ~50% on top of container optimizations.
