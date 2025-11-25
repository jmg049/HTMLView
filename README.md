# html_view

A lightweight, cross-platform HTML viewer library for Rust, providing a simple API similar to `matplotlib.pyplot.show()` in Python.

## Features

- 🪟 **Simple API**: Display HTML with a single function call
- 🎨 **Multiple Content Types**: Inline HTML, local files, directories, and remote URLs
- ⚡ **Blocking & Non-blocking**: Choose between blocking and async modes
- 🔒 **Secure by Default**: Remote content disabled, navigation restricted
- 🖥️ **Cross-platform**: Windows (WebView2), macOS (WKWebView), Linux (WebKitGTK)
- ⚙️ **Configurable**: Window size, position, timeout, and security options

## Installation

### For Library Use (Rust Projects)

First, install the viewer application globally:

```bash
cargo install --path html_view_app
```

This installs `html_view_app` to `~/.cargo/bin/`, making it available system-wide and persistent across `cargo clean` operations.

Then add the library to your `Cargo.toml`:

```toml
[dependencies]
html_view = { path = "path/to/html_view" }
```

Or if published to crates.io:

```toml
[dependencies]
html_view = "0.1"
```

### For CLI Use (Command Line)

If you just want to use the command-line tool without writing Rust code:

```bash
# Install both the app and CLI
cargo install --path html_view_app
cargo install --path html_view_cli

# Now you can use it directly
html_view_cli html "<h1>Hello!</h1>"
```

