# kaam

A fast, minimal command-line todo app written in Rust.

**kaam** (meaning "work" in Hindi) is a terminal-based task manager that helps you track your todos directly from the command line. No GUI, no bloat — just simple, efficient task management.

## Features

- **Fast & lightweight** — Built in Rust for speed and minimal resource usage
- **Colorful output** — Priority-based color coding for quick visual scanning
- **Priority levels** — Organize tasks by low, medium, or high priority
- **Due dates** — Set deadlines to stay on track
- **Filtering** — List tasks by status or priority
- **Persistent storage** — Tasks saved locally in JSON format
- **Zero dependencies at runtime** — Single binary, works offline

## Installation

### From source (requires Rust)

```bash
git clone https://github.com/always-akshat/kaam.git
cd kaam
cargo install --path .
```

### Build manually

```bash
git clone https://github.com/always-akshat/kaam.git
cd kaam
cargo build --release
# Binary available at ./target/release/kaam
```

## Usage

```bash
# Add tasks
kaam add "Buy groceries"
kaam add "Finish report" --priority high
kaam add "Call mom" --priority medium --due 2026-01-20

# List tasks
kaam list
kaam list --status pending
kaam list --status done
kaam list --priority high

# Mark task as done
kaam done 1

# Edit a task
kaam edit 1 --description "Updated task"
kaam edit 1 --priority low
kaam edit 1 --due 2026-02-01

# Delete a task
kaam delete 1

# Show detailed help
kaam usage
```

## Commands

| Command | Description |
|---------|-------------|
| `add` | Add a new task |
| `list` | List all tasks (with optional filters) |
| `done` | Mark a task as completed |
| `edit` | Modify an existing task |
| `delete` | Remove a task |
| `usage` | Show detailed usage examples |

## Options

### Add task options
- `-p, --priority <PRIORITY>` — Set priority (low, medium, high)
- `-d, --due <DATE>` — Set due date (YYYY-MM-DD format)

### List task options
- `-s, --status <STATUS>` — Filter by status (pending, done)
- `-p, --priority <PRIORITY>` — Filter by priority (low, medium, high)

### Edit task options
- `-d, --description <TEXT>` — Update task description
- `-p, --priority <PRIORITY>` — Update priority
- `--due <DATE>` — Update due date

## Data Storage

Tasks are stored in `~/.kaam.json` as human-readable JSON. You can back up, sync, or edit this file directly if needed.

## Why kaam?

- **Terminal-first** — For developers who live in the command line
- **Fast startup** — No slow electron apps or web interfaces
- **Simple** — Does one thing well: manage your todos
- **Portable** — Single binary, works on macOS, Linux, and Windows
- **Private** — All data stored locally, no accounts or cloud sync required

## Keywords

terminal todo app, command line task manager, cli todo list, rust todo app, terminal task manager, command line todo, minimalist todo app, developer todo app, productivity cli tool, task tracker terminal

## License

MIT
