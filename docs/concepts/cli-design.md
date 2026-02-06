# CLI Design Patterns

**📚 Overview:**

This page covers command-line interface design patterns used throughout the todo-cli project, from basic argument parsing to professional CLI frameworks.

**🔗 Related Versions:**

- [v0.1.0](../getting-started/v0.1.0-basic-cli.md) - Basic subcommands
- [v0.7.0](../intermediate/v0.7.0-advanced-filters.md) - Flag parsing
- [v1.4.0](../advanced/v1.4.0-tags-system.md) - Multiple flags
- [v1.6.0](../advanced/v1.6.0-professional-cli-clap.md) - Clap framework

---

## Basic Subcommand Pattern

**Simple command matching:**

```rust
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        return Err("Usage: todo <command>".into());
    }
    
    match args[1].as_str() {
        "add" => handle_add(&args),
        "list" => handle_list(&args),
        "done" => handle_done(&args),
        _ => return Err("Unknown command".into()),
    }
}
```

**Characteristics:**

- ✅ Simple to understand
- ✅ No dependencies
- ❌ Manual parsing required
- ❌ No auto-generated help

## Flag Parsing Patterns

### Single Value Flags

**Flags with values:**

```rust
"add" => {
    let mut priority = Priority::Medium;
    let mut i = 3;  // Start after command and task text
    
    while i < args.len() {
        match args[i].as_str() {
            "--priority" => {
                if i + 1 >= args.len() {
                    return Err("--priority requires a value".into());
                }
                priority = match args[i + 1].as_str() {
                    "high" => Priority::High,
                    "medium" => Priority::Medium,
                    "low" => Priority::Low,
                    _ => return Err("Invalid priority".into()),
                };
                i += 1;  // Skip the value
            }
            _ => return Err(format!("Unknown flag: {}", args[i]).into()),
        }
        i += 1;
    }
}
```

### Multiple Value Flags

**Repeatable flags:**

```rust
"add" => {
    let mut tags: Vec<String> = Vec::new();
    let mut i = 3;
    
    while i < args.len() {
        match args[i].as_str() {
            "--tag" => {
                if i + 1 >= args.len() {
                    return Err("--tag requires a value".into());
                }
                tags.push(args[i + 1].clone());
                i += 1;  // Skip the tag value
            }
            _ => { /* ... */ }
        }
        i += 1;
    }
}
```

**Usage:**

```bash
todo add "Task" --tag work --tag urgent --tag backend
```

## Mutual Exclusion Patterns

### Boolean Flags with Manual Validation

**Preventing conflicting flags:**

```rust
"list" => {
    let mut status_filter = "all";
    let mut priority_filter: Option<Priority> = None;
    let mut sort = false;
    
    // Parse flags...
    
    // Manual conflict checking
    if status_filter != "all" && priority_filter.is_some() {
        return Err("Can't combine status and priority filters".into());
    }
}
```

### Enum-Based Mutual Exclusion

