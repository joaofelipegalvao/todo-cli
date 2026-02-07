## Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- Edit command
- Recurring tasks
- Subtasks/nested tasks
- Export/import commands
- Shell completions (bash, zsh, fish)
- Unit tests
- TUI (Terminal User Interface)

## [1.7.0] - 2026-02-07

### Added

- **Professional error handling** using `anyhow` and `thiserror` crates
- `TodoError` enum for domain-specific errors with rich data
- Custom error variants:
  - `InvalidTaskId { id, max }` - Shows valid range in error message
  - `TaskAlreadyInStatus { id, status }` - Prevents duplicate state changes
  - `TagNotFound(String)` - Clear message when filtering by non-existent tag
  - `NoTasksFound` - Better UX when filters return empty results
  - `NoTagsFound` - Informative message when no tags exist
  - `NoSearchResults(String)` - Shows search query in error message
- `validate_task_id()` helper function for centralized ID validation
- Error chain display in `main()` - Shows full "Caused by:" chain
- Rich context with `.context()` on all operations
- Specific error messages for each failure mode
- Pattern matching on `std::io::ErrorKind` for granular error handling

### Changed

- **BREAKING CHANGE:** Error type migrated from `Box<dyn Error>` to `anyhow::Result`
- All function signatures updated to use `Result<T>` (shorthand for `Result<T, anyhow::Error>`)
- `load_tasks()`: Added context for parse failures and read errors
- `save_tasks()`: Added context for serialization and write failures
- `done` command: Added validation for already-completed tasks
- `undone` command: Added validation for already-pending tasks
- `remove` command: Uses centralized `validate_task_id()`
- `list` command: Returns `TodoError::NoTasksFound` instead of printing
- `search` command: Returns `TodoError::NoSearchResults` instead of printing
- `tags` command: Returns `TodoError::NoTagsFound` instead of printing
- Error display: Shows colored "Error:" prefix and "Caused by:" chain
- `main()`: Enhanced error display with full error chain traversal

### Fixed

- Clippy warning in `get_due_text()` - Removed redundant `else if` block
- Generic error messages replaced with specific, actionable messages
- Lost error context - Now displays full error chain
- User confusion about error causes - Shows root cause with "Caused by:"
- Missing context in file operations - All I/O now includes file path context
- State validation gaps - All state transitions now validated

### Changed

- Migrated project documentation to MkDocs
- Replaced legacy markdown docs with structured MkDocs layout

### Removed

- Deprecated `LEARNING.md`

## [1.6.0] - 2026-02-04

### Added

- **Professional CLI framework** using `clap` crate (v4.5) with derive macros
- **Type-safe filtering** with enum-based arguments
- `StatusFilter` enum: `All`, `Pending`, `Done` (replaces boolean flags)
- `DueFilter` enum: `Overdue`, `Soon`, `WithDue`, `NoDue` (replaces 4 boolean flags)
- `SortBy` enum: `Priority`, `Due`, `Created` (replaces string-based sorting)
- `ValueEnum` trait implementation on `Priority` for CLI value parsing
- **Auto-generated help text** with `#[command()]` attributes
- Command-level help with `long_about` descriptions
- Argument-level help with doc comments (appear in `--help` output)
- **Command aliases** for improved productivity:
  - `a` for `add`
  - `ls` for `list`
  - `rm` and `delete` for `remove`
- **Automatic argument parsing** with type safety and validation
- **Automatic NaiveDate parsing** using `clap::value_parser!(NaiveDate)`
- Professional error messages with context and suggestions
- `Cli` struct with `#[derive(Parser)]` for top-level command handling
- `Commands` enum with `#[derive(Subcommand)]` for subcommand routing
- `AddArgs` struct with `#[derive(Args)]` for complex add command arguments
- Short flags: `-t` for `--tag`, `-s` for `--sort`
- Repeatable arguments: `--tag` can be used multiple times
- `Task::matches_status()` helper method for status filtering
- `Task::matches_due_filter()` helper method for due date filtering
- Metadata in help output: program name, author, version, examples
- `after_help` examples section showing common usage patterns

### Changed

- **BREAKING CHANGE:** Complete CLI interface redesigned with Clap
- **BREAKING CHANGE:** Command syntax updated across all commands:
  - `--high/--medium/--low` → `--priority high/medium/low`
  - `--pending/--done` → `--status pending/done/all`
  - `--overdue/--due-soon/--with-due/--without-due` → `--due overdue/soon/with-due/no-due`
  - `--sort` → `--sort priority/due/created`
