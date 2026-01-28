# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- Tags/categories (`#work`, `#home`)
- Edit command
- Due dates
- Sort by date (`--sort date`)
- JSON export
- Unit tests
- Refactoring with structs

### Changed

- **BREAKING CHANGE:**

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
- Colored emojis ( <img src="../todo-cli/assets/icons/circle-high.svg" width="12" /> <img src="../todo-cli/assets/icons/circle-medium.svg" width="12" /> <img src="../todo-cli/assets/icons/circle-low.svg" width="12" /> ) for visual indication
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

[Unreleased]: https://github.com/joaofelipegalvao/todo-cli/compare/v1.0.0...HEAD
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
