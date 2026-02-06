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

# Add with specific priority
todo add "Important meeting" --priority high
todo add "Organize desk" --priority low

# Add task with tags (repeatable flag)
todo add "Study Rust" --tag programming --tag learning
todo add "Study Rust" -t programming -t learning  # short form

# Add task with due date
todo add "Submit report" --due 2026-02-15

# Combine all features
todo add "Fix critical bug" --priority high --tag work --tag urgent --due 2026-02-05
todo add "Project deadline" --priority high -t work -t client --due 2026-02-10

# Using alias
todo a "Quick task"  # 'a' is alias for 'add'
```

**Notes:**

- Default priority is `medium`
- Tasks are stored in `todos.json` in JSON format
- Multiple tags can be added with multiple `--tag` (or `-t`) flags
- Due dates use format `YYYY-MM-DD` (e.g., `2026-02-15`)
- Tasks automatically get a `created_at` timestamp
- Tags help categorize and filter tasks

**Priority values:**

- `high` - Urgent and important tasks
- `medium` - Default for most tasks (default)
- `low` - Nice to have, not urgent

**Due date format:**

- ✅ Valid: `2026-02-15`, `2026-12-31`, `2025-01-01`
- ❌ Invalid: `02/15/2026`, `15-02-2026`, `tomorrow`, `next week`

**Getting help:**

```bash
todo add --help
# Shows all available options with detailed descriptions
```

### Listing Tasks

```bash
# List all tasks (default: all statuses)
todo list
todo ls  # alias for 'list'

# Filter by status
todo list --status pending   # Only incomplete tasks
todo list --status done      # Only completed tasks
todo list --status all       # All tasks (default)

# Filter by priority
todo list --priority high    # Only high priority
todo list --priority medium  # Only medium priority
todo list --priority low     # Only low priority

# Filter by tag
todo list --tag work         # Only tasks with "work" tag
todo list --tag urgent       # Only tasks with "urgent" tag

# Filter by due date
todo list --due overdue      # Tasks past their due date
todo list --due soon         # Tasks due in next 7 days
todo list --due with-due     # Tasks that have a due date
todo list --due no-due       # Tasks without a due date

# Sort results
todo list --sort priority    # Sort by priority (High → Medium → Low)
todo list --sort due         # Sort by due date (earliest first)
todo list --sort created     # Sort by creation date (oldest first)
todo list -s priority        # short form

# Combine filters
todo list --status pending --priority high
todo list --status pending --tag work --sort due
todo list --priority high --due overdue
todo list --due soon --tag urgent --sort due
todo list --status pending --priority high --tag work --sort due
```

**Visual indicators:**

**Priority:**

- H (red) - High priority
- M (yellow) - Medium priority
- L (green) - Low priority

**Status:**

- ✅ Completed (green, strikethrough)
- ⏳ Pending (white)

**Due dates (color-coded by urgency):**

- 🚨 **Red + Bold**: Overdue (e.g., "late 3 days")
- ⚠️ **Yellow + Bold**: Due today (e.g., "due today")
- 📅 **Yellow**: Due soon (1-7 days, e.g., "in 5 days")
- 🗓️ **Cyan**: Future (>7 days, e.g., "in 30 days")

**Example output:**

```
Tasks:

  ID  P  S  Task                              Tags              Due
──────────────────────────────────────────────────────────────────────
   1  H  ⏳  Submit quarterly report           work, urgent      in 2 days
   2  M  ⏳  Team standup meeting              work, meetings    due today
   3  M  ⏳  Review pull request #42           work, code        late 4 days
   4  L  ⏳  Plan summer vacation              personal          in 40 days
   5  M  ✅  Fix login bug                     work, frontend
   6  H  ⏳  Client presentation               work, client      in 1 day
```

**Important notes:**

- Task numbers preserve original numbering in filtered views
- Due dates are hidden for completed tasks
- Column widths adjust dynamically to content
- Only one due date filter can be used at a time

**Getting help:**

```bash
todo list --help
# Shows all filtering and sorting options
```

### Searching Tasks

```bash
# Search for term in task text
todo search "rust"
todo search "meeting"