- **BREAKING CHANGE:** Priority values in JSON now lowercase (`"high"`, `"medium"`, `"low"`)
- `Priority` enum: Added `ValueEnum` derive for CLI integration
- Argument parsing: Manual `env::args()` parsing → Clap derive macros
- Help generation: Manual help strings → Automatic from struct attributes
- Validation: Manual conflict checking → Automatic with type system
- Error messages: Generic strings → Contextual clap-generated errors
- `main()` function: Simplified with `Cli::parse()` handling all parsing
- Filter mutual exclusion: Runtime checks → Compile-time type safety with enums
- Boolean filter flags (4 for dates) → Single `Option<DueFilter>` enum
- Parsing complexity: ~100 lines of manual parsing → ~20 lines of declarative structs
- Code organization: Flat argument handling → Structured with dedicated types

### Removed

- Manual argument parsing with `env::args().collect()`
- Manual flag conflict validation (15+ lines)
- Manual help text construction
- Generic "Usage: ..." error messages
- Manual `NaiveDate` parsing and error handling (8 lines → 1 line with `value_parser!`)
- Boolean flags for status filters (`--pending`, `--done`)
- Boolean flags for priority filters (`--high`, `--medium`, `--low`)
- Boolean flags for date filters (`--overdue`, `--due-soon`, `--with-due`, `--without-due`)
- String-based sorting validation

### Fixed

- Eliminated possibility of conflicting filter flags (enforced by type system)
- Improved error messages with suggestions for typos (e.g., "did you mean 'high'?")
- Consistent argument naming across all commands
- Professional command-line interface matching industry standards

### Technical Details

- **Dependencies:**
  - Added: `clap = { version = "4.5", features = ["derive"] }`
  - Enables derive macros for declarative CLI definition
- **Code reduction:** ~100 lines of parsing code → ~20 lines of struct definitions
- **Type safety improvements:**
  - `StatusFilter` enum prevents invalid status values at compile time
  - `DueFilter` enum ensures only one date filter used (via `Option<T>`)
  - `SortBy` enum validates sort fields at compile time
  - `Priority` as `ValueEnum` enables automatic CLI parsing
- **Derive macros used:**
  - `#[derive(Parser)]` on `Cli` - top-level command parsing
  - `#[derive(Subcommand)]` on `Commands` - subcommand routing
  - `#[derive(Args)]` on `AddArgs` - grouped arguments for complex commands
  - `#[derive(ValueEnum)]` on enums - enables use as CLI values
- **Attribute annotations:**
  - `#[command(name, author, version, about, after_help)]` - metadata
  - `#[arg(long, short, value_enum, default_value_t)]` - argument configuration
  - `#[command(visible_alias)]` - command aliases
  - Doc comments (`///`) automatically become help text
- **Pattern matching:**
  - `Option<DueFilter>` enables `if let Some(filter)` pattern for optional filters
  - `match` on enums is exhaustive (compiler enforces handling all cases)
- **Automatic behaviors:**
  - `--help` and `-h` flags generated automatically
  - `--version` and `-V` flags generated automatically
  - Error handling with exit codes (0 for success, non-zero for errors)
  - Suggestions for typos in values (e.g., "hgh" suggests "high")
  - Color-coded help output in supported terminals
- **Integration points:**
  - `Cli::parse()` consumes `std::env::args()` automatically
  - `value_parser!(NaiveDate)` uses `FromStr` trait from chrono
  - Enums with `ValueEnum` get case-insensitive parsing by default

### Migration Notes

Upgrading from v1.5.0:

**Command syntax changes required:**

```bash
# Old (v1.5.0):
todo add "Task" --high --tag work
todo list --pending --high --overdue --sort

# New (v1.6.0):
todo add "Task" --priority high --tag work
todo list --status pending --priority high --due overdue --sort priority
```

**Complete syntax mapping:**

| Old Flag (v1.5.0) | New Argument (v1.6.0) |
|-------------------|----------------------|
| `--high` | `--priority high` |
| `--medium` | `--priority medium` |
| `--low` | `--priority low` |
| `--pending` | `--status pending` |
| `--done` | `--status done` |
| `--overdue` | `--due overdue` |
| `--due-soon` | `--due soon` |
| `--with-due` | `--due with-due` |
| `--without-due` | `--due no-due` |
| `--sort` (bool) | `--sort priority\|due\|created` |

**Data compatibility:**

