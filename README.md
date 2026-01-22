# Timelog

Timelog is a simple, efficient and decision-making tracking tool built in Rust using the [GPUI](https://www.gpui.rs/) framework. It provides a distraction-free interface for logging generic timestamped entries, making it ideal for developers to track their train of thought, decisions made during coding sessions, or general work logs.

## Features

- **Chronological Tracking**: Automatically timestamps every entry you make.
- **Simple Storage**: Uses plain text files (`timelog.txt`) for storage, making your logs easy to read, backup, and version control.
- **Session Management**: Start a new session or load existing logs from any text file.
- **Clean UI**: A minimal, dark-themed interface designed for focus.
- **Keyboard-Driven**: Quickly add entries by typing and pressing Enter.
- **Global Hotkey**: Press `Ctrl+Shift+T` (Windows/Linux) or `Cmd+Shift+T` (macOS) to quickly show and focus the Timelog window from anywhere.
- **GPUI Powered**: Built with the high-performance GPUI framework for a responsive native experience.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- A platform supported by GPUI (macOS, Linux, Windows)

### Installation

Clone the repository and run the project using Cargo:

```bash
git clone <repository_url>
cd timelog
cargo run
```

## Usage

1. **Launch the Application**: Run `cargo run`.
2. **Global Hotkey**: Press `Ctrl+Shift+T` (Windows/Linux) or `Cmd+Shift+T` (macOS) to show/focus the window at any time.
3. **Start a Session**:
   - Click **"New Session"** to open/create `timelog.txt` in the current directory.
   - Click **"Load Session"** to browse for an existing text file.
4. **Log Entries**:
   - Type your thought or update in the input field at the bottom.
   - Press `Enter` to submit.
   - The entry will be displayed in the list and saved to the file immediately.

## File Format

Files are stored in a simple plain text format:

```text
[YYYY-MM-DD HH:MM:SS] Your log message here
[2026-01-22 09:42:56] Hello
[2026-01-22 11:27:29] Refactoring the main loop
```

## Technologies

- **Language**: Rust
- **UI Framework**: GPUI
- **Components**: `gpui-component`
- **Time Handling**: `chrono`
- **File Dialogs**: `rfd`
- **Global Hotkeys**: `global-hotkey`

## License

[MIT](LICENSE)