# Search with tag filter
todo search "bug" --tag work
todo search "docs" --tag programming
```

**Notes:**

- Case-insensitive search
- Maintains original task numbering
- Shows priority indicators, tags, and due dates
- Tag filter narrows search results
- Useful for large task lists

### Managing Tags

```bash
# List all tags with counts
todo tags

# Example output:
# 📋 Tags:
#   learning (2 tasks)
#   programming (3 tasks)
#   urgent (1 task)
#   work (4 tasks)
```

**Notes:**

- Shows all unique tags across tasks
- Displays task count for each tag
- Sorted alphabetically
- Helps discover categorization patterns

### Managing Tasks

```bash
# Mark task as completed
todo done 1

# Unmark task (mark as pending again)
todo undone 1

# Remove task permanently
todo remove 1
todo rm 1      # alias for 'remove'
todo delete 1  # also an alias for 'remove'

# Remove all tasks
todo clear
```

**Notes:**

- Task numbers are shown in `list` and `search` commands
- Numbers remain consistent even in filtered views
- `done`/`undone` only change status, don't delete
- `remove` and `clear` are permanent (no undo)
- Completed tasks don't show due date information

## Examples

### Basic Workflow

```bash
# Start your day
todo add "Review pull requests" --priority high --tag work --due 2026-02-03
todo add "Write documentation" --tag work --tag documentation --due 2026-02-05
todo add "Team meeting at 3pm" --priority high --tag work --due 2026-02-03
todo add "Refactor old code" --priority low --tag programming

# Check what's urgent
todo list --status pending --priority high --sort due
# Output:
#   ID  P  S  Task                      Tags  Due
# ────────────────────────────────────────────────
#    1  H  ⏳  Review pull requests      work  due today
#    3  H  ⏳  Team meeting at 3pm       work  due today

# See what's overdue
todo list --due overdue
# Shows tasks past their due date in red

# See what's coming up
todo list --due soon
# Shows tasks due in the next 7 days

# Complete tasks
todo done 1
todo done 3

# See progress
todo list
# Shows completed tasks with ✅ and no due date
```

### Working with Due Dates

```bash
# Add tasks with various due dates
todo add "Submit tax documents" --priority high --due 2026-04-15
todo add "Dentist appointment" --due 2026-02-10
todo add "Birthday gift for mom" --due 2026-03-05

# See everything sorted by deadline
todo list --sort due
# Output shows tasks in order: overdue → today → soon → future

# Focus on immediate deadlines
todo list --due soon
# Shows only tasks due in next 7 days

# Find tasks that are late
todo list --due overdue
# Shows tasks with due dates in the past (in red)

# Find tasks without deadlines
todo list --due no-due
# Shows tasks you can schedule later

# Find tasks with deadlines
todo list --due with-due
# Shows all tasks that have a due date set
```

### Using Filters

```bash
# Focus on what matters
todo list --status pending --priority high              # Today's priorities
todo list --status pending --priority high --sort due   # Today's priorities by deadline

# Focus on work tasks
todo list --tag work                                # All work-related tasks
todo list --status pending --tag work --sort due    # Pending work tasks by deadline

# Review achievements
todo list --status done                        # What you've completed
todo list --status done --tag programming      # Completed programming tasks

# Clean up low priority items
todo list --priority low
todo remove 5
todo remove 7

# Critical: high priority + overdue
todo list --priority high --due overdue
# Shows urgent tasks you're behind on

# Planning: medium priority + no due date
todo list --priority medium --due no-due
# Shows tasks you need to schedule
```

### Search and Update

```bash
# Find tasks about a topic
todo search "documentation"
# Output shows task numbers with original numbering

# Find work-related bugs
todo search "bug" --tag work
# Output:
# 📋 Results for "bug":
#   ID  P  S  Task              Tags          Due
# ───────────────────────────────────────────────
#    5  M  ⏳  Fix login bug     work, urgent  in 2 days

