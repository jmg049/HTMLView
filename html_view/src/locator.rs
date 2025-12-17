use crate::ViewerError;
use std::path::PathBuf;

/// Trait for locating the html_view_app binary.
///
/// This abstraction allows for testing without requiring the actual binary.
pub trait AppLocator {
    /// Locate the html_view_app binary.
    fn locate_app_binary(&self) -> Result<PathBuf, ViewerError>;
}

/// Default implementation of AppLocator.
///
/// Searches for the binary in the following order:
/// 1. Compile-time embedded path (set by build.rs, if used)
/// 2. Runtime environment variable `HTML_VIEW_APP_PATH`
/// 3. Cargo install directory (~/.cargo/bin)
/// 4. Same directory as the current executable
/// 5. Target directory relative to workspace (for development)
pub struct DefaultAppLocator;

impl AppLocator for DefaultAppLocator {
    fn locate_app_binary(&self) -> Result<PathBuf, ViewerError> {
        // 1. Check compile-time embedded path (set by build.rs)
        if let Some(embedded_path) = option_env!("HTML_VIEW_APP_PATH") {
            let path = PathBuf::from(embedded_path);
            if path.exists() && path.is_file() {
                return Ok(path);
            }
        }

        // 2. Check runtime environment variable (allows override)
        if let Ok(path) = std::env::var("HTML_VIEW_APP_PATH") {
            let path = PathBuf::from(path);
            if path.exists() && path.is_file() {
                return Ok(path);
            }
        }

        // 3. Check in Cargo install directory (~/.cargo/bin)
        if let Some(home) = std::env::var_os("CARGO_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME")
                    .or_else(|| std::env::var_os("USERPROFILE"))
                    .map(|h| PathBuf::from(h).join(".cargo"))
            })
        {
            let binary_name = if cfg!(target_os = "windows") {
                "html_view_app.exe"
            } else {
                "html_view_app"
            };

            let candidate = home.join("bin").join(binary_name);
            if candidate.exists() && candidate.is_file() {
                return Ok(candidate);
            }
        }

        // 4. Check in the same directory as the current executable
        if let Ok(exe_path) = std::env::current_exe()
            && let Some(exe_dir) = exe_path.parent() {
                let binary_name = if cfg!(target_os = "windows") {
                    "html_view_app.exe"
                } else {
                    "html_view_app"
                };

                let candidate = exe_dir.join(binary_name);
                if candidate.exists() && candidate.is_file() {
                    return Ok(candidate);
                }
            }

        // 5. Check in target directory (for development/testing)
        if let Ok(current_dir) = std::env::current_dir() {
            for profile in &["debug", "release"] {
                let binary_name = if cfg!(target_os = "windows") {
                    "html_view_app.exe"
                } else {
                    "html_view_app"
                };

                let candidate = current_dir.join("target").join(profile).join(binary_name);

                if candidate.exists() && candidate.is_file() {
                    return Ok(candidate);
                }
            }
        }

        Err(ViewerError::BinaryNotFound(
            "Could not locate html_view_app binary. \
             Please install it first with: cargo install --path html_view_app\n\
             Or set HTML_VIEW_APP_PATH environment variable to specify the location."
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_var_locator() {
        // This test would need to set up a test environment
        // For now, just verify the trait is implemented
        let locator = DefaultAppLocator;
        let _result = locator.locate_app_binary();
        // We expect this to fail in test environment, which is fine
    }
}