- JSON file format (`todos.json`) remains fully compatible
- Priority values in JSON change from `"High"` to `"high"` (lowercase)
- Existing tasks automatically migrate on first load
- No manual data migration needed
- All task data, tags, and dates preserved

**New features available:**

- Use `todo --help` for comprehensive command help
- Use `todo <command> --help` for command-specific help
- Try command aliases: `todo a`, `todo ls`, `todo rm`
- Explore short flags: `-t` for tags, `-s` for sort

**Breaking changes to address:**

1. Update any scripts using the old flag syntax
2. Priority values in JSON will be lowercase after first save
3. Sorting now requires explicit field name (not just `--sort`)
4. Status filter now explicit (no implicit "all" without flag)

## [1.5.0] - 2026-02-03

### Added

- **Due date tracking** with `chrono` crate
- `due_date` field in `Task` struct (`Option<NaiveDate>`)
- `created_at` field in `Task` struct (`NaiveDate`) - automatic timestamp on task creation
- `--due YYYY-MM-DD` flag for `add` command to set task deadlines
- `--overdue` filter to list tasks past their due date
- `--due-soon` filter to list tasks due in the next 7 days
- `--with-due` filter to list tasks that have a due date
- `--without-due` filter to list tasks without a due date
- `--sort due` option to sort tasks by due date (earliest first)
- `--sort created` option to sort tasks by creation date (oldest first)
- `Task::is_overdue()` method to check if task is past due date
- `Task::is_due_soon(days: i64)` method to check if task is due within N days
- Date parsing with `NaiveDate::parse_from_str()` using format `%Y-%m-%d`
- Color-coded due date display:
  - Red + Bold: Overdue (e.g., "late 3 days")
  - Yellow + Bold: Due today (e.g., "due today")
  - Yellow: Due soon, 1-7 days (e.g., "in 5 days")
  - Cyan: Future, 8+ days (e.g., "in 30 days")
- **Tabular display format** for professional task listing
- Dynamic column width calculation based on content
- `display_task_tabular()` function for formatted output
- `calculate_column_widths()` function for optimal column sizing
- `get_due_text()` function for human-readable due date formatting
- `get_due_colored()` function for urgency-based coloring
- Header row with column labels: `ID`, `P` (Priority), `S` (Status), `Task`, `Tags`, `Due`
- Separator line for visual clarity
- String truncation with ellipsis for long task names and tags
- Date arithmetic using `chrono::Duration` for calculating days until due

### Changed

- **BREAKING CHANGE:** Priority display changed from emojis (🔴🟡🟢) to letters (H/M/L)
  - `Priority::emoji()` renamed to `Priority::letter()`
  - More professional and terminal-friendly appearance
  - Consistent column width in tabular format
- **BREAKING CHANGE:** Task display format completely redesigned
  - Old: `1. 🔴 ⏳ Study Rust [learning, programming]`
  - New: Tabular format with aligned columns

  ```
    ID  P  S  Task           Tags              Due
  ────────────────────────────────────────────────────
     1  H  ⏳  Study Rust     learning, prog... in 5 days
  ```

- `Task::new()` signature: now accepts `due_date: Option<NaiveDate>` parameter
- `Task` JSON format: includes `due_date` (nullable) and `created_at` (required)
- `--sort` flag: renamed from boolean to value-based (`priority`, `due`, `created`)
- Due date sorting: tasks with dates come before tasks without dates
- Completed tasks no longer display due date information
- Maximum column widths enforced (task: 40 chars, tags: 20 chars, due: 20 chars)
- Minimum column widths enforced (task: 10 chars, tags: 4 chars, due: 3 chars)

### Fixed

