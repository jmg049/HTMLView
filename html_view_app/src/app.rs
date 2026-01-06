use anyhow::{Context, Result};
use html_view_shared::{
    PROTOCOL_VERSION, ViewerCommand, ViewerCommandResponse, ViewerContent, ViewerExitReason,
    ViewerExitStatus, ViewerRequest, WindowOptions,
};
use notify::{Event, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, LogicalPosition, LogicalSize, Manager, Position, State, WebviewWindow};
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

            configure_window(&window, &request_arc.window)?;

            crate::content_loader::load_content(&window, &request_arc)?;

            // Spawn command watcher if command_path is provided
            if let Some(command_path) = request_arc.command_path.clone() {
                let window_for_commands = window.clone();
                let request_for_commands = request_arc.clone();

                std::thread::spawn(move || {
                    if let Err(e) =
                        watch_commands(command_path, window_for_commands, request_for_commands)
                    {
                        eprintln!("Command watcher error: {}", e);
                    }
                });
            }

            // Set up timeout if configured
            if let Some(timeout_secs) = request_arc.environment.timeout_seconds {
                let window_for_timeout = window.clone();
                let exit_reason = exit_reason_for_timeout.clone();

                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_secs(timeout_secs));

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
        viewer_version: PROTOCOL_VERSION.to_string(),
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
    app: AppHandle,
    state: State<'_, Arc<ViewerRequest>>,
    title: String,
    body: String,
) {
    if state.behaviour.allow_notifications {
        let _ = app.notification().builder().title(title).body(body).show();
    }
}

#[tauri::command]
fn show_message_dialog(
    app: AppHandle,
    state: State<'_, Arc<ViewerRequest>>,
    title: String,
    message: String,
) {
    if state.dialog.allow_message_dialogs {
        app.dialog().message(message).title(title).show(|_| {});
    }
}

#[tauri::command]
async fn show_open_dialog(
    app: AppHandle,
    state: State<'_, Arc<ViewerRequest>>,
) -> Result<Option<String>, String> {
    if !state.dialog.allow_file_dialogs {
        return Err("File dialogs not allowed".to_string());
    }

    // Use blocking API which is simpler for this use case
    let file_path = app.dialog().file().blocking_pick_file();

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
        window.set_size(Size::Logical(LogicalSize {
            width: width as f64,
            height: height as f64,
        }))?;
    }

    // Set position
    if let (Some(x), Some(y)) = (options.x, options.y) {
        window.set_position(Position::Logical(LogicalPosition {
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

    Ok(())
}

/// Watch the command file for changes and process commands.
fn watch_commands(
    command_path: PathBuf,
    window: WebviewWindow,
    request: Arc<ViewerRequest>,
) -> Result<()> {
    let (tx, rx) = channel();

    // Create file watcher - must keep it alive for the duration
    let mut _watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Err(e) = tx.send(res) {
            eprintln!("Failed to send watch event: {}", e);
        }
    })
    .context("Failed to create filesystem watcher")?;

    // Watch the parent directory (since the file might not exist yet)
    let watch_dir = command_path
        .parent()
        .context("Failed to get parent directory")?;

    _watcher
        .watch(watch_dir, RecursiveMode::NonRecursive)
        .context("Failed to watch directory")?;

    let command_filename = command_path
        .file_name()
        .context("Failed to get command filename")?
        .to_owned();

    // Process events
    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                // Check if this event is for our command file
                let is_command_file = event
                    .paths
                    .iter()
                    .any(|p| p.file_name() == Some(command_filename.as_ref()));

                if is_command_file {
                    // Process any event type for the command file
                    if let Err(e) = process_command_file(&command_path, &window, &request) {
                        eprintln!("Failed to process command: {}", e);
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!("Watch error: {}", e);
            }
            Err(e) => {
                eprintln!("Channel error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

/// Process a command from the command file.
fn process_command_file(
    path: &PathBuf,
    window: &WebviewWindow,
    request: &ViewerRequest,
) -> Result<()> {
    // Read command file
    let data = std::fs::read_to_string(path).context("Failed to read command file")?;

    let command: ViewerCommand = serde_json::from_str(&data).context("Failed to parse command")?;

    // Execute command
    let response = match &command {
        ViewerCommand::Refresh { seq, content } => {
            match execute_refresh(window, content, request) {
                Ok(()) => ViewerCommandResponse {
                    seq: *seq,
                    success: true,
                    error: None,
                },
                Err(e) => ViewerCommandResponse {
                    seq: *seq,
                    success: false,
                    error: Some(e.to_string()),
                },
            }
        }
    };

    // Write response
    let response_path = path
        .parent()
        .context("Failed to get parent directory")?
        .join("command_responses.json");

    let response_json = serde_json::to_string(&response).context("Failed to serialize response")?;

    std::fs::write(&response_path, &response_json).context("Failed to write response file")?;

    Ok(())
}

/// Execute a refresh command.
fn execute_refresh(
    window: &WebviewWindow,
    content: &ViewerContent,
    request: &ViewerRequest,
) -> Result<()> {
    // Create temporary request with new content
    let refresh_request = ViewerRequest {
        id: request.id,
        content: content.clone(),
        window: request.window.clone(),
        behaviour: request.behaviour.clone(),
        environment: request.environment.clone(),
        dialog: request.dialog.clone(),
        command_path: request.command_path.clone(),
    };

    // Use existing content loader
    crate::content_loader::load_content(window, &refresh_request)
}