**Type-safe exclusive options:**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum StatusFilter {
    Pending,
    Done,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum DueFilter {
    Overdue,
    Soon,
    WithDue,
    NoDue,
}

// Usage in command
List {
    #[arg(long, value_enum, default_value_t = StatusFilter::All)]
    status: StatusFilter,
    
    #[arg(long, value_enum)]
    due: Option<DueFilter>,  // None = no filter, Some = exclusive filter
}
```

**Benefits:**

- ✅ **Compile-time safety** - impossible to have conflicts
- ✅ **Type safety** - no string matching
- ✅ **Auto-validation** - clap handles it
- ✅ **Better help** - enum values shown in help

## Command Aliases

**Multiple names for same command:**

```rust
// With clap
#[command(visible_alias = "a")]
Add(AddArgs),

#[command(visible_alias = "ls")]
List { /* ... */ },

#[command(visible_aliases = ["rm", "delete"])]
Remove { id: usize },
```

**Usage:**

```bash
# All equivalent:
todo add "Task"
todo a "Task"

# All equivalent:
todo list --status pending
todo ls --status pending

# All equivalent:
todo remove 3
todo rm 3
todo delete 3
```

## Help Generation Patterns

### Manual Help

**Writing help by hand:**

```rust
fn print_help() {
    println!("Usage: todo <command> [options]");
    println!("");
    println!("Commands:");
    println!("  add     Add a new task");
    println!("  list    List tasks");
    println!("  done    Mark task as completed");
    println!("");
    println!("Examples:");
    println!("  todo add \"Task\" --priority high");
}
```

**Problems:**

- ❌ Must update manually
- ❌ Easy to forget to update
- ❌ No command-specific help

### Auto-Generated Help

**With clap derive macros:**

```rust
#[derive(Parser)]
#[command(about = "A modern task manager", long_about = None)]
#[command(after_help = "EXAMPLES:
    todo add \"Task\" --priority high
    todo list --status pending
")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add a new task")]
    Add {
        #[arg(about = "Task description")]
        text: String,
        
        #[arg(long, about = "Task priority")]
        priority: Priority,
    },
}
```

**Benefits:**

- ✅ **Automatic** - no manual help writing
- ✅ **Consistent** - same style everywhere
- ✅ **Complete** - includes all options
- ✅ **Per-command** - `todo add --help` works

## Error Message Patterns

### Generic Errors

**Simple but unhelpful:**

```rust
return Err("Invalid input".into());
```

### Specific Errors

**Better - tells user what's wrong:**

```rust
return Err("Invalid task number. Use 1-5".into());
```

### Contextual Errors

**Best - shows the actual problem:**

```rust
return Err(format!("Task {} doesn't exist. Available: 1-5", number).into());
```

### Professional CLI Errors

**With clap - automatic context:**

```rust
#[arg(value_parser = clap::value_parser!(NaiveDate))]
due: Option<NaiveDate>,
```

**Result:**

```bash
$ todo add "Task" --due 2026-13-50
error: invalid value '2026-13-50' for '--due <DATE>': input is out of range
```

## Input Validation Patterns

### Argument Count Validation

**Check minimum required arguments:**

```rust
"add" => {
    if args.len() < 3 {
        return Err("Usage: todo add <task>".into());
    }
    let task = &args[2];
    // ...
}
```

### Type Validation

**Parse with error handling:**

```rust
"done" => {
    if args.len() < 3 {
        return Err("Usage: todo done <number>".into());
    }
    
    let number: usize = match args[2].parse() {
        Ok(n) => n,
        Err(_) => return Err("Task number must be a positive integer".into()),
    };
    // ...
}
```

### Range Validation

**Check if number is within valid range:**

```rust
if number == 0 || number > tasks.len() {
    return Err(format!("Invalid task number {}. Available: 1-{}", 
                     number, tasks.len()).into());
}
```

## User Experience Patterns

### Immediate Feedback

**Confirm successful operations:**

```rust
"add" => {
    // ... add task ...
    println!("{}", "✓ Task added".green());
}

"done" => {
    // ... mark task done ...
    println!("{}", "✓ Task marked as completed".green());
}
```

### Semantic Colors

**Use colors to convey meaning:**

```rust
// Success (positive actions)
println!("{}", "✓ Task added".green());
println!("{}", "✓ Task completed".green());

// Neutral (reversible actions)
println!("{}", "✓ Task unmarked".yellow());

// Destructive (permanent actions)
println!("{}", "✓ Task removed".red());
println!("{}", "✓ All tasks cleared".red().bold());
```

### Progress Indicators

**Show progress for operations:**

```rust
"list" => {
    let tasks = load_tasks()?;
    
    if tasks.is_empty() {
        println!("No tasks");
        return Ok(());
    }
    
    let completed = tasks.iter().filter(|t| t.completed).count();
    let total = tasks.len();
    let percentage = (completed as f32 / total as f32 * 100.0) as u32;
    
    // Display tasks...
    
    println!("\n{} of {} completed ({}%)", completed, total, percentage);
    
    if percentage == 100 {
        println!("{}", "🎉 All tasks completed!".green().bold());
    }
}
```

## Command Discovery Patterns

### Help Command

**Show available commands:**

```rust
"help" => {
    println!("Available commands:");
    println!("  add     - Add a new task");
    println!("  list    - List tasks");
    println!("  done    - Mark task as completed");
    println!("  remove  - Remove a task");
    println!("  clear   - Remove all tasks");
    println!("  help    - Show this help");
}
```

### List Command

**Discovery command for data:**

```rust
"tags" => {
    let tasks = load_tasks()?;
    
    // Collect all unique tags
    let mut all_tags: Vec<String> = Vec::new();
    for task in &tasks {
        for tag in &task.tags {
            if !all_tags.contains(tag) {
                all_tags.push(tag.clone());
            }
        }
    }
    
    if all_tags.is_empty() {
        println!("No tags found");
    } else {
        println!("Available tags:");
        for tag in &all_tags {
            let count = tasks.iter().filter(|t| t.tags.contains(tag)).count();
            println!("  {} ({} task{})", tag, count, 
                    if count == 1 { "" } else { "s" });
        }
    }
}
```

## Best Practices Summary

1. **Use frameworks** - clap for professional CLIs
2. **Validate early** - fail fast with clear messages
3. **Provide feedback** - confirm successful operations
4. **Use semantic colors** - convey meaning visually
5. **Handle errors gracefully** - helpful error messages
6. **Support discovery** - help and list commands
7. **Use aliases** - improve usability
8. **Auto-generate help** - keep documentation in sync

---

**📚 See Also:**

- [Error Handling](error-handling.md) - CLI error patterns
- [Type Safety](type-safety.md) - Using types to prevent CLI errors
- [Professional CLI](../advanced/v1.6.0-professional-cli-clap.md) - Complete clap implementation