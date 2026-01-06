use html_view::{ViewerOptions, ViewerResult, ViewerWaitMode};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting refresh demo...");

    // Create initial HTML content
    let initial_html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    margin: 0;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                }
                .counter {
                    background: white;
                    padding: 40px;
                    border-radius: 20px;
                    box-shadow: 0 10px 40px rgba(0,0,0,0.3);
                    text-align: center;
                }
                h1 {
                    color: #667eea;
                    font-size: 48px;
                    margin: 0;
                }
                p {
                    color: #666;
                    font-size: 18px;
                    margin-top: 10px;
                }
            </style>
        </head>
        <body>
            <div class="counter">
                <h1>0</h1>
                <p>Counter (refreshing every second)</p>
            </div>
        </body>
        </html>
    "#;

    // Create viewer options in non-blocking mode
    let mut options = ViewerOptions::inline_html(initial_html);
    options.wait = ViewerWaitMode::NonBlocking;
    options.window.title = Some("Refresh Demo - Live Counter".to_string());
    options.window.width = Some(600);
    options.window.height = Some(400);

    // Open the viewer
    println!("Opening viewer window...");
    let result = html_view::open(options)?;

    if let ViewerResult::NonBlocking(mut handle) = result {
        println!("Viewer opened successfully!");
        println!("The counter will update every second for 10 seconds.");
        println!("You can close the window at any time.");

        // Update the counter every second for 10 iterations
        for i in 1..=10 {
            // Wait 1 second
            thread::sleep(Duration::from_secs(1));

            // Check if the window is still open
            if let Some(status) = handle.try_wait()? {
                println!("Window closed by user: {:?}", status.reason);
                break;
            }

            // Generate new HTML with updated counter
            let updated_html = format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <style>
                        body {{
                            font-family: Arial, sans-serif;
                            display: flex;
                            justify-content: center;
                            align-items: center;
                            height: 100vh;
                            margin: 0;
                            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                        }}
                        .counter {{
                            background: white;
                            padding: 40px;
                            border-radius: 20px;
                            box-shadow: 0 10px 40px rgba(0,0,0,0.3);
                            text-align: center;
                            animation: pulse 0.5s ease;
                        }}
                        @keyframes pulse {{
                            0% {{ transform: scale(1); }}
                            50% {{ transform: scale(1.05); }}
                            100% {{ transform: scale(1); }}
                        }}
                        h1 {{
                            color: #667eea;
                            font-size: 48px;
                            margin: 0;
                        }}
                        p {{
                            color: #666;
                            font-size: 18px;
                            margin-top: 10px;
                        }}
                    </style>
                </head>
                <body>
                    <div class="counter">
                        <h1>{}</h1>
                        <p>Counter (refreshing every second)</p>
                    </div>
                </body>
                </html>
                "#,
                i
            );

            // Refresh the viewer with new content
            println!("Refreshing to counter value: {}", i);
            handle.refresh_html(&updated_html)?;
        }

        println!("Demo complete! Closing viewer...");
        thread::sleep(Duration::from_secs(1));
        handle.terminate()?;
    }

    Ok(())
}
