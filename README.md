# html_view

A lightweight, cross-platform HTML viewer for Rust.

`html_view` provides a minimal, ergonomic API for rendering HTML content in a native window, similar in spirit to `matplotlib.pyplot.show()` for visualisation rather than UI development.

It is intended for debugging, inspection, and lightweight display, not for building full desktop applications.

## What This Is (and Is Not)

**html_view is:**

* A quick way to render HTML from Rust
* Useful for visualisation, debugging, reports, and local tools
* Designed to require minimal setup and minimal code

**html_view is not:**

* A GUI framework
* A browser replacement
* A long-lived embedded webview inside your process
* A solution for complex application state or interaction

If you need a full application framework, use Tauri, egui, iced, or a native GUI toolkit directly.

## Architecture

`html_view` works by launching a small, native Tauri application as a **separate process** and sending it instructions over a simple JSON-based protocol.

This design:

* Keeps your Rust process lightweight and isolated
* Avoids embedding webview state or event loops
* Allows blocking or non-blocking execution
* Makes failures and crashes contained and debuggable

You install the viewer once, then reuse it across projects.

## Quick Start

Render inline HTML with a single call:

```rust
use html_view;

fn main() -> Result<(), html_view::ViewerError> {
    html_view::show("<h1>Hello, World!</h1>")?;
    Ok(())
}
```

## Core Features

* Minimal, single-function API
* Supports inline HTML, local files, application directories, and URLs
* Blocking or non-blocking execution modes
* Secure defaults with explicit opt-in for remote content
* Cross-platform native rendering
* Runtime configuration for window behaviour and lifecycle

## Installation

At runtime, `html_view` requires the viewer application binary.

Install it once:

```bash
cargo install html_view_app
```

This installs the viewer into `~/.cargo/bin`.

### Optional: Command-Line Tool

If you want to use the viewer without writing Rust code:

```bash
cargo install html_view_cli
```

## Usage Patterns

### Inline HTML

```rust
html_view::show("<h1>Hello!</h1><p>Simple HTML display</p>")?;
```

### Configurable Window

```rust
use html_view::ViewerOptions;

let mut options = ViewerOptions::inline_html("<h1>Custom Window</h1>");
options.window.width = Some(800);
options.window.height = Some(600);
options.window.title = Some("My App".to_string());

html_view::open(options)?;
```

### Non-blocking Execution

```rust
use html_view::{ViewerOptions, ViewerWaitMode, ViewerResult};

let mut options = ViewerOptions::inline_html("<h1>Non-blocking</h1>");
options.wait = ViewerWaitMode::NonBlocking;

match html_view::open(options)? {
    ViewerResult::NonBlocking(mut handle) => {
        // Do other work here
        let status = handle.wait()?;
        println!("Viewer closed: {:?}", status.reason);
    }
    _ => unreachable!(),
}
```

### Files, Directories, and URLs

```rust
ViewerOptions::local_file("index.html".into());
ViewerOptions::app_dir("./dist".into());
ViewerOptions::remote_url("https://example.com".parse()?);
```

Remote URLs require explicit permission, see Security below.

### Time-Limited Display

```rust
let mut options = ViewerOptions::inline_html("<h1>Auto-close</h1>");
options.environment.timeout_seconds = Some(5);

html_view::open(options)?;
```

## Security

By default:

* Remote content is disabled
* External navigation is blocked
* Developer tools are disabled
* Only provided content is rendered

To enable remote access:

```rust
let mut options = ViewerOptions::inline_html("<h1>Hello</h1>");
options.behaviour.allow_remote_content = true;
options.behaviour.allow_external_navigation = true;
options.behaviour.allowed_domains = Some(
    vec!["example.com".to_string()]
);
```

This design prevents accidental network access or data leakage.

## Command-Line Interface

The CLI is a thin wrapper around the same viewer.

### Examples

```bash
html_view_cli html "<h1>Hello</h1>"
html_view_cli file index.html
html_view_cli dir ./dist
html_view_cli url https://example.com
```

### Options

```bash
html_view_cli html "<h1>Custom</h1>" \
  --width 800 \
  --height 600 \
  --title "My App" \
  --timeout 10
```

## Project Structure

This repository is a Cargo workspace:

* `html_view`: Public Rust API
* `html_view_shared`: Protocol and shared types
* `html_view_app`: Tauri 2.0 viewer application
* `html_view_cli`: Optional CLI frontend

The API crate spawns the viewer and communicates via JSON files.

## Building from Source

```bash
cargo build --workspace
cargo install --path html_view_app
```

Note: the `html_view_app` is a Tauri application that bundles frontend assets; using `cargo install` in development may not include built frontend assets if they haven't been prepared. To reliably install the viewer from source, build the workspace (which runs the Tauri build step) and then install from the app path:

```bash
cargo build --workspace --release
cargo install --path html_view_app
# or from the workspace root:
cargo install --path ./html_view_app
```

If you run into missing UI assets after `cargo install`, prefer building with `cargo build` (or the platform-appropriate bundling commands) and then use the produced binary from `target/release` or use your platform's packaging pipeline. On Linux, ensure `libwebkit2gtk` is installed before running the built app.

## Platform Requirements

### Linux

Requires WebKitGTK:

```bash
sudo apt install libwebkit2gtk-4.1-dev
sudo dnf install webkit2gtk4.1-devel
sudo pacman -S webkit2gtk-4.1
```

### Run locally (Linux)

After building, you can run the viewer directly from the `target` directory. Ensure the system WebKitGTK libraries are installed first (see above). Example:

```bash
# Build release binary
cargo build -p html_view_app --release

# Run the built binary with a sample config (create paths as needed)
./target/release/html_view_app --config-path /tmp/sample_config.json --result-path /tmp/sample_result.json

# The viewer will write the JSON result file when it exits.
```

If the binary fails to start due to missing frontend assets, prefer running the built binary from a development build (i.e. `cargo run -p html_view_app`) or use the packaged output created by your platform bundler.

### macOS

Uses the system WKWebView.

### Windows

Uses WebView2, available by default on Windows 11.

## Troubleshooting

### Viewer Not Found

If the viewer cannot be located:

```bash
cargo install --path html_view_app
```

Or set the path manually:

```bash
export HTML_VIEW_APP_PATH=/path/to/html_view_app
```

The resolver checks, in order:

1. Embedded build-time path
2. `HTML_VIEW_APP_PATH`
3. `~/.cargo/bin/html_view_app`
4. Caller executable directory
5. `target/debug` and `target/release`

## License

MIT

## Contributing

Contributions are welcome. See `CONTRIBUTING.md`.

## Acknowledgements

Built using Tauri 2.0.
