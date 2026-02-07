# Todo CLI 🦀

> A minimalist command-line task manager built to learn Rust

A simple, colorful, and functional task manager developed to learn Rust in practice, focusing on CLI design, file manipulation, error handling, type safety, and visual UX.

## Features

- **Error Handling with `anyhow` and `thiserror`**
- **Professional CLI with Clap** - Auto-generated help, type-safe parsing, shell completions
- **Type-safe architecture** with structs and enums
- **Tags system** for task categorization
- **Priority system** (high, medium, low)
- **Due dates** with automatic validation
- **Advanced search and filters**
- **Progress statistics**
- **Colorful interface**
- **JSON storage** with automatic serialization
- **Fast and lightweight**

## Quick Start

```bash
# Install
git clone https://github.com/joaofelipegalvao/todo-cli
cd todo-cli
cargo install --path .

# Use
todo add "Learn Rust" --priority high --tag programming --tag learning --due 2026-02-20
todo list --status pending --sort priority
todo done 1
todo tags
```

## Commands

```bash
todo add <description> [options]              # Add a new task
todo list [options]                           # List and filter tasks
todo search <query> [--tag <name>]           # Search tasks by text
todo done <id>                                # Mark task as completed
todo undone <id>                              # Mark task as pending
todo remove <id>                              # Remove a task
todo clear                                    # Remove all tasks
todo tags                                     # List all tags with counts
```

### Add Command

```bash
todo add "Task description" [options]

Options:
  --priority <PRIORITY>   Task priority: high, medium (default), low
  -t, --tag <TAG>        Add tags (repeatable: -t work -t urgent)
  --due <DATE>           Due date in YYYY-MM-DD format
```

**Examples:**

```bash
todo add "Deploy to production" --priority high --tag work --due 2026-02-15
todo add "Buy groceries" -t personal -t shopping
```

### List Command

```bash
todo list [options]

Filters:
  --status <STATUS>      Filter by status: all (default), pending, done
  --priority <PRIORITY>  Filter by priority: high, medium, low
  --due <DUE_FILTER>    Filter by due date: overdue, soon, with-due, no-due
  --tag <TAG>           Filter by tag name

Sorting:
  -s, --sort <SORT_BY>  Sort by: priority, due, created
```

**Examples:**

```bash
# Pending high-priority tasks, sorted by due date
todo list --status pending --priority high --sort due

# Tasks due soon
todo list --due soon

# Combine multiple filters
todo list --status pending --priority high --tag work --sort priority
```

### Command Aliases

For faster typing:

```bash
todo a "Task"          # alias for 'add'
todo ls                # alias for 'list'
todo rm 3              # alias for 'remove'
todo delete 3          # also works for 'remove'
```

### Getting Help

```bash
todo --help            # Show all commands
todo add --help        # Help for specific command
todo list --help       # Detailed filtering options
```

## Documentation

- **[Complete Guide](docs/GUIDE.md)** - All commands and examples
- **[Complete Documentation](https://joaofelipegalvao.github.io/todo-cli/)** - Full learning journey, concepts, and examples
- **[Changelog](CHANGELOG.md)** - Version history

---

## Educational Project

This project was developed as a Rust learning exercise, documenting each incremental step. Each version represents a learning milestone:

| Version | Feature | Key Concepts |
|---------|---------|--------------|
| v0.1.0 | Basic CLI | `OpenOptions`, `match`, `?` operator |
| v0.2.0 | Mark as done | `.map()`, `.collect()`, `Vec<String>` |
| v0.8.0 | Priorities + Filters | `Option<T>`, pattern matching, pipeline |
| v1.0.0 | Search + Refactoring | Atomic functions, separation of concerns |
| v1.2.0 | Type-safe structs | `struct`, `enum`, `impl`, derive macros |
| v1.3.0 | JSON serialization | `serde`, automatic serialization, 91% I/O reduction |
| v1.4.0 | Tags system | `Vec<String>`, `.retain()`, bug fixes |
| v1.5.0 | Due dates | `chrono`, `NaiveDate`, date validation |
| v1.6.0 | Professional CLI | `clap`, derive macros, type-safe enums, auto-help |
| v1.7.0 | Error Handling | `anyhow`, `thiserror`, error chains |

[See full evolution →](/CHANGELOG.md)

### For Students

- Clone, read the code, explore version diffs  
- Each tag documents what was learned  
- Perfect for understanding CLI design in Rust
- Study the evolution from manual parsing to professional CLI with Clap

### For End Users

- The code works but lacks comprehensive automated tests  
- May have unhandled edge cases  
- Use at your own risk, or contribute improvements!  

## Roadmap

### Completed ✅

- Basic CRUD operations
- Priority system with visual indicators
- Advanced filters and search
- Sorting by priority and due date
- Optimized pipeline architecture
- Type-safe architecture with structs/enums
- JSON serialization with serde
- Tags system for categorization
- **Due dates with automatic validation**
- **Professional CLI with Clap framework**
- **Type-safe filtering with enums**
- **Auto-generated help and documentation**
- **Command aliases for productivity**

### Planned 🚀

- Edit command
- Recurring tasks
- Subtasks/nested tasks
- Export/import commands
- Shell completions (bash, zsh, fish)
- Unit tests
- TUI (Terminal User Interface)

## Contributing

Contributions are welcome! This is a learning project, so feel free to:

- **Report bugs** - Open an issue with details
- **Suggest features** - Share your ideas
- **Improve documentation** - Fix typos, add examples
- **Submit PRs** - Fix bugs or add features
- **Share learning insights** - Add to the wiki

### Development

```bash
# Clone and build
git clone https://github.com/joaofelipegalvao/todo-cli
cd todo-cli
cargo build

# Run tests (when available)
cargo test

# Run with logging
RUST_LOG=debug cargo run -- add "Test task"

# Check code quality
cargo clippy
cargo fmt --check
```

## License

**MIT License** - Educational project developed to learn Rust 🦀

See [LICENSE](https://github.com/joaofelipegalvao/todo-cli/blob/main/LICENSE) for full details.

---

**Built with ❤️ to learn Rust - Each commit represents a step in the learning journey**
