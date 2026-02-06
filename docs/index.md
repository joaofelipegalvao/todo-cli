# 🦀 Todo CLI Learning Wiki

> Complete learning journey from basic CLI to professional task manager

This wiki contains the complete documentation of how the todo-cli project evolved, version by version. Each version represents a learning milestone with detailed explanations of concepts, design decisions, and implementation details.

## Navigation

### Getting Started

For beginners learning Rust and CLI development:

- [v0.1.0 - Basic CLI](getting-started/v0.1.0-basic-cli.md) - First CLI with add/list
- [v0.2.0 - Done Command](getting-started/v0.2.0-done-command.md) - Mark tasks as completed
- [v0.3.0 - Remove Command](getting-started/v0.3.0-remove-command.md) - Delete specific tasks
- [v0.4.0 - Undone Command](getting-started/v0.4.0-undone-command.md) - Unmark completed tasks
- [v0.4.1 - List Bug Fix](getting-started/v0.4.1-list-bug-fix.md) - Handle empty lines properly
- [v0.4.2 - State Validations](getting-started/v0.4.2-state-validations.md) - Prevent invalid state transitions
- [v0.5.0 - Clear Command](getting-started/v0.5.0-clear-command.md) - Remove all tasks at once

### Intermediate Features

For those comfortable with Rust basics:

- [v0.6.0 - Visual Interface with Colors](intermediate/v0.6.0-visual-interface-colors.md) - Add colorful visual hierarchy
- [v0.7.0 - Advanced Filters](intermediate/v0.7.0-advanced-filters.md) - Filter by status with helper functions
- [v0.8.0 - Priority System](intermediate/v0.8.0-priority-system.md) - Three-level priority with visual indicators
- [v0.9.0 - Priority Sorting](intermediate/v0.9.0-priority-sorting.md) - Sort tasks by priority
- [v1.0.0 - Search + Refactoring](intermediate/v1.0.0-search-refactoring.md) - Add search and refactor display
- [v1.1.0 - Medium Priority Filter](intermediate/v1.1.0-medium-priority-filter.md) - Complete priority filtering system

### Advanced Architecture

For experienced developers:

- [v1.2.0 - Struct Refactoring](advanced/v1.2.0-struct-refactoring.md) - Type-safe architecture with structs/enums
- [v1.3.0 - JSON Serialization](advanced/v1.3.0-json-serialization.md) - Replace custom format with serde
- [v1.4.0 - Tags System](advanced/v1.4.0-tags-system.md) - Add categorization with tags
- [v1.5.0 - Due Dates + Tabular Display](advanced/v1.5.0-due-dates-tabular.md) - Deadline tracking with chrono
- [v1.6.0 - Professional CLI with Clap](advanced/v1.6.0-professional-cli-clap.md) - Industry-standard CLI framework

### Cross-Cutting Concepts

Key patterns and best practices used throughout the project:

- [Error Handling](concepts/error-handling.md) - From basic `?` to professional error messages
- [File Operations](concepts/file-operations.md) - File I/O patterns and JSON serialization
- [CLI Design](concepts/cli-design.md) - Command-line interface patterns and user experience
- [Type Safety](concepts/type-safety.md) - Using Rust's type system to prevent bugs

## Learning Path

### Phase 1: Fundamentals (v0.1.0 - v0.5.0)

**Focus:** Make it work

- Basic Rust syntax and ownership
- File I/O operations
- String manipulation
- Error handling with `?`
- Pattern matching

### Phase 2: Polish (v0.6.0 - v1.1.0)

**Focus:** Make it nice

- Visual design with colors
- User experience patterns
- Code organization (DRY)
- Helper functions
- Advanced filtering

### Phase 3: Architecture (v1.2.0 - v1.6.0)

**Focus:** Make it right

- Type-safe design with structs/enums
- Professional CLI patterns
- Automatic serialization
- Industry-standard frameworks

## Project Evolution

```
v0.1: String matching everywhere
   ↓
v1.2: Type-safe structs and enums (36% reduction)
   ↓
v1.3: Automatic JSON serialization (91% I/O reduction)
   ↓
v1.4: Extensible with tags (1 line = new feature)
   ↓
v1.5: Due dates + tabular display (deadline tracking + professional UX)
   ↓
v1.6: Clap + ValueEnum (zero manual parsing, compile-time safety)
```

## Key Achievements

### Code Quality

- **36% reduction** in total lines after struct refactoring
- **91% reduction** in I/O code after JSON migration
- **Zero manual parsing** after adopting clap
- **Type safety** from command line to storage