# Update status (using original number)
todo done 5  # Marks the correct task even in filtered view
```

### Organizing with Tags

```bash
# Create tasks with meaningful tags
todo add "Learn Rust macros" --tag learning --tag rust --due 2026-02-20
todo add "Team standup" --tag work --tag meetings --due 2026-02-03
todo add "Fix navbar bug" --priority high --tag work --tag frontend --due 2026-02-05

# See all your tags
todo tags
# Output:
# 📋 Tags:
#   frontend (1 task)
#   learning (1 task)
#   meetings (1 task)
#   rust (1 task)
#   work (2 tasks)

# Focus on specific areas
todo list --tag learning                       # All learning tasks
todo list --tag work --status pending          # Pending work tasks
todo list --tag frontend --priority high       # High priority frontend work
todo list --tag work --due soon                # Work tasks due soon
```

### Advanced Deadline Management

```bash
# Morning routine: what needs attention today?
todo list --due overdue           # What's already late
todo list --due soon              # What's coming up this week
todo list --priority high --sort due   # High priority sorted by deadline

# Weekly planning
todo list --due no-due            # Tasks that need scheduling
todo list --sort due              # See entire timeline
todo list --tag project-x --sort due  # Project timeline

# End of day review
todo list --status done           # What you accomplished
todo list --due overdue           # What still needs attention

# Project deadline tracking
todo add "Design phase complete" --tag project-x --due 2026-02-10
todo add "Development complete" --tag project-x --due 2026-02-20
todo add "Testing complete" --tag project-x --due 2026-02-25
todo add "Deployment" --priority high --tag project-x --due 2026-03-01

# See project timeline
todo list --tag project-x --sort due
```

## Tips and Best Practices

### Priority Guidelines

**high - High Priority (--priority high):**

- Urgent and important
- Deadlines today or tomorrow
- Blocking other work
- Critical bugs
- Client deliverables

**medium - Medium Priority (default):**

- Important but not urgent
- This week's tasks
- Regular work items
- Most tasks should be here

**low - Low Priority (--priority low):**

- Nice to have
- No deadline
- Future improvements
- Can be postponed

### Due Date Best Practices

**When to set due dates:**

- ✅ Hard deadlines (client deliverables, appointments)
- ✅ Time-sensitive tasks (event preparation)
- ✅ Personal goals with commitment (finish book by X)
- ❌ Flexible tasks (general learning, ideas)
- ❌ Ongoing work (code refactoring)

**Tips:**

- Use `--due soon` daily to stay on top of upcoming deadlines
- Check `--due overdue` regularly to catch slipping tasks
- Combine due dates with high priority for critical deadlines
- Use `--due no-due` to find tasks that need scheduling
- Sort by due date (`--sort due`) for timeline view
- Don't over-schedule: leave some tasks flexible

**Due date workflow:**

1. **Daily check:**

   ```bash
   todo list --due overdue        # Fix what's late
   todo list --due soon           # Prepare for upcoming
   ```

2. **Weekly planning:**

   ```bash
   todo list --due no-due         # Schedule flexible tasks
   todo list --sort due           # Review timeline
   ```

3. **Project tracking:**

   ```bash
   todo list --tag project --sort due  # See project timeline
   ```

### Tag Best Practices

**Recommended tag categories:**

1. **Context tags:** `work`, `personal`, `home`
2. **Project tags:** `project-name`, `client-name`
3. **Activity tags:** `coding`, `documentation`, `meetings`
4. **Status tags:** `urgent`, `blocked`, `waiting`
5. **Technology tags:** `rust`, `python`, `frontend`, `backend`

**Tips:**

- Use lowercase for consistency
- Keep tag names short and clear
- Don't over-tag (2-3 tags per task is usually enough)
- Use `todo tags` regularly to see your categorization
- Create tag conventions (e.g., `proj-` prefix for projects)
- Combine tags with due dates for powerful filtering

### Workflow Suggestions

1. **Morning routine:**

   ```bash
   todo list --due overdue                           # What's late
   todo list --due soon                              # What's coming up
   todo list --status pending --priority high --sort due  # Today's priorities
   todo list --tag work --status pending --sort due  # Work focus
   ```

2. **Quick capture:**

   ```bash
   todo add "Quick thought"
   todo add "Research topic" --tag learning
   todo add "Call dentist" --due 2026-02-10
   ```

   Don't overthink priority for quick adds

3. **End of day:**

   ```bash
   todo list --status done              # Review accomplishments
   todo list --status done --tag work   # Work achievements
   todo list --due overdue              # What needs attention
   ```

4. **Weekly review:**

   ```bash
   todo tags                            # See all categories
   todo list --priority low             # Review low priority items
   todo list --due no-due               # Schedule flexible tasks
   todo list --tag project-x --sort due # Check project timeline
   ```

5. **Weekly cleanup:**

   ```bash
   todo list --status done --tag learning  # Review completed learning
   todo list --priority low                # Clean up low priority
   ```

### Combining Filters Effectively

```bash
# Critical work
todo list --priority high --due overdue --tag work
# High priority work that's already late

