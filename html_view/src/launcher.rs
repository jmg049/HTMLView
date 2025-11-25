use crate::{AppLocator, ViewerError, ViewerHandle, ViewerOptions, ViewerResult, ViewerWaitMode};
use html_view_shared::{ViewerExitReason, ViewerExitStatus, ViewerRequest};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;

/// Launch a viewer with the given options and app locator.
pub(crate) fn launch_viewer(
    options: ViewerOptions,
    locator: &dyn AppLocator,
) -> Result<ViewerResult, ViewerError> {
    // Generate unique ID
    let id = Uuid::new_v4();

    // Create request
    let request = ViewerRequest {
        id,
        content: options.content,
        window: options.window,
        behaviour: options.behaviour,
        environment: options.environment,
        dialog: options.dialog,
    };

    // Create temporary directory for config and result files
    let temp_dir = std::env::temp_dir().join(format!("html_view_{}", id));
    fs::create_dir_all(&temp_dir).map_err(|e| ViewerError::ConfigWriteFailed(e.to_string()))?;

    let config_path = temp_dir.join("config.json");
    let result_path = temp_dir.join("result.json");

    // Write config file
    let config_json = serde_json::to_string_pretty(&request)
        .map_err(|e| ViewerError::SerdeError(e.to_string()))?;
    fs::write(&config_path, config_json)
        .map_err(|e| ViewerError::ConfigWriteFailed(e.to_string()))?;

    // Locate binary
    let app_binary = locator.locate_app_binary()?;

    // Spawn process
    let mut cmd = Command::new(&app_binary);
    cmd.arg("--config-path")
        .arg(&config_path)
        .arg("--result-path")
        .arg(&result_path);

    let mut child = cmd
        .spawn()
        .map_err(|e| ViewerError::SpawnFailed(e.to_string()))?;

    // Handle based on wait mode
    match options.wait {
        ViewerWaitMode::Blocking => {
            // Wait for process to exit
            let exit_status = child.wait()?;

            // Read result file
            let result = read_result_file(&result_path, id)?;

            // Clean up temp directory
            let _ = fs::remove_dir_all(&temp_dir);

            // Check exit code
            if !exit_status.success() {
                if let ViewerExitReason::Error { .. } = result.reason {
                    // Error already captured in result
                } else {
                    return Err(ViewerError::SpawnFailed(format!(
                        "Process exited with code {:?}",
                        exit_status.code()
                    )));
                }
            }

            Ok(ViewerResult::Blocking(result))
        }
        ViewerWaitMode::NonBlocking => {
            let handle = ViewerHandle::new(id, child, result_path);
            Ok(ViewerResult::NonBlocking(handle))
        }
    }
}

/// Read and parse the result file.
fn read_result_file(path: &PathBuf, expected_id: Uuid) -> Result<ViewerExitStatus, ViewerError> {
    // Brief wait for file to be written
    std::thread::sleep(std::time::Duration::from_millis(100));

    let data =
        fs::read_to_string(path).map_err(|e| ViewerError::ResultReadFailed(e.to_string()))?;

    let status: ViewerExitStatus =
        serde_json::from_str(&data).map_err(|e| ViewerError::InvalidResponse(e.to_string()))?;

    // Verify ID matches
    if status.id != expected_id {
        return Err(ViewerError::InvalidResponse(format!(
            "Result ID mismatch: expected {}, got {}",
            expected_id, status.id
        )));
    }

    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ViewerError;
    use std::path::PathBuf;

    struct MockAppLocator {
        path: Option<PathBuf>,
    }

    impl AppLocator for MockAppLocator {
        fn locate_app_binary(&self) -> Result<PathBuf, ViewerError> {
            self.path
                .clone()
                .ok_or_else(|| ViewerError::BinaryNotFound("Mock binary not found".to_string()))
        }
    }

    #[test]
    fn test_launcher_binary_not_found() {
        let options = ViewerOptions::inline_html("<h1>Test</h1>");
        let locator = MockAppLocator { path: None };

        let result = launch_viewer(options, &locator);
        assert!(matches!(result, Err(ViewerError::BinaryNotFound(_))));
    }
}
