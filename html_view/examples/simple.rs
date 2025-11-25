//! Simple example showing basic usage of html_view.

fn main() -> Result<(), html_view::ViewerError> {
    // Display simple HTML - blocks until window is closed
    html_view::show(include_str!("/home/jmg/code/rust/html_view/test.html"))?;

    println!("Window was closed!");

    Ok(())
}
