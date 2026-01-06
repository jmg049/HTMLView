//! Comprehensive example showcasing all available configuration options.
//!
//! This example demonstrates:
//! - All ViewerOptions fields
//! - Window configuration options
//! - Behaviour and security settings
//! - Environment options
//! - Dialog configuration
//! - Complete reference for all capabilities

use html_view::ViewerOptions;
use html_view_shared::{
    BehaviourOptions, DialogOptions, EnvironmentOptions, ToolbarOptions, WindowOptions,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("All Options Example");
    println!("===================\n");
    println!("This example demonstrates every available configuration option.");
    println!("Review the source code to see all available settings.\n");

    // Create comprehensive HTML that demonstrates the capabilities
    let html_content = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>All Options Showcase</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: #333;
            padding: 20px;
        }

        .container {
            max-width: 1000px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            overflow: hidden;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
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
            font-size: 1.1em;
            opacity: 0.9;
        }

        .content {
            padding: 40px;
        }

        .section {
            margin-bottom: 35px;
            padding-bottom: 35px;
            border-bottom: 2px solid #eee;
        }

        .section:last-child {
            border-bottom: none;
        }

        h2 {
            color: #667eea;
            margin-bottom: 20px;
            font-size: 1.8em;
        }

        h3 {
            color: #764ba2;
            margin-top: 20px;
            margin-bottom: 12px;
            font-size: 1.3em;
        }

        .option-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 15px;
            margin-top: 15px;
        }

        .option-card {
            background: #f8f9fa;
            padding: 15px;
            border-radius: 8px;
            border-left: 4px solid #667eea;
        }

        .option-name {
            font-weight: bold;
            color: #2c3e50;
            margin-bottom: 5px;
        }

        .option-value {
            color: #7f8c8d;
            font-family: 'Courier New', monospace;
            font-size: 0.9em;
        }

        .demo-section {
            background: #e8f4f8;
            padding: 20px;
            border-radius: 8px;
            margin: 20px 0;
        }

        .demo-section h3 {
            margin-top: 0;
        }

        button {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 12px 24px;
            font-size: 16px;
            border-radius: 6px;
            cursor: pointer;
            margin: 5px;
            transition: transform 0.2s;
        }

        button:hover {
            transform: translateY(-2px);
        }

        button:active {
            transform: translateY(0);
        }

        #output {
            margin-top: 15px;
            padding: 15px;
            background: white;
            border-radius: 6px;
            min-height: 20px;
        }

        .info-box {
            background: #fff3cd;
            border-left: 4px solid #ffc107;
            padding: 15px;
            margin: 20px 0;
            border-radius: 4px;
        }

        code {
            background: #f8f9fa;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: 'Courier New', monospace;
            color: #c7254e;
        }

        .feature-badge {
            display: inline-block;
            background: #28a745;
            color: white;
            padding: 4px 10px;
            border-radius: 12px;
            font-size: 0.8em;
            margin-left: 8px;
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>⚙️ All Options Showcase</h1>
            <p class="subtitle">Complete reference for HTMLView configuration</p>
        </header>

        <div class="content">
            <div class="section">
                <h2>Window Options</h2>
                <div class="option-grid">
                    <div class="option-card">
                        <div class="option-name">Title</div>
                        <div class="option-value">Custom window title</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">Size</div>
                        <div class="option-value">1100x800 pixels</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">Position</div>
                        <div class="option-value">x=100, y=100</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">Resizable</div>
                        <div class="option-value">true</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">Decorations</div>
                        <div class="option-value">true (native frame)</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">Always on Top</div>
                        <div class="option-value">false</div>
                    </div>
                </div>
            </div>

            <div class="section">
                <h2>Behaviour Options <span class="feature-badge">Security</span></h2>

                <h3>Content Loading</h3>
                <div class="option-grid">
                    <div class="option-card">
                        <div class="option-name">Remote Content</div>
                        <div class="option-value">Disabled (secure default)</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">External Navigation</div>
                        <div class="option-value">Disabled (secure default)</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">Allowed Domains</div>
                        <div class="option-value">None (all blocked)</div>
                    </div>
                </div>

                <h3>Developer Tools</h3>
                <div class="option-grid">
                    <div class="option-card">
                        <div class="option-name">DevTools</div>
                        <div class="option-value">Enabled (press F12)</div>
                    </div>
                </div>

                <h3>User Interaction</h3>
                <div class="option-grid">
                    <div class="option-card">
                        <div class="option-name">Notifications</div>
                        <div class="option-value">Enabled</div>
                    </div>
                </div>
            </div>

            <div class="section">
                <h2>Dialog Options</h2>
                <div class="option-grid">
                    <div class="option-card">
                        <div class="option-name">File Dialogs</div>
                        <div class="option-value">Enabled</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">Message Dialogs</div>
                        <div class="option-value">Enabled</div>
                    </div>
                </div>

                <div class="demo-section">
                    <h3>Try the Dialogs</h3>
                    <button onclick="testNotification()">Show Notification</button>
                    <button onclick="testAlert()">Show Alert</button>
                    <button onclick="testConfirm()">Show Confirm</button>
                    <div id="output"></div>
                </div>
            </div>

            <div class="section">
                <h2>Environment Options</h2>
                <div class="option-grid">
                    <div class="option-card">
                        <div class="option-name">Working Directory</div>
                        <div class="option-value">Current directory</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">Timeout</div>
                        <div class="option-value">None (manual close)</div>
                    </div>
                </div>
            </div>

            <div class="section">
                <h2>Content Types</h2>
                <div class="info-box">
                    This example uses <code>InlineHtml</code> content type. Other available types:
                </div>
                <div class="option-grid">
                    <div class="option-card">
                        <div class="option-name">InlineHtml</div>
                        <div class="option-value">HTML string (this example)</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">LocalFile</div>
                        <div class="option-value">Load from file path</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">AppDir</div>
                        <div class="option-value">Serve directory as app</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">RemoteUrl</div>
                        <div class="option-value">Load from URL</div>
                    </div>
                </div>
            </div>

            <div class="section">
                <h2>Additional Features</h2>
                <div class="option-grid">
                    <div class="option-card">
                        <div class="option-name">Wait Mode</div>
                        <div class="option-value">Blocking (waits for close)</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">Custom Toolbar</div>
                        <div class="option-value">Available for frameless windows</div>
                    </div>
                    <div class="option-card">
                        <div class="option-name">Transparency</div>
                        <div class="option-value">Supported on compatible platforms</div>
                    </div>
                </div>
            </div>

            <div class="info-box">
                <strong>💡 Tip:</strong> Open DevTools (F12) to inspect the DOM, test JavaScript,
                and experiment with the configuration. Check the source code of this example to see
                how each option is configured.
            </div>
        </div>
    </div>

    <script>
        console.log('HTMLView All Options Example');
        console.log('=============================');
        console.log('DevTools enabled! Try inspecting elements and testing features.');

        function testNotification() {
            if ('Notification' in window && Notification.permission === 'granted') {
                new Notification('HTMLView Notification', {
                    body: 'Notifications are working!',
                    icon: 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><text y="75" font-size="75">✅</text></svg>'
                });
                document.getElementById('output').innerHTML = '<strong>✅ Notification sent!</strong>';
            } else if ('Notification' in window) {
                Notification.requestPermission().then(permission => {
                    if (permission === 'granted') {
                        testNotification();
                    }
                });
            } else {
                document.getElementById('output').innerHTML = '<strong>⚠️ Notifications not supported</strong>';
            }
        }

        function testAlert() {
            alert('This is an alert dialog!\\n\\nMessage dialogs are enabled.');
            document.getElementById('output').innerHTML = '<strong>✅ Alert shown!</strong>';
        }

        function testConfirm() {
            const result = confirm('This is a confirm dialog.\\n\\nClick OK or Cancel.');
            document.getElementById('output').innerHTML =
                `<strong>✅ Confirm result:</strong> ${result ? 'OK' : 'Cancel'}`;
        }

        // Request notification permission on load
        if ('Notification' in window && Notification.permission === 'default') {
            Notification.requestPermission();
        }
    </script>
</body>
</html>
"#;

    // ============================================================================
    // COMPREHENSIVE OPTIONS CONFIGURATION
    // ============================================================================

    let mut options = ViewerOptions::inline_html(html_content);

    // ----------------------------------------------------------------------------
    // Window Options
    // ----------------------------------------------------------------------------
    #[allow(deprecated)]
    {
        options.window = WindowOptions {
            // Window title displayed in title bar
            title: Some("HTMLView - All Options Showcase".to_string()),

            // Window dimensions
            width: Some(1100),
            height: Some(800),

            // Window position (None = system default, centered)
            x: Some(100),
            y: Some(100),

            // Whether the window can be resized
            resizable: true,

            // Start window maximized
            maximised: false,

            // Start window in fullscreen mode
            fullscreen: false,

            // Show native window decorations (title bar, borders)
            decorations: true,

            // Make window background transparent (platform-dependent)
            transparent: false,

            // Keep window above other windows
            always_on_top: false,

            // Window theme: "light", "dark", or "system" (deprecated, use theme_enum)
            theme: None,

            // Window theme enum (preferred over string-based theme)
            theme_enum: None, // Can be Some(WindowTheme::Light), Some(WindowTheme::Dark), or Some(WindowTheme::System)

            // Background color in hex format
            background_color: None,

            // Custom toolbar for frameless windows
            // Only visible when decorations = false
            toolbar: ToolbarOptions {
                show: false,
                title_text: None,
                background_color: None,
                text_color: None,
                buttons: vec![],
            },
        };
    }

    // ----------------------------------------------------------------------------
    // Behaviour Options (Security & Features)
    // ----------------------------------------------------------------------------
    options.behaviour = BehaviourOptions {
        // Allow loading remote content (images, scripts from URLs)
        // WARNING: Enable only for trusted content
        allow_remote_content: false,

        // Allow navigation to external URLs
        // If false, clicking links does nothing
        allow_external_navigation: false,

        // Whitelist specific domains for navigation
        // Only used when allow_external_navigation = true
        // Example: Some(vec!["example.com".to_string(), "trusted.org".to_string()])
        allowed_domains: None,

        // Enable developer tools (F12 to open)
        // Useful for debugging, disable in production
        enable_devtools: true,

        // Allow JavaScript to show system notifications
        allow_notifications: true,
    };

    // ----------------------------------------------------------------------------
    // Environment Options
    // ----------------------------------------------------------------------------
    options.environment = EnvironmentOptions {
        // Working directory for resolving relative paths
        // None = current directory
        working_dir: None,

        // Auto-close window after N seconds
        // None = stays open until manually closed
        // Example: Some(30) for 30-second timeout
        timeout_seconds: None,
    };

    // ----------------------------------------------------------------------------
    // Dialog Options
    // ----------------------------------------------------------------------------
    options.dialog = DialogOptions {
        // Allow JavaScript to open file selection dialogs
        // Enables: <input type="file">, showOpenFilePicker(), etc.
        allow_file_dialogs: true,

        // Allow JavaScript to show message dialogs
        // Enables: alert(), confirm(), prompt()
        allow_message_dialogs: true,
    };

    // ----------------------------------------------------------------------------
    // Content Type (already set via inline_html)
    // ----------------------------------------------------------------------------
    // Other content types available:
    //
    // ViewerContent::LocalFile { path: PathBuf }
    // ViewerContent::AppDir { root: PathBuf, entry: Option<String> }
    // ViewerContent::RemoteUrl { url: Url }

    // ----------------------------------------------------------------------------
    // Wait Mode
    // ----------------------------------------------------------------------------
    // ViewerWaitMode::Blocking - Function blocks until window closes (default)
    // ViewerWaitMode::NonBlocking - Returns immediately with handle
    // options.wait = ViewerWaitMode::NonBlocking;

    println!("Configured Options:");
    println!("===================");
    println!("Window:");
    println!("  Title: {}", options.window.title.as_ref().unwrap());
    println!(
        "  Size: {}x{}",
        options.window.width.unwrap(),
        options.window.height.unwrap()
    );
    println!(
        "  Position: ({}, {})",
        options.window.x.unwrap(),
        options.window.y.unwrap()
    );
    println!("  Resizable: {}", options.window.resizable);
    println!("  Decorations: {}", options.window.decorations);
    println!("  Always on top: {}", options.window.always_on_top);
    println!("  Transparent: {}", options.window.transparent);

    println!("\nBehaviour:");
    println!(
        "  Remote content: {}",
        options.behaviour.allow_remote_content
    );
    println!(
        "  External navigation: {}",
        options.behaviour.allow_external_navigation
    );
    println!("  DevTools: {}", options.behaviour.enable_devtools);
    println!("  Notifications: {}", options.behaviour.allow_notifications);

    println!("\nDialogs:");
    println!("  File dialogs: {}", options.dialog.allow_file_dialogs);
    println!(
        "  Message dialogs: {}",
        options.dialog.allow_message_dialogs
    );

    println!("\nEnvironment:");
    println!("  Working directory: {:?}", options.environment.working_dir);
    println!("  Timeout: {:?}", options.environment.timeout_seconds);

    println!("\n===================");
    println!("\nOpening viewer with all options configured...");
    println!("Press F12 to open DevTools and explore!");
    println!("Try the interactive dialog buttons.\n");

    // Open the viewer
    html_view::open(options)?;

    println!("Viewer closed!");

    Ok(())
}
