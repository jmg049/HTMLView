//! Performance benchmarks for html_view
//!
//! Run with: cargo bench

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use html_view_shared::*;
use std::path::PathBuf;
use url::Url;
use uuid::Uuid;

fn benchmark_serialization(c: &mut Criterion) {
    c.bench_function("serialize ViewerRequest", |b| {
        let request = ViewerRequest {
            id: Uuid::new_v4(),
            content: ViewerContent::InlineHtml {
                html: "<h1>Benchmark Test</h1>".to_string(),
                base_dir: None,
            },
            window: WindowOptions::default(),
            behaviour: BehaviourOptions::default(),
            environment: EnvironmentOptions::default(),
            dialog: DialogOptions::default(),
        };

        b.iter(|| {
            let json = serde_json::to_string(&request).unwrap();
            black_box(json);
        });
    });

    c.bench_function("deserialize ViewerRequest", |b| {
        let request = ViewerRequest {
            id: Uuid::new_v4(),
            content: ViewerContent::InlineHtml {
                html: "<h1>Benchmark Test</h1>".to_string(),
                base_dir: None,
            },
            window: WindowOptions::default(),
            behaviour: BehaviourOptions::default(),
            environment: EnvironmentOptions::default(),
            dialog: DialogOptions::default(),
        };
        let json = serde_json::to_string(&request).unwrap();

        b.iter(|| {
            let deserialized: ViewerRequest = serde_json::from_str(&json).unwrap();
            black_box(deserialized);
        });
    });
}

fn benchmark_viewer_content_types(c: &mut Criterion) {
    c.bench_function("serialize InlineHtml", |b| {
        let content = ViewerContent::InlineHtml {
            html: "<html><body><h1>Large HTML Content</h1>".repeat(100) + "</body></html>",
            base_dir: Some(PathBuf::from("/tmp")),
        };

        b.iter(|| {
            let json = serde_json::to_string(&content).unwrap();
            black_box(json);
        });
    });

    c.bench_function("serialize LocalFile", |b| {
        let content = ViewerContent::LocalFile {
            path: PathBuf::from("/very/long/path/to/file.html"),
        };

        b.iter(|| {
            let json = serde_json::to_string(&content).unwrap();
            black_box(json);
        });
    });

    c.bench_function("serialize AppDir", |b| {
        let content = ViewerContent::AppDir {
            root: PathBuf::from("/app/root/directory"),
            entry: Some("index.html".to_string()),
        };

        b.iter(|| {
            let json = serde_json::to_string(&content).unwrap();
            black_box(json);
        });
    });

    c.bench_function("serialize RemoteUrl", |b| {
        let content = ViewerContent::RemoteUrl {
            url: Url::parse("https://example.com/page.html").unwrap(),
        };

        b.iter(|| {
            let json = serde_json::to_string(&content).unwrap();
            black_box(json);
        });
    });
}

fn benchmark_exit_status(c: &mut Criterion) {
    c.bench_function("serialize ViewerExitStatus", |b| {
        let status = ViewerExitStatus {
            id: Uuid::new_v4(),
            reason: ViewerExitReason::ClosedByUser,
            viewer_version: "0.1.0".to_string(),
        };

        b.iter(|| {
            let json = serde_json::to_string(&status).unwrap();
            black_box(json);
        });
    });

    c.bench_function("deserialize ViewerExitStatus", |b| {
        let status = ViewerExitStatus {
            id: Uuid::new_v4(),
            reason: ViewerExitReason::ClosedByUser,
            viewer_version: "0.1.0".to_string(),
        };
        let json = serde_json::to_string(&status).unwrap();

        b.iter(|| {
            let deserialized: ViewerExitStatus = serde_json::from_str(&json).unwrap();
            black_box(deserialized);
        });
    });
}

fn benchmark_uuid_generation(c: &mut Criterion) {
    c.bench_function("generate UUID v4", |b| {
        b.iter(|| {
            let id = Uuid::new_v4();
            black_box(id);
        });
    });
}

fn benchmark_temp_path_construction(c: &mut Criterion) {
    c.bench_function("construct temp directory path", |b| {
        b.iter(|| {
            let id = Uuid::new_v4();
            let temp_dir = std::env::temp_dir().join(format!("html_view_{}", id));
            black_box(temp_dir);
        });
    });
}

fn benchmark_options_construction(c: &mut Criterion) {
    use html_view::ViewerOptions;

    c.bench_function("construct ViewerOptions (inline_html)", |b| {
        b.iter(|| {
            let options = ViewerOptions::inline_html("<h1>Test</h1>");
            black_box(options);
        });
    });

    c.bench_function("construct ViewerOptions (local_file)", |b| {
        b.iter(|| {
            let options = ViewerOptions::local_file(PathBuf::from("/tmp/test.html"));
            black_box(options);
        });
    });

    c.bench_function("construct ViewerOptions (app_dir)", |b| {
        b.iter(|| {
            let options = ViewerOptions::app_dir(PathBuf::from("/tmp/app"));
            black_box(options);
        });
    });

    c.bench_function("construct ViewerOptions (remote_url)", |b| {
        let url = Url::parse("https://example.com").unwrap();
        b.iter(|| {
            let options = ViewerOptions::remote_url(url.clone());
            black_box(options);
        });
    });
}

fn benchmark_window_options(c: &mut Criterion) {
    c.bench_function("clone WindowOptions", |b| {
        let options = WindowOptions::default();
        b.iter(|| {
            let cloned = options.clone();
            black_box(cloned);
        });
    });

    c.bench_function("serialize WindowOptions", |b| {
        let options = WindowOptions::default();
        b.iter(|| {
            let json = serde_json::to_string(&options).unwrap();
            black_box(json);
        });
    });
}

criterion_group!(
    benches,
    benchmark_serialization,
    benchmark_viewer_content_types,
    benchmark_exit_status,
    benchmark_uuid_generation,
    benchmark_temp_path_construction,
    benchmark_options_construction,
    benchmark_window_options,
);
criterion_main!(benches);
