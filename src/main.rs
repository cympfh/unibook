mod book;
mod builder;
mod config;
mod toc;
mod unidoc;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "unibook")]
#[command(about = "A documentation generator with table of contents", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the book
    Build {
        /// Path to the directory containing book.toml (default: current directory)
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
    },
    /// Initialize a new book
    Init {
        /// Path to create the new book (default: current directory)
        #[arg(default_value = ".")]
        dir: PathBuf,
    },
    /// Serve the book with HTTP server
    Serve {
        /// Path to the directory containing book.toml (default: current directory)
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
        /// Port to serve on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
    /// Watch for changes and rebuild
    Watch {
        /// Path to the directory containing book.toml (default: current directory)
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
        /// Watch unibook source code and auto-recompile (for development)
        #[arg(long, default_value = "false")]
        dev: bool,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        for cause in e.chain().skip(1) {
            eprintln!("  Caused by: {}", cause);
        }
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { dir } => build_book(&dir),
        Commands::Init { dir } => init_book(&dir),
        Commands::Serve { dir, port } => serve_book(&dir, port),
        Commands::Watch { dir, dev } => watch_book(&dir, dev),
    }
}

fn build_book(dir: &PathBuf) -> Result<()> {
    // Check if unidoc is available
    unidoc::check_unidoc_available()?;

    // Load config
    let config_path = dir.join("book.toml");
    if !config_path.exists() {
        anyhow::bail!(
            "book.toml not found in {}. Run 'unibook init' to create a new book.",
            dir.display()
        );
    }

    let config = config::Config::from_file(&config_path).context("Failed to load book.toml")?;

    // Create book
    let book = book::Book::from_config(config, dir).context("Failed to create book")?;

    // Build
    let builder = builder::Builder::new(book, dir).context("Failed to create builder")?;
    builder.build()?;

    Ok(())
}

fn init_book(dir: &PathBuf) -> Result<()> {
    let dir = if dir == &PathBuf::from(".") {
        env::current_dir().context("Failed to get current directory")?
    } else {
        dir.clone()
    };

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&dir).context("Failed to create directory")?;

    // Create book.toml
    let book_toml_path = dir.join("book.toml");
    if book_toml_path.exists() {
        anyhow::bail!("book.toml already exists in {}", dir.display());
    }

    let book_toml_content = r#"[book]
title = "My Book"
description = "A book built with unibook"
authors = ["Your Name"]

[build]
src_dir = "src"
output_dir = "doc"

[[pages]]
title = "Introduction"
path = "intro.md"

[[pages]]
title = "Chapter 1"
path = "chapter1.md"
"#;

    std::fs::write(&book_toml_path, book_toml_content).context("Failed to create book.toml")?;
    println!("Created {}", book_toml_path.display());

    // Create src directory
    let src_dir = dir.join("src");
    std::fs::create_dir_all(&src_dir).context("Failed to create src directory")?;
    println!("Created {}/", src_dir.display());

    // Create intro.md
    let intro_path = src_dir.join("intro.md");
    let intro_content = r#"# Introduction

Welcome to your book!

This is the introduction page. You can edit this file to add your content.

## Getting Started

1. Edit the markdown files in the `src/` directory
2. Update `book.toml` to configure your book
3. Run `unibook build` to generate HTML

## Navigation

The table of contents will appear on the left side of each page, allowing readers to easily navigate between chapters.
"#;

    std::fs::write(&intro_path, intro_content).context("Failed to create intro.md")?;
    println!("Created {}", intro_path.display());

    // Create chapter1.md
    let chapter1_path = src_dir.join("chapter1.md");
    let chapter1_content = r#"# Chapter 1

This is the first chapter of your book.

## Section 1.1

Add your content here.

## Section 1.2

Add more content here.
"#;

    std::fs::write(&chapter1_path, chapter1_content).context("Failed to create chapter1.md")?;
    println!("Created {}", chapter1_path.display());

    println!("\nBook initialized successfully!");
    println!("Run 'unibook build' to build your book.");

    Ok(())
}

