use html_view::ViewerOptions;

fn main() -> Result<(), html_view::ViewerError> {
    // Test the builder pattern with all new features
    let result = ViewerOptions::new()
        .title("Builder Test")
        .size(1200, 800)
        .no_decorations()
        .transparent()
        .always_on_top()
        .enable_notifications()
        .enable_dialogs()
        .toolbar(html_view::ToolbarOptions {
            show: true,
            title_text: Some("My Custom App".to_string()),
            background_color: Some("#2d2d2d".to_string()),
            text_color: Some("#ffffff".to_string()),
            buttons: vec![],
        })
        .show_html(r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {
                        margin: 0;
                        padding: 40px 20px 20px 20px;
                        font-family: system-ui, sans-serif;
                        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                        color: white;
                        min-height: 100vh;
                    }
                    h1 {
                        font-size: 2.5em;
                        margin: 0 0 20px 0;
                    }
                </style>
            </head>
            <body>
                <h1>Builder Pattern Test</h1>
                <p>This window demonstrates:</p>
                <ul>
                    <li>Custom title: "Builder Test"</li>
                    <li>Size: 1200x800</li>
                    <li>No decorations (frameless)</li>
                    <li>Transparent background support</li>
                    <li>Always on top</li>
                    <li>Notifications enabled</li>
                    <li>Dialogs enabled</li>
                    <li>Custom dark toolbar</li>
                </ul>
            </body>
            </html>
        "#)?;

    match result {
        html_view::ViewerResult::Blocking(status) => {
            println!("Viewer closed: {:?}", status.reason);
        }
        _ => unreachable!(),
    }

    Ok(())
}
