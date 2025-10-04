# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based egui GUI wrapper for the [GIA (Google Intelligence Assistant)](https://github.com/panjamo/gia) CLI tool. The application provides a graphical interface to interact with GIA, which must be installed and available in PATH.

## Build and Run Commands

```bash
# Build the project
cargo build --release

# Run in development mode
cargo run

# Run with release optimizations
cargo run --release

# Format code
cargo fmt

# Run clippy lints
cargo clippy --fix
```

## Architecture

The application is a single-file Rust application (`src/main.rs`) using the eframe/egui framework for the GUI:

- **GiaApp struct**: Main application state holding prompt, options, checkboxes, and response text
- **Keyboard shortcuts**: Ctrl+Enter (send), Ctrl+R (record audio), Ctrl+L (clear), Ctrl+Shift+C (copy), Ctrl+O (conversation), F1 (help)
- **Command execution**: Uses `std::process::Command` to spawn `gia` CLI with appropriate arguments
- **Auto-resume**: After sending a prompt, the Resume checkbox is automatically enabled to continue the conversation

## Important Behaviors

- **Response box preservation**: The `show_conversation()` method (Ctrl+O) spawns the GIA command without clearing or modifying the response box. This is intentional - do not change this behavior.
- **Focus management**: The prompt input automatically receives focus on application start
- **Icon handling**: Application icon and logo are embedded from `icons/gia.png`

## Dependencies

- `eframe`/`egui`: GUI framework
- `arboard`: Clipboard access
- `image`: Icon/logo loading and processing
