//! Example showing timeout functionality.

use html_view::ViewerOptions;

fn main() -> Result<(), html_view::ViewerError> {
    let mut options = ViewerOptions::inline_html(
        r#"
        <!DOCTYPE html>
        <html>
        <body style="padding: 40px; font-family: Arial;">
            <h1>Timeout Example</h1>
            <p>This window will automatically close after 5 seconds.</p>
            <p id="countdown">5</p>
            <script>
                let count = 5;
                setInterval(() => {
                    count--;
                    if (count >= 0) {
                        document.getElementById('countdown').textContent = count;
                    }
                }, 1000);
            </script>
        </body>
        </html>
        "#,
    );

    options.environment.timeout_seconds = Some(5);

    println!("Opening viewer with 5 second timeout...");

    let html = match options.content {
        html_view::ViewerContent::InlineHtml { html, .. } => html,
        _ => unreachable!(),
    };

    html_view::show(html)?;

    println!("Viewer closed (likely due to timeout)!");

    Ok(())
}
