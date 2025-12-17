use anyhow::{Context, Result};
use html_view_shared::{ViewerExitReason, ViewerExitStatus, ViewerRequest, WindowOptions};
use std::sync::{Arc, Mutex};
use tauri::{Manager, WebviewWindow};

use tauri_plugin_dialog::DialogExt;
use tauri_plugin_notification::NotificationExt;

/// Run the Tauri application with the given request.
pub fn run_app(request: ViewerRequest) -> Result<ViewerExitStatus> {
    // Store the request and exit reason in shared state
    let request_arc = Arc::new(request.clone());
    let exit_reason = Arc::new(Mutex::new(ViewerExitReason::ClosedByUser));

    // Clone for use in closures
    let exit_reason_for_timeout = exit_reason.clone();
    let _request_for_timeout = request_arc.clone();

    tauri::Builder::default()
        // .plugin(tauri_plugin_dialog::init())
        // .plugin(tauri_plugin_notification::init())
        // .plugin(tauri_plugin_cli::init())
        .invoke_handler(tauri::generate_handler![
            toolbar_action,
            show_notification,
            show_message_dialog,
            show_open_dialog
        ])
        .setup(move |app| {
            app.manage(request_arc.clone());

            let window = app
                .get_webview_window("main")
                .context("Main window not found")?;

            // Configure window
            configure_window(&window, &request_arc.window)?;

            // Load content
            crate::content_loader::load_content(&window, &request_arc)?;

            // Set up timeout if configured
            if let Some(timeout_secs) = request_arc.environment.timeout_seconds {
                let window_for_timeout = window.clone();
                let exit_reason = exit_reason_for_timeout.clone();

                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(timeout_secs));

                    // Update exit reason
                    if let Ok(mut reason) = exit_reason.lock() {
                        *reason = ViewerExitReason::TimedOut;
                    }

                    // Close window
                    let _ = window_for_timeout.close();
                });
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .context("Failed to build Tauri application")?
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {
                // App is exiting
            }
        });

    // Return the exit status
    let reason = exit_reason.lock().unwrap().clone();
    Ok(ViewerExitStatus {
        id: request.id,
        reason,
    })
}

#[tauri::command]
fn toolbar_action(action: String, window: tauri::Window) {
    match action.as_str() {
        "minimize" => {
            let _ = window.minimize();
        }
        "maximize" => {
            if let Ok(is_maximized) = window.is_maximized() {
                if is_maximized {
                    let _ = window.unmaximize();
                } else {
                    let _ = window.maximize();
                }
            }
        }
        "close" => {
            let _ = window.close();
        }
        _ => {}
    }
}

#[tauri::command]
fn show_notification(
    app: tauri::AppHandle,
    state: tauri::State<'_, Arc<ViewerRequest>>,
    title: String,
    body: String,
) {
    if state.behaviour.allow_notifications {
        let _ = app.notification().builder().title(title).body(body).show();
    }
}

#[tauri::command]
fn show_message_dialog(
    app: tauri::AppHandle,
    state: tauri::State<'_, Arc<ViewerRequest>>,
    title: String,
    message: String,
) {
    if state.dialog.allow_message_dialogs {
        app.dialog()
            .message(message)
            .title(title)
            .show(|_| {});
    }
}

#[tauri::command]
async fn show_open_dialog(
    app: tauri::AppHandle,
    state: tauri::State<'_, Arc<ViewerRequest>>,
) -> Result<Option<String>, String> {
    if !state.dialog.allow_file_dialogs {
        return Err("File dialogs not allowed".to_string());
    }

    // Use blocking API which is simpler for this use case
    let file_path = app.dialog()
        .file()
        .blocking_pick_file();
    
    Ok(file_path.map(|fp| fp.to_string()))
}


/// Configure the window based on WindowOptions.
fn configure_window(window: &WebviewWindow, options: &WindowOptions) -> Result<()> {
    // Set title
    if let Some(ref title) = options.title {
        window.set_title(title)?;
    }

    // Set size
    if let (Some(width), Some(height)) = (options.width, options.height) {
        use tauri::Size;
        window.set_size(Size::Logical(tauri::LogicalSize {
            width: width as f64,
            height: height as f64,
        }))?;
    }

    // Set position
    if let (Some(x), Some(y)) = (options.x, options.y) {
        use tauri::Position;
        window.set_position(Position::Logical(tauri::LogicalPosition {
            x: x as f64,
            y: y as f64,
        }))?;
    }

    // Set resizable
    window.set_resizable(options.resizable)?;

    // Set maximised
    if options.maximised {
        window.maximize()?;
    }

    // Set fullscreen
    if options.fullscreen {
        window.set_fullscreen(true)?;
    }

    // Set decorations
    window.set_decorations(options.decorations)?;

    // Set always on top
    window.set_always_on_top(options.always_on_top)?;

    // Set background color if provided
    // Note: Transparency requires window creation flag in Tauri v1, 
    // but in v2 we can set it here if the window was created with transparency support.
    // For now we'll rely on the main window creation config in tauri.conf.json being permissive.
    
    // Note: Theme handling is platform specific and often requires restart or initial config,
    // skipping dynamic theme update for now as it's complex in Tauri 2.0 without plugins.

    Ok(())
}
