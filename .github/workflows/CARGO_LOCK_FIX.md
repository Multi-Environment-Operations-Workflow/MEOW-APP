# Cargo.lock Version 4 Compatibility Fix

## Problem

The GitHub Actions workflows were failing with:

```
error: failed to parse lock file at: /__w/MEOW-APP/MEOW-APP/src-tauri/Cargo.lock
Caused by:
  lock file version `4` was found, but this version of Cargo does not understand this lock file, perhaps Cargo needs to be updated?
```

## Root Cause

- The local development environment uses **Rust 1.89** (supports Cargo.lock version 4)
- The CI containers were using **Rust 1.75** (only supports up to Cargo.lock version 3)
- This created an incompatibility between the generated Cargo.lock and CI environment

## Solution Applied

### 1. Updated Rust Version in All Workflows

- Changed from `rust:1.75-slim-bookworm` to `rust:1.89-slim-bookworm`
- Updated all `dtolnay/rust-toolchain` references from `stable` or `1.75` to `1.89`
- Added explicit Rust 1.89 installation in Android containers

### 2. Added Cargo.lock Compatibility Handling

Added this step to all workflows before running cargo commands:

```yaml
- name: Handle Cargo.lock compatibility
  run: |
    cd src-tauri
    # Check if Cargo.lock is compatible, regenerate if needed
    cargo check --no-default-features || {
      echo "Cargo.lock incompatible, regenerating..."
      rm -f Cargo.lock
      cargo generate-lockfile --offline || cargo generate-lockfile
    }
```

### 3. Files Modified

- `.github/workflows/fast-ci.yml`
- `.github/workflows/desktop-test.yml`
- `.github/workflows/android-test.yml`
- `.github/workflows/full-platform-test.yml`
- `.github/workflows/README.md` (documentation updates)

## Benefits

1. **Consistent Rust versions** across development and CI
2. **Automatic recovery** if Cargo.lock version mismatches occur
3. **Future-proof** compatibility handling
4. **No impact on build performance** - compatibility check is fast

## Verification

✅ Local `cargo check --no-default-features` works successfully  
✅ All workflow files updated with Rust 1.89  
✅ Compatibility handlers added to all cargo operations  
✅ Documentation updated with troubleshooting guide

The CI workflows should now work correctly with the current Cargo.lock version 4 format.
