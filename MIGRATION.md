# Migration Guide: Bundled Feature

This guide helps you migrate between manual and bundled installation modes for `html_view`.

## Table of Contents

- [Upgrading from Manual to Bundled](#upgrading-from-manual-to-bundled)
- [Downgrading from Bundled to Manual](#downgrading-from-bundled-to-manual)
- [CI/CD Considerations](#cicd-considerations)
- [Docker Deployments](#docker-deployments)
- [Troubleshooting Migration Issues](#troubleshooting-migration-issues)

---

## Upgrading from Manual to Bundled

### When to Choose Bundled

Consider using the bundled feature if:
- ✅ You want a single-command installation
- ✅ You're prototyping or learning
- ✅ You have stable internet access during builds
- ✅ Binary size (6-12 MB download) is acceptable

### Migration Steps

#### Before (Manual Installation)

```toml
# Cargo.toml
[dependencies]
html_view = "0.1"
```

Installation workflow:
```bash
# Step 1: Install library
cargo add html_view

# Step 2: Install binary (separate step)
cargo install html_view_app

# Step 3: Build project
cargo build
```

#### After (Bundled Installation)

```toml
# Cargo.toml
[dependencies]
html_view = { version = "0.1", features = ["bundled"] }
```

Installation workflow:
```bash
# Single step: Library + binary auto-download
cargo add html_view --features bundled
cargo build  # Binary downloads automatically on first build
```

### What Changes

1. **First Build Time**: Increased by ~30-60 seconds for binary download
2. **OUT_DIR**: Binary stored in build output directory
3. **No Manual Install**: `cargo install html_view_app` no longer needed
4. **Build Dependency**: `ureq` crate added (only when bundled feature active)

### Verification Steps

After upgrading:

```bash
# Clean previous builds
cargo clean

# Build with bundled feature
cargo build --features bundled
```

You should see:
```
cargo:warning=Downloading html_view_app binary from https://...
cargo:warning=This is a one-time download (~6-12 MB depending on platform)
cargo:warning=Successfully downloaded binary to ...
```

Test that it works:
```bash
# Run your program
cargo run --features bundled
```

### Cleanup (Optional)

After upgrading to bundled, you can optionally remove the manually installed binary:

```bash
# Remove manually installed binary
cargo uninstall html_view_app

# Verify removal
which html_view_app
# Should return: not found
```

Your program will still work because the bundled feature provides the binary.

---

## Downgrading from Bundled to Manual

### When to Choose Manual

Consider switching to manual installation if:
- ✅ You need faster incremental builds
- ✅ You're in a restricted network environment
- ✅ You prefer explicit dependency management
- ✅ You're deploying in CI/CD environments

### Migration Steps

#### Before (Bundled Installation)

```toml
# Cargo.toml
[dependencies]
html_view = { version = "0.1", features = ["bundled"] }
```

#### After (Manual Installation)

```toml
# Cargo.toml
[dependencies]
html_view = "0.1"  # Remove features = ["bundled"]
```

Installation workflow:
```bash
# Step 1: Update Cargo.toml (remove bundled feature)

# Step 2: Install binary manually
cargo install html_view_app

# Step 3: Clean and rebuild
cargo clean
cargo build
```

### What Changes

1. **Manual Binary Management**: Must run `cargo install html_view_app` yourself
2. **Faster Builds**: No download step during builds
3. **No Build Dependencies**: `ureq` no longer compiled
4. **Binary Location**: Uses system-installed binary from `~/.cargo/bin/`

### Verification Steps

After downgrading:

```bash
# Verify binary is installed
html_view_app --version

# Check binary location
which html_view_app
# Should return: /home/user/.cargo/bin/html_view_app (or equivalent)

# Build without bundled
cargo build

# Run your program
cargo run
```

---

## CI/CD Considerations

### Recommendation: Use Manual Installation for CI

For CI/CD pipelines, **manual installation is recommended** over bundled:

#### Why Manual Is Better for CI

1. **Better Caching**: CI systems cache `~/.cargo/bin/` effectively
2. **Explicit Dependencies**: Version pinning is clearer
3. **Faster Builds**: No repeated downloads on cache hits
4. **Network Reliability**: CI doesn't depend on GitHub release downloads

### GitHub Actions Example

#### With Manual Installation (Recommended)

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo bin
        uses: actions/cache@v4
        with:
          path: ~/.cargo/bin
          key: ${{ runner.os }}-cargo-bin-html_view_app

      - name: Install html_view_app
        run: cargo install html_view_app || echo "Already cached"

      - name: Build
        run: cargo build --verbose

      - name: Run tests
        run: cargo test --verbose
```

#### With Bundled Feature (Not Recommended)

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build with bundled
        run: cargo build --features bundled --verbose
        # Downloads binary on every fresh build

      - name: Run tests
        run: cargo test --features bundled --verbose
```

**Issues with bundled in CI:**
- ❌ Downloads binary on every fresh build
- ❌ Depends on GitHub being accessible
- ❌ Slower builds
- ❌ Less cache-friendly

### GitLab CI Example

```yaml
test:
  image: rust:latest
  before_script:
    - apt-get update && apt-get install -y libwebkit2gtk-4.1-dev
    - cargo install html_view_app || echo "Already installed"
  script:
    - cargo build
    - cargo test
  cache:
    paths:
      - ~/.cargo/bin/html_view_app
      - target/
```

---

## Docker Deployments

### With Manual Installation (Recommended)

```dockerfile
FROM rust:1.75 as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.1-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Install html_view_app (cached layer)
RUN cargo install html_view_app

# Copy source
COPY . .

# Build application (without bundled feature)
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.1-0 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary and html_view_app from builder
COPY --from=builder /usr/local/cargo/bin/html_view_app /usr/local/bin/
COPY --from=builder /app/target/release/your_app /usr/local/bin/

CMD ["your_app"]
```

### With Bundled Feature (Not Recommended)

```dockerfile
FROM rust:1.75 as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.1-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY . .

# Build with bundled feature
# Downloads binary during build - slower, network-dependent
RUN cargo build --release --features bundled

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.1-0 \
    && rm -rf /var/lib/apt/lists/*

# Bundled binary is embedded in your_app
COPY --from=builder /app/target/release/your_app /usr/local/bin/

CMD ["your_app"]
```

**Issues with bundled in Docker:**
- ❌ Downloads on every image build
- ❌ Build cache invalidation on code changes
- ❌ Larger build context
- ❌ Network dependency during build

---

## Troubleshooting Migration Issues

### Build Fails After Migration

#### Symptom

```
error: failed to download `html_view`
```

#### Solution

Ensure you've updated `Cargo.toml` correctly:

```bash
# Clean cargo cache
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build
```

### Binary Not Found After Removing Bundled

#### Symptom

```
Error: html_view_app binary not found
```

#### Solution

Install the binary manually:

```bash
cargo install html_view_app
```

Verify installation:

```bash
which html_view_app
html_view_app --version
```

### Features Not Working After Migration

#### Symptom

Code works with bundled but fails with manual installation.

#### Cause

The code itself doesn't change - both use the same API. This usually means the binary isn't installed.

#### Solution

```bash
# Ensure binary is installed
cargo install html_view_app --force

# Verify version matches
cargo tree | grep html_view
html_view_app --version
# Versions should match or be compatible
```

### Slow Builds After Adding Bundled

#### Symptom

First build with bundled feature is very slow.

#### Cause

Binary download takes time (6-12 MB, 5-minute timeout).

#### Solution

**Option 1**: Be patient - subsequent builds are fast (binary is cached)

**Option 2**: Switch back to manual:
```bash
# Remove bundled feature
# Edit Cargo.toml: html_view = "0.1"

cargo install html_view_app
cargo build
```

### Version Mismatch After Migration

#### Symptom

```
Error: version mismatch: library v0.2.0, viewer v0.1.0
```

#### Solution

**With Bundled**:
```bash
cargo clean
cargo build --features bundled
# Downloads correct version automatically
```

**With Manual**:
```bash
cargo install html_view_app --force
# Installs latest version matching your library
```

---

## Quick Reference

### Bundled Feature

| Aspect | Details |
|--------|---------|
| **Cargo.toml** | `html_view = { version = "0.1", features = ["bundled"] }` |
| **Install Command** | `cargo add html_view --features bundled` |
| **Binary Location** | `target/*/build/html_view-*/out/html_view_app` |
| **First Build Time** | +30-60 seconds (download) |
| **Subsequent Builds** | Normal (cached) |
| **Network Required** | Yes (first build only) |
| **Best For** | Prototyping, quick start, single-command setup |

### Manual Installation

| Aspect | Details |
|--------|---------|
| **Cargo.toml** | `html_view = "0.1"` |
| **Install Commands** | `cargo add html_view` + `cargo install html_view_app` |
| **Binary Location** | `~/.cargo/bin/html_view_app` |
| **Build Time** | Faster (no download) |
| **Network Required** | Only for `cargo install` |
| **Best For** | Production, CI/CD, explicit control |

---

## Need Help?

If you encounter issues during migration:

1. Check [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues
2. Search [existing issues](https://github.com/jmg049/HTMLView/issues)
3. Open a [new issue](https://github.com/jmg049/HTMLView/issues/new) with:
   - Migration direction (manual→bundled or bundled→manual)
   - Full error message
   - OS and Rust version
   - Steps to reproduce
