//! Example demonstrating base_dir for relative asset loading.
//!
//! Shows how to use base_dir to resolve relative paths in inline HTML.
//! When base_dir is set, relative paths in img, link, script tags, etc.
//! will be resolved against that directory.
//!
//! Run with: cargo run --example base_dir

use html_view::ViewerOptions;
use html_view_shared::ViewerContent;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for demonstration
    let temp_dir = std::env::temp_dir().join("html_view_base_dir_example");
    fs::create_dir_all(&temp_dir)?;

    // Create a sample CSS file
    let css_path = temp_dir.join("styles.css");
    fs::write(
        &css_path,
        r#"
body {
    font-family: system-ui, -apple-system, sans-serif;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 40px;
    margin: 0;
    min-height: 100vh;
}

.container {
    max-width: 800px;
    margin: 0 auto;
    background: rgba(255, 255, 255, 0.1);
    padding: 30px;
    border-radius: 20px;
    backdrop-filter: blur(10px);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

h1 {
    margin-top: 0;
    font-size: 2.5em;
    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
}

.info-box {
    background: rgba(0, 0, 0, 0.2);
    padding: 20px;
    border-radius: 10px;
    margin: 20px 0;
}

.path {
    background: rgba(255, 255, 255, 0.2);
    padding: 10px;
    border-radius: 5px;
    font-family: monospace;
    margin: 10px 0;
    word-break: break-all;
}

.success {
    color: #4CAF50;
    font-weight: bold;
}
    "#,
    )?;

    // Create a sample JavaScript file
    let js_path = temp_dir.join("script.js");
    fs::write(
        &js_path,
        r#"
console.log('JavaScript loaded successfully from relative path!');

document.addEventListener('DOMContentLoaded', function() {
    const statusElement = document.getElementById('js-status');
    if (statusElement) {
        statusElement.textContent = '✓ JavaScript loaded and executed';
        statusElement.className = 'success';
    }
});

function showAlert() {
    alert('JavaScript is working!\n\nThis script was loaded from:\nscript.js\n\nRelative to the base_dir.');
}
    "#,
    )?;

    // HTML with relative asset references
    let html = format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>Base Directory Example</title>
    <!-- This CSS file is loaded using a relative path -->
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div class="container">
        <h1>Base Directory Example</h1>

        <div class="info-box">
            <h2>What is base_dir?</h2>
            <p>
                When using inline HTML with the <code>ViewerContent::InlineHtml</code> variant,
                you can optionally specify a <code>base_dir</code>. This directory is used as
                the base for resolving relative paths in your HTML.
            </p>
        </div>

        <div class="info-box">
            <h2>Asset Loading Status</h2>
            <p>
                <strong>CSS:</strong> ✓ Loaded successfully from <code>styles.css</code>
            </p>
            <p>
                <strong>JavaScript:</strong> <span id="js-status">Loading...</span>
            </p>
        </div>

        <div class="info-box">
            <h2>Configuration</h2>
            <p><strong>Base Directory:</strong></p>
            <div class="path">{}</div>

            <p><strong>Loaded Assets:</strong></p>
            <ul>
                <li><code>styles.css</code> - Relative CSS file</li>
                <li><code>script.js</code> - Relative JavaScript file</li>
            </ul>
        </div>

        <div class="info-box">
            <h2>Code Example</h2>
            <pre style="background: rgba(0,0,0,0.3); padding: 15px; border-radius: 8px; overflow-x: auto;"><code>let mut opts = ViewerOptions::inline_html(html);

opts.content = ViewerContent::InlineHtml {{
    html: html.to_string(),
    base_dir: Some(base_dir.clone()),
}};</code></pre>
        </div>

        <button onclick="showAlert()" style="
            padding: 12px 24px;
            font-size: 16px;
            cursor: pointer;
            background: white;
            color: #667eea;
            border: none;
            border-radius: 8px;
            font-weight: 600;
            margin-top: 20px;
        ">Test JavaScript</button>
    </div>

    <!-- This JavaScript file is loaded using a relative path -->
    <script src="script.js"></script>
</body>
</html>
    "#,
        temp_dir.display()
    );

    let mut opts = ViewerOptions::inline_html(&html);

    // Set base directory to the temp directory
    // This allows relative paths in HTML to resolve correctly
    opts.content = ViewerContent::InlineHtml {
        html: html.clone(),
        base_dir: Some(temp_dir.clone()),
    };

    opts.window.width = Some(850);
    opts.window.height = Some(750);
    opts.window.title = Some("Base Directory Example".to_string());

    println!("Base Directory Example");
    println!("======================\n");
    println!("Created temporary directory with assets:");
    println!("  {}", temp_dir.display());
    println!();
    println!("Assets created:");
    println!("  ✓ styles.css  - CSS stylesheet");
    println!("  ✓ script.js   - JavaScript file");
    println!();
    println!("Opening viewer with base_dir set to temp directory...");
    println!("Relative paths in HTML will resolve against this directory.");
    println!();

    html_view::open(opts)?;

    // Cleanup
    println!("Cleaning up temporary files...");
    fs::remove_dir_all(&temp_dir)?;
    println!("✓ Cleanup complete");

    Ok(())
}
