# DevTimer

A simple, fast, and efficient command-line time tracking tool written in Rust.

## Description

DevTimer is a lightweight CLI application that helps developers track time spent on various tasks. It stores tracking data locally in JSON format, making it easy to manage and review your work sessions.

## Features

- **Fast and Lightweight** - Built with Rust for optimal performance
- **Simple Commands** - Easy-to-remember commands for starting, stopping, and viewing timers
- **Local Storage** - All data stored locally in JSON format
- **Detailed Statistics** - View elapsed time in days, hours, minutes, and seconds
- **No Dependencies** - Self-contained executable, no external services required

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or higher)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/devtimer.git
cd devtimer

# Build the project
cargo build --release

# The executable will be available at ./target/release/devtimer
```

### Add to PATH (Optional)

To use `devtimer` from anywhere, add the executable to your PATH:

**Windows:**
```powershell
# Copy to a directory in your PATH or add the target/release directory to PATH
```

**Linux/macOS:**
```bash
# Copy to a directory in your PATH
sudo cp target/release/devtimer /usr/local/bin/
```

## Usage

### Quick Start

```bash
# Start tracking time
devtimer start Working on feature X

# Stop the timer
devtimer stop

# View statistics for the last entry
devtimer statistics
```

### Commands

#### Start Timer
```bash
devtimer start <description>
```
Starts a new timer with the given description.

**Example:**
```bash
devtimer start Implementing user authentication
```

#### Stop Timer
```bash
devtimer stop
```
Stops the currently running timer.

#### View Statistics
```bash
devtimer statistics
```
Displays the duration and description of the last tracked time entry.

**Example Output:**
```
Task: Implementing user authentication
Duration: 0 days, 2 hours, 34 minutes, 12 seconds
```

### Help
```bash
devtimer
```
Running `devtimer` without arguments shows the help menu with available commands.

## Data Storage

DevTimer stores all tracking data in a `saved_times.json` file in the current directory. The file contains:
- Task description
- Start timestamp
- Stop timestamp (when stopped)

**Example JSON structure:**
```json
[
  {
    "text": "Working on feature X",
    "timestamp_start": "2026-02-08 14:30:00",
    "timestamp_stop": "2026-02-08 16:45:30"
  }
]
```

## Roadmap

- [ ] List all tracked entries
- [ ] Export data to CSV
- [ ] Daily/weekly/monthly summaries
- [ ] Multiple simultaneous timers
- [ ] Tags and categories
- [ ] Interactive TUI mode

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [serde](https://serde.rs/) for JSON serialization
- Uses [chrono](https://github.com/chronotope/chrono) for date/time handling

## Contact

For questions or suggestions, please open an issue on GitHub.

---

Made with love by developers, for developers
