# giagui

Simple egui-based GUI wrapper for the [GIA (Google Intelligence Assistant)](https://github.com/panjamo/gia) CLI tool.

## Features

- Prompt input with multi-line text editor
- Custom options input field
- Clipboard input option (-c)
- Browser output option (--browser-output)
- Resume conversation option (-R) - auto-enabled after sending prompts
- Response display with monospace font
- Copy response to clipboard
- Show conversation in browser (Ctrl+O)
- Help display (F1)
- Audio recording support (Ctrl+R)

## Keyboard Shortcuts

- **Ctrl+Enter**: Send prompt
- **Ctrl+R**: Send with audio recording
- **Ctrl+L**: Clear form
- **Ctrl+Shift+C**: Copy response to clipboard
- **Ctrl+O**: Show conversation in browser
- **F1**: Show help

## Requirements

- [gia](https://github.com/panjamo/gia) must be installed and available in PATH
- For local development: See [GIA README](C:\Development\github\gia\README.md)

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run
```
