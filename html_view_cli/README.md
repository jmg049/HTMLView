# html_view_cli

Command-line interface for the html_view HTML viewer.

## Installation

Install both the viewer app and the CLI tool:

```bash
# From the workspace root
cargo install --path html_view_app
cargo install --path html_view_cli
```

Both will be installed to `~/.cargo/bin/` (ensure this is in your PATH).

## Usage

```bash
html_view_cli [SUBCOMMAND] [OPTIONS]
```

### Subcommands

#### `html` - Display inline HTML

```bash
html_view_cli html "<h1>Hello, World!</h1>"
html_view_cli html "<h1>Title</h1><p>Content</p>" --width 800 --height 600
```

#### `file` - Display a local HTML file

```bash
html_view_cli file index.html
html_view_cli file ./build/index.html --title "My App"
```

#### `dir` - Display an HTML application directory

```bash
html_view_cli dir ./dist
html_view_cli dir ./build --entry main.html
```

The `--entry` option specifies which HTML file to load (defaults to `index.html`).

#### `url` - Display a remote URL

```bash
html_view_cli url https://example.com
html_view_cli url https://docs.rs/html_view --width 1200
```

### Global Options

All subcommands support these options:

- `--width <pixels>` - Set window width (default: 1024)
- `--height <pixels>` - Set window height (default: 768)
- `--title <string>` - Set window title
- `--devtools` - Enable developer tools for debugging
- `--timeout <seconds>` - Auto-close window after N seconds

### Examples

**Quick HTML preview:**
```bash
html_view_cli html "<h1>Quick Test</h1>"
```

**View documentation:**
```bash
cargo doc --no-deps
html_view_cli file target/doc/my_crate/index.html
```

**Preview a static site:**
```bash
# Build your static site
npm run build

# View it
html_view_cli dir ./dist
```

**Debug a web page:**
```bash
html_view_cli file debug.html --devtools
```

**Temporary notification:**
```bash
html_view_cli html "<h1>Build Complete!</h1>" \
  --timeout 5 \
  --width 400 \
  --height 200
```

**Custom window:**
```bash
html_view_cli html "<h1>Custom</h1>" \
  --width 800 \
  --height 600 \
  --title "My Custom Window"
```

**Pipe HTML from another command:**
```bash
cat template.html | html_view_cli html "$(cat -)"
```

**View with styling:**
```bash
html_view_cli html '
<!DOCTYPE html>
<html>
<head>
  <style>
    body { font-family: Arial; padding: 40px; }
    h1 { color: #4A90E2; }
  </style>
</head>
<body>
  <h1>Styled Content</h1>
  <p>This is a styled HTML page.</p>
</body>
</html>
'
```

## Tips

1. **Environment Files**: Save commonly used HTML snippets and load them:
   ```bash
   html_view_cli html "$(cat my-template.html)"
   ```

2. **Integration with Build Tools**: Add to your build scripts:
   ```bash
   # In package.json or Makefile
   cargo build && html_view_cli html "<h1>Build Success!</h1>" --timeout 3
   ```

3. **Data Visualization**: Generate HTML reports and view them:
   ```bash
   ./my-analyzer --format html > report.html
   html_view_cli file report.html
   ```

4. **Testing**: Quickly test HTML without opening a browser:
   ```bash
   html_view_cli html "<button onclick='alert(\"test\")'>Click</button>" --devtools
   ```

## Requirements

The `html_view_app` binary must be installed and available. The CLI will look for it in:

1. `~/.cargo/bin/html_view_app` (preferred location)
2. `HTML_VIEW_APP_PATH` environment variable
3. Same directory as `html_view_cli`
4. Development target directories

If you get a "binary not found" error, ensure `html_view_app` is installed:

```bash
cargo install --path html_view_app
```

## Platform Support

- **Linux**: Requires WebKitGTK (`libwebkit2gtk-4.1-dev`)
- **macOS**: Uses system WKWebView (no additional dependencies)
- **Windows**: Uses WebView2 (included in Windows 11)

## License

MIT OR Apache-2.0
