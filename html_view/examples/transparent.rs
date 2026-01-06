//! Example showing transparent/frameless windows.
//!
//! Demonstrates:
//! - Transparent window background
//! - No decorations (frameless)
//! - Alpha transparency in HTML/CSS
//! - Always on top behavior
//!
//! Run with: cargo run --example transparent

use html_view::ViewerOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <style>
        body {
            background: transparent;
            margin: 0;
            padding: 0;
            overflow: hidden;
            font-family: system-ui, -apple-system, sans-serif;
        }

        .floating-card {
            background: rgba(255, 255, 255, 0.95);
            border-radius: 20px;
            padding: 30px;
            margin: 20px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            backdrop-filter: blur(10px);
            -webkit-backdrop-filter: blur(10px);
            animation: fadeIn 0.5s ease-out;
        }

        @keyframes fadeIn {
            from {
                opacity: 0;
                transform: translateY(-20px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        h1 {
            margin: 0 0 15px 0;
            color: #333;
            font-size: 24px;
        }

        p {
            color: #666;
            line-height: 1.6;
            margin: 10px 0;
        }

        .close-btn {
            position: absolute;
            top: 15px;
            right: 15px;
            width: 32px;
            height: 32px;
            border-radius: 50%;
            background: rgba(255, 59, 48, 0.8);
            color: white;
            border: none;
            cursor: pointer;
            font-size: 20px;
            font-weight: bold;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: all 0.2s ease;
        }

        .close-btn:hover {
            background: rgba(255, 59, 48, 1);
            transform: scale(1.1);
        }

        .close-btn:active {
            transform: scale(0.95);
        }

        .feature-list {
            list-style: none;
            padding: 0;
            margin: 20px 0 0 0;
        }

        .feature-list li {
            padding: 8px 0;
            color: #555;
        }

        .feature-list li::before {
            content: "✓ ";
            color: #4CAF50;
            font-weight: bold;
            margin-right: 8px;
        }

        .gradient-border {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            padding: 3px;
            border-radius: 22px;
        }

        .inner-card {
            background: rgba(255, 255, 255, 0.95);
            border-radius: 20px;
            padding: 27px;
        }

        .badge {
            display: inline-block;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 4px 12px;
            border-radius: 12px;
            font-size: 12px;
            font-weight: 600;
            margin-bottom: 10px;
        }
    </style>
</head>
<body>
    <div class="gradient-border">
        <div class="inner-card">
            <button class="close-btn" onclick="window.close()" title="Close window">×</button>

            <div class="badge">TRANSPARENT MODE</div>

            <h1>Frameless Transparent Window</h1>

            <p>This window demonstrates several advanced features:</p>

            <ul class="feature-list">
                <li>Transparent background</li>
                <li>No window decorations (frameless)</li>
                <li>Always on top of other windows</li>
                <li>Glass morphism effect</li>
                <li>Smooth animations</li>
            </ul>

            <p style="margin-top: 20px; font-size: 14px; color: #888;">
                Click the red button in the top-right corner to close this window.
            </p>
        </div>
    </div>

    <script>
        // Make the window draggable (useful for frameless windows)
        const card = document.querySelector('.gradient-border');
        let isDragging = false;
        let currentX;
        let currentY;
        let initialX;
        let initialY;

        card.addEventListener('mousedown', function(e) {
            // Don't drag if clicking the close button
            if (e.target.classList.contains('close-btn')) {
                return;
            }

            isDragging = true;
            initialX = e.clientX;
            initialY = e.clientY;
        });

        document.addEventListener('mousemove', function(e) {
            if (isDragging) {
                currentX = e.clientX - initialX;
                currentY = e.clientY - initialY;
                // Note: Window dragging would need native support
                // This is just for demonstration
            }
        });

        document.addEventListener('mouseup', function() {
            isDragging = false;
        });
    </script>
</body>
</html>
    "#;

    let mut opts = ViewerOptions::inline_html(html);

    // Transparent window settings
    opts.window.transparent = true;
    opts.window.decorations = false;
    opts.window.always_on_top = true;

    opts.window.width = Some(450);
    opts.window.height = Some(400);

    println!("Opening transparent frameless window...");
    println!();
    println!("Window features:");
    println!("  ✓ Transparent background");
    println!("  ✓ No decorations (frameless)");
    println!("  ✓ Always on top");
    println!();
    println!("Click the red × button to close.");
    println!();

    html_view::open(opts)?;

    Ok(())
}
