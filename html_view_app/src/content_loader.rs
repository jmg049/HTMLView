use anyhow::{Context, Result};
use html_view_shared::{ToolbarOptions, ViewerContent, ViewerRequest};
use tauri::WebviewWindow;
use url::Url;

/// Load content into the window based on ViewerContent type.
/// Load content into the window based on ViewerContent type.
pub fn load_content(window: &WebviewWindow, request: &ViewerRequest) -> Result<()> {
    let toolbar_html = if request.window.toolbar.show {
        Some(generate_toolbar_html(&request.window.toolbar))
    } else {
        None
    };

    match &request.content {
        ViewerContent::InlineHtml { html, base_dir: _ } => {
            let mut final_html = html.clone();
            if let Some(toolbar) = &toolbar_html {
                final_html = inject_into_html(&final_html, toolbar, None);
            }
            load_inline_html(window, &final_html)?;
        }
        ViewerContent::LocalFile { path } => {
            if let Some(toolbar) = &toolbar_html {
                // Read file, inject base and toolbar
                let content = std::fs::read_to_string(path).context("Failed to read HTML file")?;
                let base_path = path
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| std::path::PathBuf::from("."));
                let abs_path = std::fs::canonicalize(&base_path)
                    .context("Failed to canonicalize base path")?;
                let base_url = Url::from_file_path(&abs_path)
                    .map_err(|_| anyhow::anyhow!("Invalid file path {:?}", abs_path))?;
                let final_html = inject_into_html(&content, toolbar, Some(base_url.as_str()));
                load_inline_html(window, &final_html)?;
            } else {
                // Use file URL to ensure relative paths (images, css) work correctly
                let abs_path =
                    std::fs::canonicalize(path).context("Failed to canonicalize file path")?;
                let url = Url::from_file_path(&abs_path)
                    .map_err(|_| anyhow::anyhow!("Invalid file path {:?}", abs_path))?;
                window
                    .navigate(url)
                    .context("Failed to navigate to local file")?;
            }
        }
        ViewerContent::AppDir { root, entry } => {
            let entry_file = entry.as_deref().unwrap_or("index.html");
            let full_path = root.join(entry_file);

            if let Some(toolbar) = &toolbar_html {
                let content =
                    std::fs::read_to_string(&full_path).context("Failed to read app entry file")?;
                let root =
                    std::fs::canonicalize(root).context("Failed to canonicalize app root")?;
                let base_url =
                    Url::from_file_path(root).map_err(|_| anyhow::anyhow!("Invalid file path"))?;
                let final_html = inject_into_html(&content, toolbar, Some(base_url.as_str()));
                load_inline_html(window, &final_html)?;
            } else {
                let abs_path = std::fs::canonicalize(&full_path)
                    .context("Failed to canonicalize app entry file path")?;
                let url = Url::from_file_path(&abs_path)
                    .map_err(|_| anyhow::anyhow!("Invalid file path {:?}", abs_path))?;
                window
                    .navigate(url)
                    .context("Failed to navigate to app entry file")?;
            }
        }
        ViewerContent::RemoteUrl { url } => {
            if !request.behaviour.allow_remote_content {
                anyhow::bail!("Remote content is not allowed");
            }

            if let Some(toolbar) = &toolbar_html {
                // For remote URLs with toolbar, use iframe wrapper
                let wrapper = format!(
                    r#"<!DOCTYPE html>
                    <html>
                    <body style="margin:0; padding:0; overflow:hidden;">
                        {}
                        <iframe src="{}" style="width:100%; height:calc(100vh - 30px); border:none;"></iframe>
                    </body>
                    </html>"#,
                    toolbar, url
                );
                load_inline_html(window, &wrapper)?;
            } else {
                // For remote URLs without toolbar, use redirect
                let redirect_html = format!(
                    r#"<!DOCTYPE html>
                    <html>
                    <head>
                        <meta http-equiv="refresh" content="0;url={}">
                    </head>
                    <body>
                        <p>Redirecting to <a href="{}">{}</a>...</p>
                    </body>
                    </html>"#,
                    url, url, url
                );
                load_inline_html(window, &redirect_html)?;
            }
        }
    }

    Ok(())
}

