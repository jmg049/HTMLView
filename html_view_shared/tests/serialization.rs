use html_view_shared::*;
use std::path::PathBuf;
use url::Url;
use uuid::Uuid;

#[test]
fn test_viewer_request_roundtrip() {
    let request = ViewerRequest {
        id: Uuid::new_v4(),
        content: ViewerContent::InlineHtml {
            html: "<h1>Test</h1>".to_string(),
            base_dir: None,
        },
        window: WindowOptions::default(),
        behaviour: BehaviourOptions::default(),
        environment: EnvironmentOptions::default(),
        dialog: DialogOptions::default(),
    };

    let json = serde_json::to_string(&request).unwrap();
    let deserialized: ViewerRequest = serde_json::from_str(&json).unwrap();

    assert_eq!(request.id, deserialized.id);
}

#[test]
fn test_viewer_content_inline_html() {
    let content = ViewerContent::InlineHtml {
        html: "<p>Hello</p>".to_string(),
        base_dir: Some(PathBuf::from("/tmp")),
    };

    let json = serde_json::to_string(&content).unwrap();
    let deserialized: ViewerContent = serde_json::from_str(&json).unwrap();

    match deserialized {
        ViewerContent::InlineHtml { html, base_dir } => {
            assert_eq!(html, "<p>Hello</p>");
            assert_eq!(base_dir, Some(PathBuf::from("/tmp")));
        }
        _ => panic!("Expected InlineHtml"),
    }
}

#[test]
fn test_viewer_content_local_file() {
    let content = ViewerContent::LocalFile {
        path: PathBuf::from("/path/to/file.html"),
    };

    let json = serde_json::to_string(&content).unwrap();
    let deserialized: ViewerContent = serde_json::from_str(&json).unwrap();

    match deserialized {
        ViewerContent::LocalFile { path } => {
            assert_eq!(path, PathBuf::from("/path/to/file.html"));
        }
        _ => panic!("Expected LocalFile"),
    }
}

#[test]
fn test_viewer_content_app_dir() {
    let content = ViewerContent::AppDir {
        root: PathBuf::from("/app/root"),
        entry: Some("main.html".to_string()),
    };

    let json = serde_json::to_string(&content).unwrap();
    let deserialized: ViewerContent = serde_json::from_str(&json).unwrap();

    match deserialized {
        ViewerContent::AppDir { root, entry } => {
            assert_eq!(root, PathBuf::from("/app/root"));
            assert_eq!(entry, Some("main.html".to_string()));
        }
        _ => panic!("Expected AppDir"),
    }
}

#[test]
fn test_viewer_content_remote_url() {
    let content = ViewerContent::RemoteUrl {
        url: Url::parse("https://example.com").unwrap(),
    };

    let json = serde_json::to_string(&content).unwrap();
    let deserialized: ViewerContent = serde_json::from_str(&json).unwrap();

    match deserialized {
        ViewerContent::RemoteUrl { url } => {
            assert_eq!(url.as_str(), "https://example.com/");
        }
        _ => panic!("Expected RemoteUrl"),
    }
}

#[test]
fn test_viewer_exit_status_roundtrip() {
    let status = ViewerExitStatus {
        id: Uuid::new_v4(),
        reason: ViewerExitReason::ClosedByUser,
    };

    let json = serde_json::to_string(&status).unwrap();
    let deserialized: ViewerExitStatus = serde_json::from_str(&json).unwrap();

    assert_eq!(status.id, deserialized.id);
    matches!(deserialized.reason, ViewerExitReason::ClosedByUser);
}

#[test]
fn test_viewer_exit_reason_timed_out() {
    let reason = ViewerExitReason::TimedOut;
    let json = serde_json::to_string(&reason).unwrap();
    let deserialized: ViewerExitReason = serde_json::from_str(&json).unwrap();
    matches!(deserialized, ViewerExitReason::TimedOut);
}

#[test]
fn test_viewer_exit_reason_error() {
    let reason = ViewerExitReason::Error {
        message: "Test error".to_string(),
    };
    let json = serde_json::to_string(&reason).unwrap();
    let deserialized: ViewerExitReason = serde_json::from_str(&json).unwrap();

    match deserialized {
        ViewerExitReason::Error { message } => {
            assert_eq!(message, "Test error");
        }
        _ => panic!("Expected Error"),
    }
}

#[test]
fn test_window_options_defaults() {
    let opts = WindowOptions::default();
    assert_eq!(opts.title, Some("HTML Viewer".to_string()));
    assert_eq!(opts.width, Some(1024));
    assert_eq!(opts.height, Some(768));
    assert!(opts.resizable);
    assert!(!opts.maximised);
    assert!(!opts.fullscreen);
}

#[test]
fn test_behaviour_options_defaults() {
    let opts = BehaviourOptions::default();
    assert!(!opts.allow_external_navigation);
    assert!(!opts.enable_devtools);
    assert!(!opts.allow_remote_content);
    assert_eq!(opts.allowed_domains, None);
}