# This week's focus
todo list --status pending --due soon --sort due
# What needs attention soon

# Project deadlines
todo list --tag project-x --due with-due --sort due
# Project tasks with deadlines, in timeline order

# Flexible work
todo list --tag work --due no-due
# Work tasks you can schedule when you have time

# What to do next
todo list --status pending --priority high --sort due
# High priority tasks in deadline order

# Tag + deadline combination
todo list --tag frontend --due soon
# Frontend work due soon

# Review by category
todo list --status done --tag learning
# Completed learning tasks
```

### Sorting Strategies

**Priority sort (`--sort priority`):**

- Best for: Daily task selection
- Shows: High → Medium → Low
- Use when: Choosing what to work on next

**Due date sort (`--sort due`):**

- Best for: Timeline planning
- Shows: Overdue → Today → Soon → Future → No due date
- Use when: Managing deadlines and schedules

**Created sort (`--sort created`):**

- Best for: Seeing task history
- Shows: Oldest → Newest
- Use when: Finding long-standing tasks

### Command Aliases

Save time with built-in aliases:

```bash
# Add command
todo a "Task"              # Same as: todo add "Task"

# List command
todo ls                    # Same as: todo list
todo ls --status pending   # Same as: todo list --status pending

# Remove command
todo rm 3                  # Same as: todo remove 3
todo delete 3              # Same as: todo remove 3
```

## File Format

Tasks are stored in `todos.json` as JSON:

```json
[
  {
    "text": "Study Rust",
    "completed": false,
    "priority": "high",
    "tags": ["programming", "learning"],
    "due_date": "2026-02-15",
    "created_at": "2026-02-03"
  },
  {
    "text": "Fix bug",
    "completed": true,
    "priority": "medium",
    "tags": ["work", "urgent"],
    "due_date": "2026-02-01",
    "created_at": "2026-01-28"
  },
  {
    "text": "Buy coffee",
    "completed": false,
    "priority": "low",
    "tags": [],
    "due_date": null,
    "created_at": "2026-02-03"
  }
]
```

**Format breakdown:**

- `text`: Task description (string)
- `completed`: Status (boolean - `false` = pending, `true` = completed)
- `priority`: Priority level (string - "high", "medium", or "low" in lowercase)
- `tags`: List of tags (array of strings, can be empty)
- `due_date`: Due date in YYYY-MM-DD format (string or `null`)
- `created_at`: Creation date in YYYY-MM-DD format (string, always present)

**Notes:**

- JSON format enables automatic serialization with `serde`
- `due_date` can be `null` (no deadline)
- `created_at` is set automatically when task is added
- Priority values are stored in lowercase in JSON
- File can be edited manually if needed, but be careful with syntax
- Backup recommended before manual edits
- Invalid JSON will cause the app to fail to load tasks

## Troubleshooting

### Tasks not showing up

```bash
# Check if file exists
cat todos.json

