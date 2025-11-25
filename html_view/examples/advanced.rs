//! Advanced example showing non-blocking mode and custom configuration.

use html_view::{ViewerOptions, ViewerResult, ViewerWaitMode};

fn main() -> Result<(), html_view::ViewerError> {
    // Create custom options
    let mut options = ViewerOptions::inline_html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    padding: 40px;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    color: white;
                }
                h1 { font-size: 3em; }
            </style>
        </head>
        <body>
            <h1>Advanced Example</h1>
            <p>This window was opened in non-blocking mode.</p>
            <p>The program continues running while this is open.</p>
        </body>
        </html>
        "#,
    );

    // Customize window
    options.window.width = Some(800);
    options.window.height = Some(600);
    options.window.title = Some("Advanced Example".to_string());

    // Use non-blocking mode
    options.wait = ViewerWaitMode::NonBlocking;

    // Open the viewer
    match html_view::open(options)? {
        ViewerResult::NonBlocking(mut handle) => {
            println!("Viewer opened with ID: {}", handle.id);
            println!("Doing other work while viewer is open...");

            // Simulate doing other work
            for i in 1..=5 {
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Working... {}/5", i);

                // Check if viewer is still open
                if let Some(status) = handle.try_wait()? {
                    println!("Viewer closed early: {:?}", status.reason);
                    return Ok(());
                }
            }

            println!("Waiting for viewer to close...");
            let status = handle.wait()?;
            println!("Viewer closed: {:?}", status.reason);
        }
        ViewerResult::Blocking(_) => {
            unreachable!("We set NonBlocking mode");
        }
    }

    Ok(())
}
