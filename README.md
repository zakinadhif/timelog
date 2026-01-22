# Timelog

Timelog is a simple, efficient devlog and decision-making tracking tool built with Rust and [GPUI](https://www.gpui.rs/). It provides a clean interface for maintaining a chronological journal of your development process, thoughts, and decisions.

## Features

- **Chronological Tracking**: Automatically timestamps every entry you make.
- **Simple Storage**: Uses plain text files (`timelog.txt`) for storage, making your logs easy to read, backup, and version control.
- **Session Management**: Start a new session or load existing logs from any text file.
- **Clean UI**: A minimal, dark-themed interface designed for focus.
- **Keyboard-Driven**: Quickly add entries by typing and pressing Enter.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### Running the Application

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/timelog.git
   cd timelog
   ```

2. Run with Cargo:
   ```bash
   cargo run
   ```

## Usage

1. **New Session**: Click "New Session" to start logging to the default `timelog.txt` file in the current directory.
2. **Load Session**: Click "Load Session" to open a file picker and select an existing log file.
3. **Logging**: 
   - Type your thought, code change, or decision in the input field at the bottom.
   - Press **Enter** to save.
   - The entry will be appended to the file and displayed in the view.

## File Format

Entries are stored in a simple, human-readable format:

```text
[YYYY-MM-DD HH:MM:SS] Your log message here
[2024-03-20 10:15:30] Started working on the new feature
[2024-03-20 10:45:00] Decided to use GPUI for the frontend because of its performance
```

## Tech Stack

- **Language**: Rust
- **UI Framework**: GPUI
- **Components**: `gpui-component`
- **Time Handling**: `chrono`