# Verify JSON format (should be valid JSON array)
# Use a JSON validator if needed
```

### Invalid JSON error

If you get a JSON parsing error:

1. Check `todos.json` for syntax errors
2. Ensure proper JSON format (commas, quotes, brackets)
3. Verify date format is `YYYY-MM-DD` or `null`
4. Verify priority is lowercase: "high", "medium", or "low"
5. Restore from backup if available
6. Use `todo clear` to start fresh (deletes all tasks)

### Invalid date format error

If you get a date parsing error:

```bash
# Wrong formats:
todo add "Task" --due 02/15/2026  # ❌
todo add "Task" --due 15-02-2026  # ❌
todo add "Task" --due tomorrow    # ❌

# Correct format:
todo add "Task" --due 2026-02-15  # ✅

# Format: YYYY-MM-DD (year-month-day)
```

### Invalid priority value error

```bash
# Wrong values:
todo add "Task" --priority urgent  # ❌
todo add "Task" --priority HIGH    # ❌

# Correct values:
todo add "Task" --priority high    # ✅
todo add "Task" --priority medium  # ✅
todo add "Task" --priority low     # ✅

# Possible values shown in help:
todo add --help
```

### Wrong priority colors

- Ensure terminal supports colors
- Check `colored` crate is installed
- Try a different terminal emulator

### Can't find task number

```bash
# Use list to see all numbers
todo list

# Or search for the task
todo search "keyword"

# Remember: filtered views preserve original numbers
todo list --tag work  # Numbers won't be 1, 2, 3... if tasks are filtered
```

### Tags not showing

```bash
# Verify task has tags in JSON
cat todos.json

# Tags are case-sensitive when filtering
todo list --tag Work   # Won't match "work"
todo list --tag work   # Will match "work"
```

### Date filters not working

```bash
# Can't use multiple due date values:
todo list --due overdue --due soon  # ❌ Error: can't use argument multiple times

# Use one at a time:
todo list --due overdue             # ✅
todo list --due soon                # ✅

# But you can combine due filter with other filters:
todo list --due overdue --priority high      # ✅
todo list --due soon --tag work              # ✅
todo list --status pending --due soon        # ✅
```

### Due date colors not showing

- Ensure terminal supports ANSI colors
- Colors are based on urgency:
  - Red + Bold = overdue
  - Yellow + Bold = due today
  - Yellow = due in 1-7 days
  - Cyan = due in 8+ days
- Completed tasks don't show due dates

### Getting help for any command

Every command has built-in help:

```bash
todo --help           # Main help
todo add --help       # Help for add command
todo list --help      # Help for list command
todo search --help    # Help for search command

# Shows:
# - Available options
# - Possible values for enums
# - Default values
# - Short and long forms
# - Detailed descriptions
```

## Development Usage

If running from source with Cargo:

```bash
# All commands work with cargo run --
cargo run -- add "Task"
cargo run -- add "Task" --tag work --due 2026-02-15
cargo run -- list --status pending
cargo run -- list --due overdue
cargo run -- list --sort due
cargo run -- list --tag work
cargo run -- tags
cargo run -- done 1
```

## Advanced Usage

### Batch Operations

```bash
# Add multiple related tasks with deadlines
todo add "Setup project" --priority high --tag project-x --due 2026-02-10
todo add "Design database" --tag project-x --tag backend --due 2026-02-12
todo add "Create API endpoints" --tag project-x --tag backend --due 2026-02-18
todo add "Build frontend" --tag project-x --tag frontend --due 2026-02-20
todo add "Testing" --tag project-x --due 2026-02-25
todo add "Deployment" --priority high --tag project-x --due 2026-03-01

# Review project timeline
todo list --tag project-x --sort due

# Focus on what's next
todo list --tag project-x --status pending --sort due
```

### Context Switching

```bash
# Switch to work mode
alias work="todo list --tag work --status pending --sort due"
work

