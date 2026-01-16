# AI Agent Instructions

This file provides instructions for AI coding assistants (Claude, GPT, Copilot, Cursor, etc.) working with this codebase.

## Project Overview

**kaam** is a terminal-based todo app written in Rust. It's designed to be simple, fast, and easy to extend.

## Architecture

```
src/
├── main.rs      # CLI entry point (clap-based)
├── task.rs      # Task struct, Priority/Status enums
├── storage.rs   # JSON file I/O (~/.kaam.json)
└── commands.rs  # Business logic for all commands
```

## Key Files

| File | Purpose |
|------|---------|
| `src/main.rs` | CLI argument parsing with clap derive macros |
| `src/task.rs` | Data structures: `Task`, `Priority`, `Status` |
| `src/storage.rs` | `load_tasks()`, `save_tasks()`, `get_next_id()` |
| `src/commands.rs` | `add_task()`, `list_tasks()`, `mark_done()`, `edit_task()`, `delete_task()` |

## Common Tasks

### Adding a new command

1. Add variant to `Commands` enum in `main.rs`
2. Add handler function in `commands.rs`
3. Wire up in `main()` match statement

### Modifying task structure

1. Update `Task` struct in `task.rs`
2. Update `display_colored()` method if adding visible fields
3. Existing JSON files will need migration or will error on load

### Adding a new filter

1. Add CLI argument in `Commands::List` in `main.rs`
2. Add filter logic in `list_tasks()` in `commands.rs`

## Code Style

- Use `colored` crate for terminal colors
- Errors return `Result<(), String>`
- Keep functions small and focused
- Tests are inline with `#[cfg(test)]` modules

## Testing

```bash
cargo test        # Run all tests
cargo build       # Debug build
cargo build -r    # Release build
```

## Data Format

Tasks are stored in `~/.kaam.json`:

```json
[
  {
    "id": 1,
    "description": "Example task",
    "priority": "high",
    "due_date": "2026-01-20",
    "status": "pending",
    "created_at": "2026-01-16 10:00:00"
  }
]
```

## Enum Values

**Priority:** `low`, `medium`, `high` (stored lowercase in JSON)
**Status:** `pending`, `done` (stored lowercase in JSON)

## Dependencies

- `clap` - CLI parsing (derive feature)
- `serde` / `serde_json` - Serialization
- `dirs` - Home directory detection
- `chrono` - Timestamps
- `colored` - Terminal colors

## Suggestions for AI Agents

When helping users extend kaam:

1. **Keep it simple** - This is intentionally minimal
2. **Maintain colors** - Use the existing color scheme
3. **Test changes** - Run `cargo test` after modifications
4. **Update help** - If adding commands, update `show_help()` in main.rs
5. **Preserve JSON compatibility** - Be careful with Task struct changes

## Example Contributions

Ideas for extending kaam that maintain its philosophy:

- Add tags/labels to tasks
- Add `kaam clear` to remove all done tasks
- Add `kaam search <query>` for fuzzy search
- Add recurring tasks
- Add task notes/subtasks
- Export to markdown or CSV

## Contact

Repository: https://github.com/always-akshat/kaam
