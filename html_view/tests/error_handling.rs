//! Comprehensive error handling tests

use html_view::ViewerError;
use std::io;

#[test]
fn test_error_display_binary_not_found() {
    let err = ViewerError::BinaryNotFound("detailed error message".to_string());
    let display = format!("{}", err);
    assert!(display.contains("html_view_app binary not found"));
    assert!(display.contains("detailed error message"));
}

#[test]
fn test_error_display_spawn_failed() {
    let err = ViewerError::SpawnFailed("permission denied".to_string());
    let display = format!("{}", err);
    assert!(display.contains("failed to spawn"));
    assert!(display.contains("permission denied"));
}

#[test]
fn test_error_display_config_write_failed() {
    let err = ViewerError::ConfigWriteFailed("/tmp/invalid/path".to_string());
    let display = format!("{}", err);
    assert!(display.contains("failed to write configuration"));
    assert!(display.contains("/tmp/invalid/path"));
}

#[test]
fn test_error_display_result_read_failed() {
    let err = ViewerError::ResultReadFailed("file not found".to_string());
    let display = format!("{}", err);
    assert!(display.contains("failed to read result file"));
    assert!(display.contains("file not found"));
}

#[test]
fn test_error_display_invalid_response() {
    let err = ViewerError::InvalidResponse("malformed JSON".to_string());
    let display = format!("{}", err);
    assert!(display.contains("invalid response"));
    assert!(display.contains("malformed JSON"));
}

#[test]
fn test_error_display_timeout() {
    let err = ViewerError::Timeout;
    let display = format!("{}", err);
    assert_eq!(display, "viewer timed out");
}

#[test]
fn test_error_display_version_mismatch() {
    let err = ViewerError::VersionMismatch {
        library: "0.2.0".to_string(),
        viewer: "0.1.0".to_string(),
        suggestion: "Please rebuild".to_string(),
    };
    let display = format!("{}", err);
    assert!(display.contains("version mismatch"));
    assert!(display.contains("library v0.2.0"));
    assert!(display.contains("viewer v0.1.0"));
    assert!(display.contains("Please rebuild"));
}

#[test]
fn test_error_from_io_error() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err: ViewerError = io_err.into();

    match err {
        ViewerError::IoError(e) => {
            assert_eq!(e.kind(), io::ErrorKind::NotFound);
            assert!(e.to_string().contains("file not found"));
        }
        _ => panic!("Expected IoError variant"),
    }
}

#[test]
fn test_error_clone_binary_not_found() {
    let err = ViewerError::BinaryNotFound("test".to_string());
    let cloned = err.clone();

    match (&err, &cloned) {
        (ViewerError::BinaryNotFound(a), ViewerError::BinaryNotFound(b)) => {
            assert_eq!(a, b);
        }
        _ => panic!("Clone failed to preserve variant"),
    }
}

#[test]
fn test_error_clone_timeout() {
    let err = ViewerError::Timeout;
    let cloned = err.clone();

    assert!(matches!(cloned, ViewerError::Timeout));
}

#[test]
fn test_error_clone_version_mismatch() {
    let err = ViewerError::VersionMismatch {
        library: "0.2.0".to_string(),
        viewer: "0.1.0".to_string(),
        suggestion: "Rebuild".to_string(),
    };
    let cloned = err.clone();

    match (&err, &cloned) {
        (
            ViewerError::VersionMismatch {
                library: l1,
                viewer: v1,
                suggestion: s1,
            },
            ViewerError::VersionMismatch {
                library: l2,
                viewer: v2,
                suggestion: s2,
            },
        ) => {
            assert_eq!(l1, l2);
            assert_eq!(v1, v2);
            assert_eq!(s1, s2);
        }
        _ => panic!("Clone failed to preserve variant"),
    }
}

#[test]
fn test_error_clone_io_error() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let err: ViewerError = io_err.into();
    let cloned = err.clone();

    match (&err, &cloned) {
        (ViewerError::IoError(e1), ViewerError::IoError(e2)) => {
            assert_eq!(e1.kind(), e2.kind());
            assert_eq!(e1.to_string(), e2.to_string());
        }
        _ => panic!("Clone failed to preserve variant"),
    }
}

#[test]
fn test_error_debug_format() {
    let err = ViewerError::BinaryNotFound("test error".to_string());
    let debug = format!("{:?}", err);
    assert!(debug.contains("BinaryNotFound"));
    assert!(debug.contains("test error"));
}

#[test]
fn test_error_is_send_sync() {
    // Compile-time check that ViewerError implements Send + Sync
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ViewerError>();
}