# Switch to learning mode
alias learn="todo list --tag learning --status pending"
learn

# Review personal tasks
alias personal="todo list --tag personal --sort due"
personal

# Check what's urgent across all contexts
alias urgent="todo list --due overdue"
urgent

# See this week's deadlines
alias thisweek="todo list --due soon --sort due"
thisweek
```

### Time-based Workflows

```bash
# Sprint planning (2-week cycle)
todo add "Feature A" --priority high --tag sprint-5 --due 2026-02-15
todo add "Feature B" --tag sprint-5 --due 2026-02-15
todo add "Bug fixes" --tag sprint-5 --due 2026-02-12

# Track sprint progress
todo list --tag sprint-5 --sort due
todo list --tag sprint-5 --status pending

# Daily standup prep
todo list --status done --tag sprint-5              # What I did
todo list --status pending --tag sprint-5 --sort due  # What I'm doing
todo list --tag sprint-5 --due overdue              # Blockers/issues
```

### Recurring Tasks Simulation

```bash
# Add weekly tasks with due dates
todo add "Weekly report" --tag work --due 2026-02-07
todo add "Team meeting prep" --tag work --due 2026-02-06

# When done, re-add for next week
todo done 1
todo add "Weekly report" --tag work --due 2026-02-14

# Monthly tasks
todo add "Monthly review" --tag personal --due 2026-03-01
todo add "Expense report" --tag work --due 2026-03-05
```

### Migration from Previous Versions

**From v1.5.0 (manual parsing):**

The command syntax has changed significantly in v1.6.0:

```bash
# Old syntax (v1.5.0):
todo add "Task" --high --tag work --tag urgent
todo list --pending --high --overdue

# New syntax (v1.6.0):
todo add "Task" --priority high --tag work --tag urgent
todo list --status pending --priority high --due overdue
```

**Key changes:**

- `--high/--medium/--low` → `--priority high/medium/low`
- `--pending/--done` → `--status pending/done`
- `--overdue/--due-soon/--with-due/--without-due` → `--due overdue/soon/with-due/no-due`
- Sorting: `--sort` → `--sort priority/due/created`

**Data compatibility:**

- Your `todos.json` file remains fully compatible
- No data migration needed
- All existing tasks, tags, and dates work as-is

**From v1.4.0 (without due dates):**

1. Backup your `todos.json` file
2. The new version adds `due_date` (optional) and `created_at` (required)
3. Old tasks are compatible - they'll get `due_date: null`
4. `created_at` will be set to current date on first load
5. No data loss - all tasks, tags, and priorities preserved

## Next Steps

- Check [LEARNING.md](LEARNING.md) to understand how it was built
- Read [CHANGELOG.md](../CHANGELOG.md) for version history
- Explore the new Clap-powered CLI features
- Use auto-generated help: `todo --help` and `todo <command> --help`
- Try command aliases for faster workflow
- Contribute improvements on GitHub
- Share your productivity workflows!

## Quick Reference Card

```bash
# Essential commands
todo add "Task" [--priority high|medium|low] [--tag TAG]... [--due YYYY-MM-DD]
todo list [--status pending|done|all] [--priority high|medium|low]
todo list [--due overdue|soon|with-due|no-due] [--tag TAG]
todo list [--sort priority|due|created]
todo done NUMBER
todo search "keyword" [--tag TAG]

# Aliases
todo a "Task"              # add
todo ls                    # list
todo rm NUMBER             # remove

# Powerful combinations
todo list --status pending --priority high --sort due     # Today's priorities
todo list --due overdue --tag work                        # Late work tasks
todo list --due soon --sort due                           # This week's deadlines
todo list --tag project --sort due                        # Project timeline
todo list --due no-due --priority medium                  # Tasks to schedule

# Daily routine
todo list --due overdue                           # What's late
todo list --due soon                              # What's coming
todo list --status pending --priority high --sort due  # What to do now
todo list --status done                           # What you accomplished

# Get help
todo --help                # Main help
todo add --help            # Command-specific help
```