/// Load inline HTML into the window using a data URL.
fn load_inline_html(window: &WebviewWindow, html: &str) -> Result<()> {
    use base64::{Engine as _, engine::general_purpose};

    // Encode HTML as base64 data URL
    let encoded = general_purpose::STANDARD.encode(html.as_bytes());
    let data_url = format!("data:text/html;base64,{}", encoded);

    // Try to parse the data URL. Some embedded WebView implementations may
    // reject extremely long data URLs or have parsing quirks; in that case
    // fall back to writing the HTML using `eval` and `atob` which is more
    // robust for large payloads.
    match Url::parse(&data_url) {
        Ok(url) => {
            window.navigate(url).context("Failed to load HTML")?;
        }
        Err(_) => {
            // Use JavaScript to write the decoded HTML into the document.
            let js = format!(
                "document.open();document.write(atob(\"{}\"));document.close();",
                encoded
            );
            window
                .eval(&js)
                .context("Failed to load HTML via eval fallback")?;
        }
    }

    Ok(())
}

/// Generate HTML for the custom toolbar.
fn generate_toolbar_html(options: &ToolbarOptions) -> String {
    // NOTE: The generated toolbar uses inline `onclick` handlers that call
    // `window.__TAURI__.invoke('toolbar_action', { action: '...' })` to send
    // commands to the Rust backend. Tauri's recommended frontend API is
    // `@tauri-apps/api` (which exposes an `invoke` function), but in many
    // packaging setups `window.__TAURI__` is available as a backwards
    // compatibility shim. If you change your frontend bundling or update
    // Tauri, ensure the `invoke` function is reachable from the global
    // `window` object, or adapt these handlers to use your frontend's
    // API (e.g. `import { invoke } from '@tauri-apps/api'`).

    let title = options.title_text.as_deref().unwrap_or("HTML Viewer");
    let bg_color = options.background_color.as_deref().unwrap_or("#f0f0f0");
    let text_color = options.text_color.as_deref().unwrap_or("#333333");

    format!(
        r#"
        <div data-tauri-drag-region style="
            height: 30px;
            background: {bg_color};
            color: {text_color};
            display: flex;
            align_items: center;
            justify_content: space-between;
            padding: 0 10px;
            font-family: system-ui, sans-serif;
            font-size: 12px;
            user-select: none;
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            z-index: 999999;
            box-shadow: 0 1px 2px rgba(0,0,0,0.1);
        ">
            <div data-tauri-drag-region style="flex: 1; display: flex; align_items: center;">
                <span data-tauri-drag-region style="font-weight: 600;">{title}</span>
            </div>
            <div style="display: flex; gap: 8px;">
                <button onclick="window.__TAURI__.invoke('toolbar_action', {{ action: 'minimize' }})" style="border: none; background: transparent; cursor: pointer; color: inherit; padding: 4px;">&#9472;</button>
                <button onclick="window.__TAURI__.invoke('toolbar_action', {{ action: 'maximize' }})" style="border: none; background: transparent; cursor: pointer; color: inherit; padding: 4px;">&#9633;</button>
                <button onclick="window.__TAURI__.invoke('toolbar_action', {{ action: 'close' }})" style="border: none; background: transparent; cursor: pointer; color: inherit; padding: 4px;">&#10005;</button>
            </div>
        </div>
        <div style="height: 30px;"></div> <!-- Spacer -->
        "#,
        bg_color = bg_color,
        text_color = text_color,
        title = title
    )
}

/// Inject content into HTML string.
fn inject_into_html(html: &str, toolbar: &str, base_url: Option<&str>) -> String {
    let mut result = html.to_string();

    // Inject base URL if provided
    if let Some(base) = base_url {
        let base_tag = format!(r#"<base href="{}">"#, base);
        if let Some(head_end) = result.find("</head>") {
            result.insert_str(head_end, &base_tag);
        } else if let Some(html_start) = result.find("<html>") {
            result.insert_str(html_start + 6, &format!("<head>{}</head>", base_tag));
        } else {
            result = format!("<head>{}</head>{}", base_tag, result);
        }
    }

    // Inject toolbar
    if let Some(body_start) = result.find("<body") {
        if let Some(body_open_end) = result[body_start..].find(">") {
            let insert_pos = body_start + body_open_end + 1;
            result.insert_str(insert_pos, toolbar);
        } else {
            result = format!("{}{}", toolbar, result);
        }
    } else {
        result = format!("{}{}", toolbar, result);
    }

    result
}