- Mutual exclusion validation for date filters (can't use multiple date filters together)
- Proper handling of `None` values in due date sorting
- Grammar in due date text ("1 day" vs "2 days")
- Visual hierarchy with color-coding guides attention to urgent items

### Technical Details

- `chrono` crate added with `serde` feature for automatic date serialization
- `NaiveDate` used for dates (no timezone information needed for due dates)
- Date creation: `Local::now().naive_local().date()` for current date
- Date comparison: direct comparison operators (`<`, `>`, `==`) work on `NaiveDate`
- Pattern matching on `Option<NaiveDate>` for flexible due date handling
- Format specifiers: `{:>3}` (right-align), `{:<40}` (left-align with width)
- String slicing for truncation: `&text[..width-3]` with "..." suffix
- Four-way pattern matching in due date sort for handling `Option` combinations
- Tabular format uses `print!()` for inline formatting and `println!()` for line breaks
- Date filters use `retain()` to preserve task indices

### Migration Notes

Upgrading from v1.4.0:

- Old tasks remain compatible - `due_date` defaults to `null`
- `created_at` will be set to current date on first load for existing tasks
- No data loss - all existing fields preserved
- New fields can be added manually to JSON if needed
- Priority indicators will display as letters instead of emojis

## [1.4.0] - 2026-01-31

### Added

- **Tags system** for task categorization
- `tags` field in `Task` struct (`Vec<String>`)
- `--tag <name>` flag for `add` command (can be used multiple times)
- `--tag <name>` filter for `list` command
- `--tag <name>` filter for `search` command
- `tags` command to list all tags with task counts
- Tag display in task output (colored cyan for pending, dimmed for completed)

### Changed

- `Task::new()` signature: now accepts `tags: Vec<String>` parameter
- `display_task()`: Added tag display after task text
- `list` command: Added tag filtering with `--tag` flag
- `search` command: Added tag filtering with `--tag` flag
- Task display format: shows tags as `[tag1, tag2]` after task text

### Fixed

- **Critical bug:** Task numbering now maintains original indices when filtering
  - Before: Filtered lists showed renumbered tasks (1, 2, 3...), causing `done`/`undone`/`remove` to operate on wrong tasks
  - After: Filtered lists show original task numbers, ensuring commands work correctly
  - Changed `display_lists()` to accept `Vec<(usize, &Task)>` with original indices
  - All filter operations now use `retain()` instead of `filter().collect()` to preserve indices

### Technical Details

- Tags are stored as `Vec<String>` in JSON
- Empty tag vectors serialize to `[]` in JSON
- Tag filtering is case-sensitive (matches exact tag names)
- `tags` command deduplicates and sorts tags alphabetically
- Original task numbering preserved through tuple `(usize, &Task)` pattern
- Serde automatically handles tags serialization/deserialization

## [1.3.0] - 2026-01-30

### Added

- JSON serialization using `serde` and `serde_json`
- Automatic serialization/deserialization with derive macros
- `#[derive(Serialize, Deserialize)]` on `Task` and `Priority`
- Pretty-printed JSON output with `to_string_pretty()`
- Automatic type validation and descriptive error messages
- Universal format support (JSON as standard)

### Changed

- **BREAKING CHANGE:** File format migrated from custom text (`todos.txt`) to JSON (`todos.json`)
- `load_tasks()`: Replaced manual parsing with `serde_json::from_str()` (12 lines → 3 lines)
- `save_tasks()`: Replaced manual formatting with `serde_json::to_string_pretty()` (4 lines → 2 lines)
- Storage file: `todos.txt` → `todos.json`
- `clear` command: Updated to delete `todos.json` instead of `todos.txt`

### Removed

- `Task::to_line()` method - replaced by automatic serialization (12 lines deleted)
- `Task::from_line()` method - replaced by automatic deserialization (25 lines deleted)
- All custom text parsing logic (37 lines total)

### Technical Details

- **91% code reduction** in I/O operations (53 lines → 5 lines)
- Serde generates 100+ lines of optimized serialization code automatically
- Format-agnostic design allows easy migration to TOML, YAML, or binary formats
- Extensibility: Adding new fields now requires only 1 line (struct field) instead of 30+ lines (parser updates)
- Better error messages: "missing field `priority` at line 4 column 3" vs generic parsing errors
- Git-friendly: JSON diffs clearly show what changed
- Tooling support: Can use `jq`, JSON validators, formatters, etc.

### Migration Notes

Users need to migrate from `todos.txt` to `todos.json`:

- Option 1: Start fresh (delete `todos.txt`, recreate tasks)
- Option 2: Manual migration (convert old format to JSON)
- Future: Migration script could be provided

## [1.2.0] - 2026-01-29

### Added

- Type-safe architecture with structs and enums
- `Priority` enum (High, Medium, Low) replacing string-based priorities
- `Task` struct encapsulating task data (text, completed, priority)
- `impl` blocks with methods: `new()`, `to_line()`, `from_line()`, `mark_done()`, `mark_undone()`
- Centralized I/O with `load_tasks()` and `save_tasks()` helper functions
- Derive macros: `Debug`, `Clone`, `PartialEq`, `Copy` for type safety

### Changed

- **BREAKING CHANGE:** Complete refactoring from string parsing to struct-based architecture
- All commands now use `Task` struct instead of raw string manipulation
- Parsing logic centralized in `Task::from_line()` method
- File I/O consolidated into two functions (36% code reduction)
- `add` command: uses `Task::new()` constructor
- `done`/`undone` commands: use `task.mark_done()`/`task.mark_undone()` methods
- `list` command: type-safe field access (`task.completed`, `task.priority`)
- Priority comparison: string matching → enum comparison
- Display logic: uses `Priority::emoji()` and `Priority::order()` methods

### Fixed

- Ownership issues with priority filters using `Copy` trait
- Clippy warnings: redundant closures replaced with function pointers
- Type safety: compiler now catches priority typos at compile time

### Technical Details

- Code metrics: ~180 lines → ~115 lines (36% reduction)
- Maintainability: Adding new fields now requires changes in only 3 places instead of 7+
- Extensibility: Easy to add timestamps, tags, subtasks in future versions
- Type safety: No more runtime errors from typos like `"hihg"`

## [1.1.0] - 2026-01-28

### Added

- `--medium` flag to filter tasks by medium priority
- Complete symmetry in priority filtering (high/medium/low)
- Dynamic titles for medium priority task combinations

### Changed

- Updated help messages to include `--medium` option
- Improved consistency between task creation and filtering

### Fixed

- Design flaw: users can now filter by medium priority, completing the priority filter set

## [1.0.1] - 2026-01-27

### Changed

- **BREAKING CHANGE:** Entire codebase translated to English
- All variable names Portuguese → English
- All function names Portuguese → English  
- All user-facing messages Portuguese → English
- All dynamic titles and error messages now in English
- Achieve full consistency with English documentation
- Updated function names: `extrair_prioridade` → `extract_priority`, etc.

## [1.0.0] - 2026-01-27

### Added

- `search <term>` command to search tasks by term
- `display_task()` function for atomic rendering
- `display_lists()` function for list rendering with statistics

### Changed

- Complete refactoring: separation of parsing vs rendering
- Better code reuse without duplication

### Fixed

- Correct numbering in search command

## [0.9.0] - 2026-01-27

### Added

- `--sort` flag to sort tasks by priority
- `priority_order()` function for high/medium/low mapping
- Optimized pipeline: filter → then sort

## [0.8.0] - 2026-01-26

### Added

- Priority system (high, medium, low)
- `--high` and `--low` flags to filter by priority
- Colored emojis (🔴 🟡 🟢) for visual indication
- `extract_priority()` function for parsing
- `priority_emoji()` function for rendering
- Filter combination (status + priority)
- Conflicting flags validation
- Dynamic titles based on context

## [0.7.0] - 2026-01-26

### Added

- `--pending` and `--done` flags to filter by status
- Filter combination support
- Helper functions for code reuse

## [0.6.0] - 2026-01-25

### Added

- Colorful visual interface using `colored` crate
- Visual hierarchy with dimmed/bold formatting
- Progress counter with percentage
- Color-coded priority indicators

## [0.5.0] - 2026-01-24

### Added

- `clear` command to remove all tasks
- File existence validation with `fs::metadata()`

## [0.4.2] - 2026-01-23

### Fixed

- State validation to prevent duplicate operations
- More specific error messages

## [0.4.1] - 2026-01-23

### Fixed

- Bug in `list` command with empty lines
- Robust line filtering with `trim()`

## [0.4.0] - 2026-01-23

### Added

- `undone` command to unmark tasks

## [0.3.0] - 2026-01-23

### Added

- `remove` command to delete specific tasks
- Index validation
- Comprehensive error handling

## [0.2.0] - 2026-01-23

### Added

- `done` command to mark tasks as completed
- String manipulation with `.replace()`, `.map()`, `.collect()`

## [0.1.0] - 2026-01-23

### Added

- `add` command to add tasks
- `list` command to list all tasks
- Basic file operations with `OpenOptions`
- Pattern matching for subcommands
- Error handling with `?` operator

[Unreleased]: https://github.com/joaofelipegalvao/todo-cli/compare/v1.7.0...HEAD
[1.7.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v1.6.0...v1.7.0
[1.6.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v1.5.0...v1.6.0
[1.5.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v1.4.0...v1.5.0
[1.4.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v1.3.0...v1.4.0
[1.3.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v1.0.1...v1.1.0
[1.0.1]: https://github.com/joaofelipegalvao/todo-cli/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.9.0...v1.0.0
[0.9.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.4.2...v0.5.0
[0.4.2]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/joaofelipegalvao/todo-cli/releases/tag/v0.1.0