fn serve_book(dir: &PathBuf, port: u16) -> Result<()> {
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
    use std::sync::{Arc, Mutex, mpsc::channel};
    use std::thread;
    use std::time::Duration;

    // First, build the book
    build_book(dir)?;

    // Load config to get output directory
    let config_path = dir.join("book.toml");
    let config = config::Config::from_file(&config_path)?;
    let src_dir = dir.join(&config.build.src_dir);
    let output_dir = dir.join(&config.build.output_dir);

    let addr = format!("0.0.0.0:{}", port);
    let server = Arc::new(
        tiny_http::Server::http(&addr)
            .map_err(|e| anyhow::anyhow!("Failed to start server on {}: {}", addr, e))?,
    );

    println!("\nServing book at http://localhost:{}/", port);
    println!("Watching for changes in {}...", src_dir.display());
    println!("Press Ctrl+C to stop\n");

    // Setup file watcher
    let (watch_tx, watch_rx) = channel();
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = watch_tx.send(event);
            }
        },
        Config::default(),
    )?;

    watcher.watch(&src_dir, RecursiveMode::Recursive)?;
    watcher.watch(&config_path, RecursiveMode::NonRecursive)?;

    // Spawn watcher thread
    let dir_clone = dir.clone();
    let watcher_handle = thread::spawn(move || {
        let mut last_build = std::time::Instant::now();

        loop {
            match watch_rx.recv_timeout(Duration::from_millis(100)) {
                Ok(event) => {
                    if last_build.elapsed() > Duration::from_millis(500) {
                        println!("\n[Watch] Change detected: {:?}", event);
                        println!("[Watch] Rebuilding...");
                        match build_book(&dir_clone) {
                            Ok(_) => println!("[Watch] Build successful!\n"),
                            Err(e) => eprintln!("[Watch] Build failed: {}\n", e),
                        }
                        last_build = std::time::Instant::now();
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // Continue
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }
    });

    // Handle HTTP requests
    for request in server.incoming_requests() {
        let url: String = request.url().to_string();

        // Handle root path
        let file_path = if url == "/" {
            output_dir.join("index.html")
        } else {
            // Remove leading slash and join with output dir
            let path = url.trim_start_matches('/');
            output_dir.join(path)
        };

        if file_path.exists() && file_path.is_file() {
            match std::fs::read(&file_path) {
                Ok(content) => {
                    // Determine content type
                    let content_type =
                        if file_path.extension().and_then(|s| s.to_str()) == Some("html") {
                            "text/html; charset=utf-8"
                        } else if file_path.extension().and_then(|s| s.to_str()) == Some("css") {
                            "text/css"
                        } else if file_path.extension().and_then(|s| s.to_str()) == Some("js") {
                            "application/javascript"
                        } else {
                            "application/octet-stream"
                        };

                    let response = tiny_http::Response::from_data(content).with_header(
                        tiny_http::Header::from_bytes(
                            &b"Content-Type"[..],
                            content_type.as_bytes(),
                        )
                        .unwrap(),
                    );
                    let _ = request.respond(response);
                }
                Err(_) => {
                    let response = tiny_http::Response::from_string("500 Internal Server Error")
                        .with_status_code(500);
                    let _ = request.respond(response);
                }
            }
        } else {
            let response = tiny_http::Response::from_string("404 Not Found").with_status_code(404);
            let _ = request.respond(response);
        }
    }

    drop(watcher);
    let _ = watcher_handle.join();

    Ok(())
}

fn watch_book(dir: &PathBuf, dev_mode: bool) -> Result<()> {
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;

    // Load config to get source directory
    let config_path = dir.join("book.toml");
    let config = config::Config::from_file(&config_path)?;
    let src_dir = dir.join(&config.build.src_dir);

    // Initial build
    println!("Initial build...");
    build_book(dir)?;

    println!("\nWatching for changes in {}...", src_dir.display());
    if dev_mode {
        println!("Dev mode: Also watching unibook source for changes");
    }
    println!("Press Ctrl+C to stop");

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default(),
    )?;

    // Watch the book source directory
    watcher.watch(&src_dir, RecursiveMode::Recursive)?;
    // Also watch book.toml
    watcher.watch(&config_path, RecursiveMode::NonRecursive)?;

    // If dev mode, also watch unibook's src directory
    if dev_mode {
        // Assume we're running from the workspace
        if let Ok(current_dir) = env::current_dir() {
            let unibook_src_dir = current_dir.join("src");
            let unibook_cargo = current_dir.join("Cargo.toml");

            if unibook_src_dir.exists() && unibook_cargo.exists() {
                println!("Watching unibook source: {}", unibook_src_dir.display());
                watcher.watch(&unibook_src_dir, RecursiveMode::Recursive)?;
            }
        }
    }

    let mut last_build = std::time::Instant::now();
    let mut last_compile = std::time::Instant::now();

    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => {
                // Check if this is a unibook source file change
                let is_unibook_source = if dev_mode {
                    if let Some(paths) = event.paths.first() {
                        paths.to_string_lossy().contains("/src/")
                            && (paths.extension().and_then(|s| s.to_str()) == Some("rs")
                                || paths.file_name().and_then(|s| s.to_str()) == Some("Cargo.toml"))
                    } else {
                        false
                    }
                } else {
                    false
                };

                if is_unibook_source {
                    // Debounce: only recompile if 1s has passed since last compile
                    if last_compile.elapsed() > Duration::from_millis(1000) {
                        println!("\n[DEV] Unibook source changed, recompiling...");

                        let output = std::process::Command::new("cargo").arg("build").output();

                        match output {
                            Ok(out) if out.status.success() => {
                                println!("[DEV] Recompile successful!");
                                // Also rebuild the book
                                println!("Rebuilding book...");
                                match build_book(dir) {
                                    Ok(_) => println!("Build successful!"),
                                    Err(e) => eprintln!("Build failed: {}", e),
                                }
                            }
                            Ok(out) => {
                                eprintln!("[DEV] Recompile failed:");
                                eprintln!("{}", String::from_utf8_lossy(&out.stderr));
                            }
                            Err(e) => {
                                eprintln!("[DEV] Failed to run cargo: {}", e);
                            }
                        }
                        last_compile = std::time::Instant::now();
                        last_build = std::time::Instant::now();
                    }
                } else {
                    // Regular book rebuild
                    // Debounce: only rebuild if 500ms have passed since last build
                    if last_build.elapsed() > Duration::from_millis(500) {
                        println!("\nChange detected: {:?}", event);
                        println!("Rebuilding...");
                        match build_book(dir) {
                            Ok(_) => println!("Build successful!"),
                            Err(e) => eprintln!("Build failed: {}", e),
                        }
                        last_build = std::time::Instant::now();
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // No events, continue
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
        }
    }

    Ok(())
}
