//! CLI tool for html_view - display HTML from the command line.

use clap::{Parser, Subcommand};
use html_view::{
    BehaviourOptions, EnvironmentOptions, ViewerContent, ViewerOptions, WindowOptions,
};
use std::path::PathBuf;
use url::Url;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Window width
    #[arg(long, global = true)]
    width: Option<u32>,

    /// Window height
    #[arg(long, global = true)]
    height: Option<u32>,

    /// Window title
    #[arg(long, global = true)]
    title: Option<String>,

    /// Enable devtools
    #[arg(long, global = true)]
    devtools: bool,

    /// Timeout in seconds
    #[arg(long, global = true)]
    timeout: Option<u64>,

    /// Hide window decorations
    #[arg(long, global = true)]
    no_decorations: bool,

    /// Make window transparent
    #[arg(long, global = true)]
    transparent: bool,

    /// Keep window always on top
    #[arg(long, global = true)]
    always_on_top: bool,

    /// Show custom toolbar
    #[arg(long, global = true)]
    show_toolbar: bool,

    /// Toolbar title text
    #[arg(long, global = true)]
    toolbar_title: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Display inline HTML
    Html {
        /// HTML string to display
        html: String,
    },
    /// Display a local HTML file
    File {
        /// Path to HTML file
        path: PathBuf,
    },
    /// Display an HTML application directory
    Dir {
        /// Root directory
        root: PathBuf,
        /// Entry file (default: index.html)
        #[arg(long)]
        entry: Option<String>,
    },
    /// Display a remote URL
    Url {
        /// URL to display
        url: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Create base options based on command
    let content = match cli.command {
        Commands::Html { html } => ViewerContent::InlineHtml {
            html,
            base_dir: None,
        },
        Commands::File { path } => ViewerContent::LocalFile { path },
        Commands::Dir { root, entry } => ViewerContent::AppDir { root, entry },
        Commands::Url { url } => ViewerContent::RemoteUrl {
            url: Url::parse(&url)?,
        },
    };

    // Build window options
    let mut window = WindowOptions::default();
    if let Some(width) = cli.width {
        window.width = Some(width);
    }
    if let Some(height) = cli.height {
        window.height = Some(height);
    }
    if let Some(title) = cli.title {
        window.title = Some(title);
    }
    
    if cli.no_decorations {
        window.decorations = false;
    }
    
    if cli.transparent {
        window.transparent = true;
    }
    
    if cli.always_on_top {
        window.always_on_top = true;
    }

    if cli.show_toolbar {
        window.toolbar.show = true;
        if let Some(title) = cli.toolbar_title {
            window.toolbar.title_text = Some(title);
        }
    }

    // Build behaviour options
    let behaviour = BehaviourOptions {
        enable_devtools: cli.devtools,
        allow_remote_content: matches!(content, ViewerContent::RemoteUrl { .. }),
        ..Default::default()
    };

    // Build environment options
    let environment = EnvironmentOptions {
        timeout_seconds: cli.timeout,
        ..Default::default()
    };

    // Create full options
    let options = ViewerOptions {
        content,
        window,
        behaviour,
        environment,
        dialog: html_view::DialogOptions::default(),
        wait: html_view::ViewerWaitMode::Blocking,
    };

    // Open viewer
    match html_view::open(options)? {
        html_view::ViewerResult::Blocking(status) => {
            println!("Viewer exited: {:?}", status.reason);
        }
        _ => unreachable!(),
    }

    Ok(())
}
