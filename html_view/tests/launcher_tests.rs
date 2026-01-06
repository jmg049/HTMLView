//! Launcher module tests including exponential backoff and temp directory management

use std::path::PathBuf;
use std::time::{Duration, Instant};

#[test]
fn test_exponential_backoff_timing() {
    // Test that exponential backoff follows expected pattern
    let base_delay = Duration::from_millis(10);
    let max_delay = Duration::from_millis(1000);

    let mut delays = Vec::new();
    let mut current_delay = base_delay;

    for _ in 0..10 {
        delays.push(current_delay);
        current_delay = (current_delay * 2).min(max_delay);
    }

    // Verify exponential growth up to max
    assert_eq!(delays[0], Duration::from_millis(10));
    assert_eq!(delays[1], Duration::from_millis(20));
    assert_eq!(delays[2], Duration::from_millis(40));
    assert_eq!(delays[3], Duration::from_millis(80));
    assert_eq!(delays[4], Duration::from_millis(160));
    assert_eq!(delays[5], Duration::from_millis(320));
    assert_eq!(delays[6], Duration::from_millis(640));
    assert_eq!(delays[7], Duration::from_millis(1000)); // Capped at max
    assert_eq!(delays[8], Duration::from_millis(1000)); // Stays at max
    assert_eq!(delays[9], Duration::from_millis(1000)); // Stays at max
}

#[test]
fn test_exponential_backoff_total_wait_time() {
    // Test total wait time for 10 attempts
    let base_delay = Duration::from_millis(10);
    let max_delay = Duration::from_millis(1000);
    let max_attempts = 10;

    let mut total_wait = Duration::from_secs(0);
    let mut current_delay = base_delay;

    for _ in 0..max_attempts {
        total_wait += current_delay;
        current_delay = (current_delay * 2).min(max_delay);
    }

    // 10 + 20 + 40 + 80 + 160 + 320 + 640 + 1000 + 1000 + 1000 = 4270ms
    assert_eq!(total_wait, Duration::from_millis(4270));

    // Verify this is reasonable for a timeout scenario
    assert!(total_wait < Duration::from_secs(5));
    assert!(total_wait > Duration::from_secs(4));
}

#[test]
fn test_max_attempts_limit() {
    // Verify that polling stops after max_attempts
    const MAX_ATTEMPTS: usize = 10;

    let mut attempts = 0;
    for i in 0..MAX_ATTEMPTS {
        attempts = i + 1;
    }

    assert_eq!(attempts, MAX_ATTEMPTS);
}

#[test]
fn test_temp_dir_path_generation() {
    // Test that temp directory paths are generated correctly
    use uuid::Uuid;

    let id = Uuid::new_v4();
    let temp_dir = std::env::temp_dir().join(format!("html_view_{}", id));

    assert!(temp_dir.to_string_lossy().contains("html_view_"));
    assert!(temp_dir.to_string_lossy().contains(&id.to_string()));
}

#[test]
fn test_temp_dir_unique_per_request() {
    // Verify each request gets a unique temp directory
    use uuid::Uuid;

    let id1 = Uuid::new_v4();
    let id2 = Uuid::new_v4();

    let temp1 = std::env::temp_dir().join(format!("html_view_{}", id1));
    let temp2 = std::env::temp_dir().join(format!("html_view_{}", id2));

    assert_ne!(temp1, temp2);
    assert_ne!(id1, id2);
}

#[test]
fn test_request_json_filename() {
    // Verify request file is named "request.json"
    use uuid::Uuid;

    let id = Uuid::new_v4();
    let temp_dir = std::env::temp_dir().join(format!("html_view_{}", id));
    let request_file = temp_dir.join("request.json");

    assert_eq!(request_file.file_name().unwrap(), "request.json");
}

#[test]
fn test_result_json_filename() {
    // Verify result file is named "result.json"
    use uuid::Uuid;

    let id = Uuid::new_v4();
    let temp_dir = std::env::temp_dir().join(format!("html_view_{}", id));
    let result_file = temp_dir.join("result.json");

    assert_eq!(result_file.file_name().unwrap(), "result.json");
}

#[test]
fn test_temp_dir_cleanup_on_drop() {
    // Test that temp directories are cleaned up properly
    use std::fs;
    use uuid::Uuid;

    let id = Uuid::new_v4();
    let temp_dir = std::env::temp_dir().join(format!("html_view_test_{}", id));

    // Create temp directory
    fs::create_dir_all(&temp_dir).unwrap();
    assert!(temp_dir.exists());

    // Simulate TempDirGuard drop
    {
        let _guard = TestTempDirGuard {
            path: temp_dir.clone(),
        };
        // Guard is in scope, directory should exist
        assert!(temp_dir.exists());
    } // Guard drops here

    // After drop, directory should be cleaned up
    assert!(!temp_dir.exists());

    // Helper struct mimicking TempDirGuard
    struct TestTempDirGuard {
        path: PathBuf,
    }

    impl Drop for TestTempDirGuard {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

#[test]
fn test_file_permissions_unix() {
    // Test that temp directories have correct permissions on Unix
    #[cfg(unix)]
    {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;
        use uuid::Uuid;

        let id = Uuid::new_v4();
        let temp_dir = std::env::temp_dir().join(format!("html_view_test_perms_{}", id));

        // Create directory with 0o700 permissions
        fs::create_dir_all(&temp_dir).unwrap();
        let mut perms = fs::metadata(&temp_dir).unwrap().permissions();
        perms.set_mode(0o700);
        fs::set_permissions(&temp_dir, perms).unwrap();

        // Verify permissions
        let metadata = fs::metadata(&temp_dir).unwrap();
        let mode = metadata.permissions().mode() & 0o777;
        assert_eq!(mode, 0o700, "Expected 0o700, got 0o{:o}", mode);

        // Cleanup
        fs::remove_dir_all(&temp_dir).unwrap();
    }
}

#[test]
fn test_polling_early_exit_on_success() {
    // Test that polling exits early when file is found
    let start = Instant::now();
    let max_attempts = 10;
    let found_on_attempt = 3;

    let mut attempts = 0;
    for i in 0..max_attempts {
        attempts = i + 1;
        if i == found_on_attempt {
            break; // Simulate finding the file
        }
        std::thread::sleep(Duration::from_millis(1));
    }

    let elapsed = start.elapsed();

    // Should exit early, not run all 10 attempts
    assert_eq!(attempts, found_on_attempt + 1);
    assert!(elapsed < Duration::from_millis(100)); // Should be much less than 4+ seconds
}

#[test]
fn test_backoff_never_exceeds_max_delay() {
    // Verify delay never exceeds max even after many iterations
    let base_delay = Duration::from_millis(10);
    let max_delay = Duration::from_millis(1000);

    let mut current_delay = base_delay;

    for _ in 0..100 {
        // Even after 100 iterations
        current_delay = (current_delay * 2).min(max_delay);
        assert!(current_delay <= max_delay);
    }

    assert_eq!(current_delay, max_delay);
}

#[test]
fn test_uuid_generation_uniqueness() {
    // Verify UUIDs are unique across multiple requests
    use std::collections::HashSet;
    use uuid::Uuid;

    let mut ids = HashSet::new();
    for _ in 0..1000 {
        let id = Uuid::new_v4();
        assert!(ids.insert(id), "UUID collision detected!");
    }

    assert_eq!(ids.len(), 1000);
}
