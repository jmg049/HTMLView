//! Simple example showing basic usage of html_view.

fn main() -> Result<(), html_view::ViewerError> {
    // Display simple HTML - blocks until window is closed
    html_view::show(
        "<!DOCTYPE html>
        <html>
        <body style=\"padding: 40px; font-family: Arial;\">
            <h1>Simple Example</h1>
            <p>This is a simple HTML viewer example.</p>
        </body>
        </html>",
    )?;

    println!("Window was closed!");

    Ok(())
}