### Features

- ✅ Complete CRUD operations
- ✅ Priority system with visual indicators
- ✅ Advanced filters and search
- ✅ Tags for categorization
- ✅ Due dates with deadline tracking
- ✅ Professional CLI with auto-help
- ✅ Type-safe architecture throughout

### Learning Outcomes

1. **Rust fundamentals** through practical application
2. **CLI design patterns** that feel natural to users
3. **Type safety** - using compiler to prevent bugs
4. **Refactoring strategy** - evolve code without breaking it
5. **Professional development** - industry-standard patterns

## Technical Stack

- **Language:** Rust 🦀
- **CLI Framework:** Clap (v1.6.0+)
- **Serialization:** Serde + JSON
- **Colors:** Colored crate
- **Dates:** Chrono
- **File Format:** JSON (v1.3.0+)

## Version Summary

| Version | Feature | Key Concepts | Lines of Code |
|---------|---------|--------------|--------------|
| v0.1.0 | Basic CLI | `OpenOptions`, `match`, `?` | ~50 |
| v0.2.0 | Done Command | `.map()`, `.collect()`, `Vec<String>` | ~80 |
| v0.3.0 | Remove Command | Index validation, `Vec::remove()` | ~100 |
| v0.4.0 | Undone Command | State machine, inverse operations | ~120 |
| v0.5.0 | Clear Command | `fs::metadata()`, idempotent operations | ~130 |
| v0.6.0 | Visual Interface | `colored` crate, visual hierarchy | ~180 |
| v0.7.0 | Advanced Filters | Helper functions, DRY principle | ~200 |
| v0.8.0 | Priority System | `Option<T>`, pattern matching, pipeline | ~250 |
| v0.9.0 | Priority Sorting | `.sort_by()`, `Ordering`, optimization | ~270 |
| v1.0.0 | Search + Refactoring | Atomic functions, separation of concerns | ~290 |
| v1.1.0 | Medium Filter | API completeness, symmetry design | ~300 |
| v1.2.0 | Struct Refactoring | Type safety, 36% code reduction | ~115 |
| v1.3.0 | JSON Serialization | Serde, 91% I/O reduction | ~5 |
| v1.4.0 | Tags System | `Vec<String>`, `.retain()`, bug fixes | ~120 |
| v1.5.0 | Due Dates | `chrono`, date arithmetic, tabular display | ~150 |
| v1.6.0 | Professional CLI | Clap, `ValueEnum`, zero manual parsing | ~80 |

## For Students

### How to Use This Wiki

1. **Read chronologically** - Start with v0.1.0 and progress through versions
2. **Study the code** - Each version links to the exact commit
3. **Understand the "why"** - Each version explains design decisions
4. **Try the concepts** - Implement similar patterns in your projects
5. **Compare approaches** - See how code evolved from strings to structs

### Learning Goals

After studying this wiki, you should understand:

- **Rust fundamentals**: Ownership, borrowing, lifetimes, error handling
- **CLI design**: Subcommands, flags, help generation, user experience
- **Code organization**: When to use functions, structs, enums
- **Type safety**: How to use Rust's type system to prevent bugs
- **Refactoring**: How to evolve code without breaking functionality
- **Professional development**: Industry-standard patterns and tools

### For Each Version

1. **Read the documentation** - Understand the goals and concepts
2. **Examine the code** - Look at the actual implementation
3. **Study the diff** - See what changed from previous version
4. **Run the code** - Test the functionality yourself
5. **Experiment** - Try modifying and extending the features

## Next Steps

The CLI is now production-ready and serves as an excellent foundation for learning more advanced Rust concepts:

### Potential Future Versions

- **v1.7:** Recurring tasks with chrono patterns
- **v1.8:** Subtasks/nested tasks with recursive data structures
- **v1.9:** Multiple projects/contexts
- **v2.0:** TUI with `ratatui`
- **v2.1:** Configuration file with `config` crate
- **v2.2:** Shell completions (bash, zsh, fish)
- **v2.3:** Export/import (CSV, JSON, Markdown)
- **v2.4:** Sync with cloud storage
- **v2.5:** Web API with `axum`
- **v3.0:** Plugin system

### Learning Extensions

Each future version would teach new Rust concepts while building on the solid foundation established here.

---

**The beauty of this architecture:** All new features benefit from the type-safe, extensible foundation built through careful refactoring.

**🦀 Happy learning!**

