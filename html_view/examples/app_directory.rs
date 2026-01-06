//! Example showing how to serve an HTML application directory.
//!
//! This example demonstrates:
//! - Serving a complete application directory
//! - Setting a custom entry point
//! - Handling multiple files (HTML, CSS, JS, images)
//! - Use case for built web applications

use html_view::ViewerOptions;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary app directory structure
    let temp_dir = std::env::temp_dir().join("html_view_app_example");
    fs::create_dir_all(&temp_dir)?;

    // Create index.html
    let index_html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>App Directory Example</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div class="container">
        <header>
            <h1>🚀 Application Directory Example</h1>
            <p class="subtitle">Serving a complete web application</p>
        </header>

        <main>
            <section class="feature">
                <h2>What is this?</h2>
                <p>This is a complete mini web application served from a directory.</p>
                <p>It includes:</p>
                <ul>
                    <li><strong>index.html</strong> - This main file</li>
                    <li><strong>styles.css</strong> - External stylesheet</li>
                    <li><strong>app.js</strong> - External JavaScript</li>
                </ul>
            </section>

            <section class="feature">
                <h2>Use Cases</h2>
                <ul>
                    <li>Previewing built React/Vue/Svelte apps</li>
                    <li>Viewing documentation sites</li>
                    <li>Testing static site generators</li>
                    <li>Displaying bundled applications</li>
                </ul>
            </section>

            <section class="feature">
                <h2>Interactive Demo</h2>
                <button id="testButton" class="btn">Click Me!</button>
                <p id="output"></p>
            </section>
        </main>

        <footer>
            <p>All assets loaded from: <code>./app_directory_example/</code></p>
        </footer>
    </div>

    <script src="app.js"></script>
</body>
</html>
"#;

    // Create styles.css
    let styles_css = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    line-height: 1.6;
    color: #333;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    min-height: 100vh;
    padding: 20px;
}

.container {
    max-width: 800px;
    margin: 0 auto;
    background: white;
    border-radius: 10px;
    overflow: hidden;
    box-shadow: 0 10px 40px rgba(0,0,0,0.2);
}

header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 40px;
    text-align: center;
}

header h1 {
    font-size: 2.5em;
    margin-bottom: 10px;
}

.subtitle {
    font-size: 1.2em;
    opacity: 0.9;
}

main {
    padding: 40px;
}

.feature {
    margin-bottom: 30px;
    padding-bottom: 30px;
    border-bottom: 1px solid #eee;
}

.feature:last-child {
    border-bottom: none;
}

.feature h2 {
    color: #667eea;
    margin-bottom: 15px;
}

.feature ul {
    margin-left: 20px;
}

.feature li {
    margin: 10px 0;
}

.btn {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    padding: 12px 30px;
    font-size: 16px;
    border-radius: 5px;
    cursor: pointer;
    transition: transform 0.2s;
}

.btn:hover {
    transform: translateY(-2px);
}

.btn:active {
    transform: translateY(0);
}

#output {
    margin-top: 15px;
    padding: 15px;
    background: #f8f9fa;
    border-radius: 5px;
    min-height: 20px;
}

footer {
    background: #f8f9fa;
    padding: 20px;
    text-align: center;
    font-size: 0.9em;
    color: #666;
}

code {
    background: #e8f4f8;
    padding: 2px 8px;
    border-radius: 3px;
    font-family: 'Courier New', monospace;
    color: #333;
}
"#;

    // Create app.js
    let app_js = r#"
console.log('JavaScript loaded from external file!');

document.addEventListener('DOMContentLoaded', () => {
    const button = document.getElementById('testButton');
    const output = document.getElementById('output');

    let clickCount = 0;

    button.addEventListener('click', () => {
        clickCount++;
        output.innerHTML = `
            <strong>Button clicked ${clickCount} time${clickCount === 1 ? '' : 's'}!</strong><br>
            <em>External JavaScript is working correctly.</em>
        `;
    });

    // Display loaded message
    console.log('App initialized successfully!');
});
"#;

    // Write all files
    fs::write(temp_dir.join("index.html"), index_html)?;
    fs::write(temp_dir.join("styles.css"), styles_css)?;
    fs::write(temp_dir.join("app.js"), app_js)?;

    println!("Created application directory at: {}", temp_dir.display());
    println!("  - index.html");
    println!("  - styles.css");
    println!("  - app.js");
    println!("\nOpening HTML viewer...\n");

    // Create viewer options for the directory
    let mut options = ViewerOptions::app_dir(temp_dir.clone());

    // Optional: specify a different entry point
    // If you had "main.html" instead of "index.html", you would do:
    // if let ViewerContent::AppDir { entry, .. } = &mut options.content {
    //     *entry = Some("main.html".to_string());
    // }

    // Customize the window
    options.window.title = Some("App Directory Example".to_string());
    options.window.width = Some(900);
    options.window.height = Some(750);

    // Enable devtools to see that resources loaded correctly
    options.behaviour.enable_devtools = true;

    // Open the viewer
    html_view::open(options)?;

    println!("Viewer closed!");

    // Clean up the temporary directory
    fs::remove_dir_all(&temp_dir)?;
    println!("Cleaned up temporary directory");

    Ok(())
}