See [CLI Usage](#cli-usage) section below for details.

## Quick Start

Display HTML:

```rust
use html_view;

fn main() -> Result<(), html_view::ViewerError> {
    html_view::show("<h1>Hello, World!</h1>")?;
    Ok(())
}
```

## Usage Examples

### Simple HTML

```rust
html_view::show("<h1>Hello!</h1><p>Simple HTML display</p>")?;
```

### Custom Window Configuration

```rust
use html_view::{ViewerOptions, WindowOptions};

let mut options = ViewerOptions::inline_html("<h1>Custom Window</h1>");
options.window.width = Some(800);
options.window.height = Some(600);
options.window.title = Some("My App".to_string());

html_view::open(options)?;
```

### Non-blocking Mode

```rust
use html_view::{ViewerOptions, ViewerWaitMode, ViewerResult};

let mut options = ViewerOptions::inline_html("<h1>Non-blocking</h1>");
options.wait = ViewerWaitMode::NonBlocking;

match html_view::open(options)? {
    ViewerResult::NonBlocking(mut handle) => {
        // Do other work while viewer is open
        println!("Viewer is open, doing other work...");
        
        // Wait for it to close
        let status = handle.wait()?;
        println!("Closed: {:?}", status.reason);
    }
    _ => unreachable!(),
}
```

### Local HTML File

```rust
use std::path::PathBuf;

let options = ViewerOptions::local_file(PathBuf::from("index.html"));
html_view::open(options)?;
```

### HTML Application Directory

```rust
use std::path::PathBuf;

let options = ViewerOptions::app_dir(PathBuf::from("./dist"));
html_view::open(options)?;
```

### Remote URL (with caution)

```rust
use url::Url;

let options = ViewerOptions::remote_url(Url::parse("https://example.com")?);
html_view::open(options)?;
```

### Timeout

```rust
let mut options = ViewerOptions::inline_html("<h1>Auto-close</h1>");
options.environment.timeout_seconds = Some(5); // Close after 5 seconds
html_view::open(options)?;
```

## Architecture

This library is a Cargo workspace with four crates:

- **`html_view`**: Public API library (what you import)
- **`html_view_shared`**: Shared protocol types
- **`html_view_app`**: Tauri 2.0 application binary (installed globally)
- **`html_view_cli`**: Optional CLI tool

The API crate spawns the Tauri app as a subprocess and communicates via JSON files.

## Building from Source

Build all crates:

```bash
cargo build --workspace
```

Install the app globally:

```bash
cargo install --path html_view_app
```

Run examples:

```bash
# Simple blocking example
cargo run --package html_view --example simple

# Advanced non-blocking example
cargo run --package html_view --example advanced

# Timeout example
cargo run --package html_view --example timeout
```

## CLI Usage

The CLI tool provides a command-line interface for displaying HTML without writing Rust code.

### Installation

Install both the viewer app and CLI globally:

```bash
# Install the viewer application (required)
cargo install --path html_view_app

# Install the CLI tool
cargo install --path html_view_cli
```

Both binaries will be installed to `~/.cargo/bin/` (which should be in your PATH).

### Usage

Once installed, use the `html_view_cli` command directly:

```bash
# Display inline HTML
html_view_cli html "<h1>Hello, World!</h1>"

# Display a local HTML file
html_view_cli file index.html

# Display an HTML application directory
html_view_cli dir ./dist

# Display a remote URL
html_view_cli url https://example.com

# Customize the window
html_view_cli html "<h1>Custom Window</h1>" \
  --width 800 \
  --height 600 \
  --title "My App"

# Enable devtools for debugging
html_view_cli file debug.html --devtools

# Auto-close after 10 seconds
html_view_cli html "<h1>Temporary</h1>" --timeout 10
```

### CLI Command Reference

**Subcommands:**
- `html <html-string>` - Display inline HTML
- `file <path>` - Display a local HTML file
- `dir <root> [--entry <file>]` - Display an HTML application directory
- `url <url>` - Display a remote URL (requires network access)

**Global Options:**
- `--width <pixels>` - Set window width
- `--height <pixels>` - Set window height  
- `--title <string>` - Set window title
- `--devtools` - Enable developer tools
- `--timeout <seconds>` - Auto-close after N seconds

### Examples

```bash
# Quick visualization of data
echo "<table><tr><td>Data</td></tr></table>" | \
  html_view_cli html "$(cat -)"

# View build output
html_view_cli file target/doc/my_crate/index.html --devtools

# Preview a web app
html_view_cli dir ./frontend/dist --entry index.html

# Display with custom styling
html_view_cli html "<h1 style='color: blue;'>Styled</h1>" \
  --width 400 --height 300
```

## Troubleshooting

### Binary Not Found Error

If you get an error about `html_view_app` not being found:

```bash
# Install the binary globally
cargo install --path html_view_app

# Or set the path manually
export HTML_VIEW_APP_PATH=/path/to/html_view_app
```

The locator checks these locations in order:
1. Compile-time embedded path (if built with build.rs)
2. `HTML_VIEW_APP_PATH` environment variable
3. `~/.cargo/bin/html_view_app` (cargo install location)
4. Same directory as your executable
5. `target/debug/` and `target/release/` directories

## Security

By default, the viewer is configured securely:

- ❌ Remote content disabled
- ❌ External navigation disabled
- ❌ Devtools disabled
- ✅ Only displays the content you explicitly provide

To enable remote content or navigation, you must explicitly configure it:

```rust
let mut options = ViewerOptions::inline_html("<h1>Hello</h1>");
options.behaviour.allow_remote_content = true;
options.behaviour.allow_external_navigation = true;
options.behaviour.allowed_domains = Some(vec!["example.com".to_string()]);
```

## Platform Requirements

### Linux
Requires WebKitGTK. Install with:
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.1-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel

# Arch
sudo pacman -S webkit2gtk-4.1
```

### macOS
No additional dependencies (uses system WKWebView).

### Windows
No additional dependencies (uses WebView2, automatically installed on Windows 11).

## License

MIT OR Apache-2.0

## Contributing

Contributions welcome! Please open an issue or PR.

## Similar Projects

- [tauri](https://tauri.app/) - Full application framework (html_view uses Tauri internally)
- [web-view](https://github.com/Boscop/web-view) - Lightweight webview bindings

## Acknowledgments

Built with [Tauri 2.0](https://tauri.app/).
