//! Example showing how to load a local HTML file with assets.
//!
//! This example demonstrates:
//! - Loading HTML from a file on disk
//! - Handling relative paths for CSS, JS, and images
//! - Error handling for missing files
//! - Custom window configuration for file viewing

use html_view::ViewerOptions;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // For this example, we'll create a temporary HTML file
    let temp_dir = std::env::temp_dir();
    let html_path = temp_dir.join("html_view_example.html");

    // Create example HTML with inline CSS
    let html_content = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Local File Example</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 40px;
            background: #f5f5f5;
        }
        .card {
            background: white;
            border-radius: 8px;
            padding: 30px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        h1 {
            color: #2c3e50;
            margin-top: 0;
        }
        .info {
            background: #e8f4f8;
            padding: 15px;
            border-left: 4px solid #3498db;
            margin: 20px 0;
        }
        code {
            background: #f8f9fa;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: 'Courier New', monospace;
        }
    </style>
</head>
<body>
    <div class="card">
        <h1>📄 Local File Example</h1>
        <p>This HTML was loaded from a local file!</p>

        <div class="info">
            <strong>File Location:</strong><br>
            <code>"{}"</code>
        </div>

        <h2>Features</h2>
        <ul>
            <li>HTML loaded from filesystem</li>
            <li>Inline CSS styles working correctly</li>
            <li>Relative paths resolve properly</li>
            <li>File assets can be referenced</li>
        </ul>

        <h2>Use Cases</h2>
        <ul>
            <li>Viewing generated HTML reports</li>
            <li>Documentation browsing</li>
            <li>Static site preview</li>
            <li>HTML email templates</li>
        </ul>
    </div>

    <script>
        // JavaScript also works from local files
        console.log('JavaScript loaded from local HTML file!');

        // Add timestamp
        const now = new Date().toLocaleString();
        document.body.innerHTML += '<p style="text-align: center; color: #7f8c8d; margin-top: 40px;">Loaded at: ' + now + '</p>';
    </script>
</body>
</html>
"#;

    // Write the HTML file
    let formatted_html = html_content.replace("{}", &html_path.display().to_string());
    fs::write(&html_path, formatted_html)?;

    println!("Created temporary HTML file at: {}", html_path.display());
    println!("Opening HTML_view...\n");

    // Create viewer options for the file
    let mut options = ViewerOptions::local_file(html_path.clone());

    // Customize the window
    options.window.title = Some("Local File Example".to_string());
    options.window.width = Some(900);
    options.window.height = Some(700);

    // Enable devtools for inspection
    options.behaviour.enable_devtools = true;

    // Open the viewer
    html_view::open(options)?;

    println!("Viewer closed!");

    // Clean up the temporary file
    fs::remove_file(&html_path)?;
    println!("Cleaned up temporary file");

    Ok(())
}
