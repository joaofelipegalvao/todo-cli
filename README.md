# Todo CLI 🦀

> A minimalist command-line task manager built to learn Rust

A simple, colorful, and functional task manager developed to learn Rust in practice, focusing on CLI design, file manipulation, error handling, and visual UX.

## Features

- **Priority system** (high, medium, low)
- **Advanced search and filters**
- **Progress statistics**
- **Colorful interface**
- **Fast and lightweight**

## Quick Start

```bash
# Install
git clone https://github.com/joaofelipegalvao/todo-cli
cd todo-cli
cargo install --path .

# Use
todo add "Learn Rust" --high
todo list --pending --sort
todo done 1
```

## Commands

```bash
todo add <task> [--high|--low]      # Add task (default: medium priority)
todo list [flags]                    # List tasks
todo search <term>                   # Search tasks
todo done <number>                   # Mark as completed
todo undone <number>                 # Unmark task
todo remove <number>                 # Remove task
todo clear                           # Remove all tasks
```

### List Flags

- `--pending` / `--done` - Filter by status
- `--high` / `--low` - Filter by priority
- `--sort` - Sort by priority
- Combine flags: `todo list --pending --high --sort`

## Documentation

- **[Complete Guide](docs/GUIDE.md)** - All commands and examples
- **[Learning Journey](docs/LEARNING.md)** - How this project evolved, version by version
- **[Changelog](CHANGELOG.md)** - Version history

## Educational Project

This project was developed as a Rust learning exercise, documenting each incremental step. Each version represents a learning milestone:

| Version | Feature | Key Concepts |
|---------|---------|--------------|
| v0.1.0 | Basic CLI | `OpenOptions`, `match`, `?` operator |
| v0.2.0 | Mark as done | `.map()`, `.collect()`, `Vec<String>` |
| v0.8.0 | Priorities + Filters | `Option<T>`, pattern matching, pipeline |
| v1.0.0 | Search + Refactoring | Atomic functions, separation of concerns |

[See full evolution →](docs/LEARNING.md)

### For Students

- Clone, read the code, explore version diffs  
- Each tag documents what was learned  
- Perfect for understanding CLI design in Rust  

### For End Users

- The code works but lacks comprehensive automated tests  
- May have unhandled edge cases  
- Use at your own risk, or contribute improvements!  

## Roadmap

### Completed

- Basic CRUD operations
- Priority system with visual indicators
- Advanced filters and search
- Sorting by priority
- Optimized pipeline architecture

### Planned

- Tags/categories (`#work`, `#home`)
- Edit command
- Due dates
- JSON export
- Unit tests
- Refactoring with structs

## Contributing

Contributions are welcome! This is a learning project, so feel free to:

- **Report bugs** - Open an issue with details
- **Suggest features** - Share your ideas
- **Improve documentation** - Fix typos, add examples
- **Submit PRs** - Fix bugs or add features

## License

**MIT License** - Educational project developed to learn Rust 🦀

See [LICENSE](https://github.com/joaofelipegalvao/todo-cli/blob/main/LICENSE) for full details.

---

**Each commit represents a step in the learning process**
