# html_view

The library component of the ``html_view`` suite of crates.
Allows the user to open up the viewer from within their application.

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

## License

MIT

## Contributing

Contributions are welcome. See `CONTRIBUTING.md`.

## Acknowledgements

Built using Tauri 2.0.
