//! Example showing how to create a frameless window with custom toolbar.
//!
//! This example demonstrates:
//! - Creating a frameless window (no native decorations)
//! - Adding a custom HTML/CSS title bar
//! - Making the window draggable via custom toolbar
//! - Custom window controls (minimize, maximize, close)

use html_view::ViewerOptions;
use html_view_shared::ToolbarOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Custom Toolbar Example");
    println!("======================\n");

    // Create the main content
    let html_content = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Custom Toolbar Example</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            overflow: hidden;
            background: #f5f5f5;
        }

        .main-content {
            padding: 80px 40px 40px;
            height: 100vh;
            overflow-y: auto;
        }

        .card {
            background: white;
            border-radius: 12px;
            padding: 30px;
            box-shadow: 0 4px 20px rgba(0,0,0,0.08);
            max-width: 800px;
            margin: 0 auto;
        }

        h1 {
            color: #2c3e50;
            margin-bottom: 20px;
            font-size: 2em;
        }

        h2 {
            color: #34495e;
            margin-top: 30px;
            margin-bottom: 15px;
            font-size: 1.5em;
        }

        p {
            color: #555;
            line-height: 1.6;
            margin-bottom: 15px;
        }

        .feature-list {
            list-style: none;
            padding: 0;
        }

        .feature-list li {
            padding: 12px;
            margin: 8px 0;
            background: #e8f4f8;
            border-left: 4px solid #3498db;
            border-radius: 4px;
        }

        .note {
            background: #fff3cd;
            border-left: 4px solid #ffc107;
            padding: 15px;
            margin: 20px 0;
            border-radius: 4px;
        }

        .note strong {
            color: #856404;
        }

        code {
            background: #f8f9fa;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: 'Courier New', monospace;
            color: #c7254e;
        }
    </style>
</head>
<body>
    <div class="main-content">
        <div class="card">
            <h1>🎨 Custom Toolbar Example</h1>
            <p>This window demonstrates a frameless window with a custom toolbar.</p>

            <div class="note">
                <strong>Note:</strong> The custom title bar at the top is provided by the library's
                toolbar system. The native window decorations have been removed.
            </div>

            <h2>Features</h2>
            <ul class="feature-list">
                <li><strong>Frameless Window:</strong> No native OS decorations</li>
                <li><strong>Custom Toolbar:</strong> HTML/CSS-based title bar</li>
                <li><strong>Draggable:</strong> Click and drag the toolbar to move the window</li>
                <li><strong>Modern Design:</strong> Clean, minimalist appearance</li>
            </ul>

            <h2>Use Cases</h2>
            <ul class="feature-list">
                <li>Custom-branded applications</li>
                <li>Kiosk or presentation mode</li>
                <li>Modern desktop applications</li>
                <li>Splash screens and dialogs</li>
            </ul>

            <h2>Implementation</h2>
            <p>This window was created using:</p>
            <ul class="feature-list">
                <li><code>window.decorations = false</code> - Remove native frame</li>
                <li><code>window.toolbar = ToolbarOptions::default()</code> - Add custom toolbar</li>
                <li>Custom HTML content below the toolbar</li>
            </ul>

            <h2>Toolbar Options</h2>
            <p>The toolbar can be customized with:</p>
            <ul class="feature-list">
                <li>Custom title text</li>
                <li>Background color</li>
                <li>Height adjustment</li>
                <li>Window control buttons (minimize, maximize, close)</li>
            </ul>
        </div>
    </div>
</body>
</html>
"#;

    // Create viewer options with custom toolbar
    let mut options = ViewerOptions::inline_html(html_content);

    // Remove native window decorations
    options.window.decorations = false;

    // Configure the custom toolbar
    options.window.toolbar = ToolbarOptions {
        show: true,
        title_text: Some("Custom Toolbar Example".to_string()),
        background_color: Some("#3498db".to_string()),
        text_color: Some("#ffffff".to_string()),
        buttons: vec![],
    };

    // Window configuration
    options.window.width = Some(900);
    options.window.height = Some(700);
    options.window.resizable = true;

    // Enable devtools for inspection
    options.behaviour.enable_devtools = true;

    println!("Features:");
    println!("  - Frameless window (no native decorations)");
    println!("  - Custom toolbar with title: 'Custom Toolbar Example'");
    println!("  - Blue toolbar background (#3498db)");
    println!("  - Draggable via toolbar");
    println!("  - DevTools enabled (press F12)\n");

    println!("Window will open shortly...");
    println!("Try dragging the custom toolbar to move the window!\n");

    // Open the viewer
    html_view::open(options)?;

    println!("Viewer closed!");

    Ok(())
}
