# Complete User Guide

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/joaofelipegalvao/todo-cli
cd todo-cli

# Build release version
cargo build --release

# Install globally (optional)
sudo cp target/release/todo-cli /usr/local/bin/todo
```

### Requirements

- Rust 1.70 or higher
- Cargo

## Commands Reference

### Adding Tasks

```bash
# Add task with default priority (medium)
todo add "Learn Rust"

# Add high priority task
todo add "Important meeting" --high

# Add low priority task
todo add "Organize desk" --low
```

**Notes:**

- Default priority is `medium` (<img src="../assets/icons/circle-medium.svg" width="12">)
- Tasks are stored in `todos.txt` in plain text format
- Format: `[ ] (priority) Task text`

### Listing Tasks

```bash
# List all tasks (creation order)
todo list

# Sort by priority (high → medium → low)
todo list --sort

# Filter by status
todo list --pending        # Only incomplete tasks
todo list --done          # Only completed tasks

# Filter by priority
todo list --high          # Only high priority
todo list --low           # Only low priority

# Combine filters
todo list --pending --high              # Pending high priority
todo list --done --low                  # Completed low priority
todo list --pending --high --sort       # Pending high priority, sorted
```

**Visual indicators:**

- <img src="../assets/icons/circle-high.svg" width="12" /> High priority (red)
- <img src="../assets/icons/circle-medium.svg" width="12" /> Medium priority (yellow)
- <img src="../assets/icons/circle-low.svg" width="12" /> Low priority (green)
- <img src="../assets/icons/checkbox.svg" width="16" /> Completed (green, strikethrough)
- <img src="../assets/icons/unchecked.svg" width="16" /> Pending (yellow)

### Searching Tasks

```bash
# Search for term in task text
todo search "rust"
todo search "meeting"
```

**Notes:**

- Case-insensitive search
- Maintains original task numbering
- Shows priority indicators
- Useful for large task lists

### Managing Tasks

```bash
# Mark task as completed
todo done 1

# Unmark task (mark as pending again)
todo undone 1

# Remove task permanently
todo remove 1

# Remove all tasks
todo clear
```

**Notes:**

- Task numbers are shown in `list` and `search` commands
- `done`/`undone` only change status, don't delete
- `remove` and `clear` are permanent (no undo)

## Examples

### Basic Workflow

```bash
# Start your day
todo add "Review pull requests" --high
todo add "Write documentation"
todo add "Team meeting at 3pm" --high
todo add "Refactor old code" --low

# Check what's urgent
todo list --pending --high --sort
# Output:
# 📋 Pending high priority tasks:
# 1. 🔴 ⏳ Review pull requests
# 3. 🔴 ⏳ Team meeting at 3pm

# Complete tasks
todo done 1
todo done 3

# See progress
todo list
# Shows completed tasks with ✅ and strikethrough
```

### Using Filters

```bash
# Focus on what matters
todo list --pending --high    # Today's priorities

# Review achievements
todo list --done              # What you've completed

# Clean up low priority items
todo list --low
todo remove 5
todo remove 7
```

### Search and Update

```bash
# Find tasks about a topic
todo search "documentation"
# Output shows task numbers

# Update status
todo done 2  # Mark documentation task as done
```

## Tips and Best Practices

### Priority Guidelines

**<img src="../assets/icons/circle-high.svg" width="12"> High Priority (--high):**

- Urgent and important
- Deadlines today
- Blocking other work
- Critical bugs

**<img src="../assets/icons/circle-medium.svg" width="12"> Medium Priority (default):**

- Important but not urgent
- This week's tasks
- Regular work items
- Most tasks should be here

**<img src="../assets/icons/circle-low.svg" width="12"> Low Priority (--low):**

- Nice to have
- No deadline
- Future improvements
- Can be postponed

### Workflow Suggestions

1. **Morning routine:**

   ```bash
   todo list --pending --high --sort
   ```

   Focus on urgent tasks first

2. **Quick capture:**

   ```bash
   todo add "Quick thought"
   ```

   Don't overthink priority for quick adds

3. **End of day:**

   ```bash
   todo list --done
   ```

   Review what you accomplished

4. **Weekly cleanup:**

   ```bash
   todo list --low
   todo clear  # Or selectively remove
   ```

### Combining Filters Effectively

```bash
# See only what needs attention
todo list --pending --high

# Review completed work by priority
todo list --done --high

# Organize by seeing everything sorted
todo list --sort

# Find specific tasks quickly
todo search "keyword"
```

## File Format

Tasks are stored in `todos.txt` as plain text:

```
[ ] (high) Important task
[ ] Regular task
[x] (low) Completed task
```

**Format breakdown:**

- `[ ]` = pending, `[x]` = completed
- `(high)` / `(low)` = priority (optional, default is medium)
- Text after priority marker = task description

**Notes:**

- File can be edited manually if needed
- Human-readable format
- One task per line
- Backup recommended before manual edits

## Troubleshooting

### Tasks not showing up

```bash
# Check if file exists
cat todos.txt

# Verify format (should have [ ] or [x])
```

### Wrong priority colors

- Ensure terminal supports colors
- Check `colored` crate is installed

### Can't find task number

```bash
# Use list to see all numbers
todo list

# Or search for the task
todo search "keyword"
```

## Development Usage

If running from source with Cargo:

```bash
# All commands work with cargo run --
cargo run -- add "Task"
cargo run -- list --pending
cargo run -- done 1
```

## Next Steps

- Check [LEARNING.md](LEARNING.md) to understand how it was built
- Read [CHANGELOG.md](../CHANGELOG.md) for version history
- Contribute improvements on GitHub
