# File Operations

**📚 Overview:**

This page covers file manipulation patterns used throughout the todo-cli project, from basic text file operations to JSON serialization.

**🔗 Related Versions:**

- [v0.1.0](../getting-started/v0.1.0-basic-cli.md) - Basic file writing
- [v0.3.0](../getting-started/v0.3.0-remove-command.md) - File reading/writing
- [v0.5.0](../getting-started/v0.5.0-clear-command.md) - File existence checks
- [v1.3.0](../advanced/v1.3.0-json-serialization.md) - JSON operations

---

## Basic File Writing

**Using `OpenOptions` for controlled file creation:**

```rust
use std::fs::OpenOptions;

"add" => {
    let task = &args[2];
    let mut file = OpenOptions::new()
        .create(true)    // Create if doesn't exist
        .append(true)    // Don't overwrite, add to end
        .open("todos.txt")?;
    
    writeln!(file, "[ ] {}", task)?;
}
```

**Why `OpenOptions` instead of `File::create()`?**

```rust
// ❌ This OVERWRITES the entire file!
let file = File::create("todos.txt")?;

// ✅ OpenOptions gives granular control
let file = OpenOptions::new()
    .create(true)    // Create if doesn't exist
    .append(true)    // Don't overwrite, add to end
    .open("todos.txt")?;
```

## File Reading

**Reading entire file content:**

```rust
use std::fs;

"done" => {
    let content = fs::read_to_string("todos.txt")?;
    
    let mut lines: Vec<String> = content
        .lines()
        .map(|l| l.to_string())
        .collect();
    
    // ... process lines ...
}
```

**What `fs::read_to_string()` does:**

- Opens file for reading
- Reads entire content into String
- Closes file automatically
- Returns Result for error handling

## File Overwriting

**Replacing entire file content:**

```rust
use std::fs;

"done" => {
    // ... modify lines in memory ...
    
    let new_content = lines.join("\n") + "\n";
    fs::write("todos.txt", new_content)?;
    
    println!("✓ Task marked as completed");
}
```

**When to use `fs::write()`:**

- Need to replace entire file
- Have modified content in memory
- File is small enough to fit in memory

## File Existence Checks

**Checking if file exists without opening:**

```rust
use std::fs;

"clear" => {
    if fs::metadata("todos.txt").is_ok() {
        fs::remove_file("todos.txt")?;
        println!("✓ All tasks have been removed");
    } else {
        println!("No tasks to remove");
    }
}
```

**Why use `fs::metadata()` instead of trying to delete?**

```rust
// ❌ Without check - shows error to user
fs::remove_file("todos.txt")?;
// If file doesn't exist → Error propagated to user
// User sees: "Error: No such file or directory"

// ✅ With check - friendly message
if fs::metadata("todos.txt").is_ok() {
    fs::remove_file("todos.txt")?;
    println!("✓ All tasks have been removed");
} else {
    println!("No tasks to remove");  // Not an error!
}
```

## File Deletion

**Removing files permanently:**

```rust
use std::fs;

"clear" => {
    if fs::metadata("todos.txt").is_ok() {
        fs::remove_file("todos.txt")?;
        println!("✓ All tasks have been removed");
    }
}
```

**Important considerations:**

- **Permanent** - no undo available
- **Returns Result** - can fail if no permissions
- **Use with caution** - always validate before deleting

## JSON File Operations

**Reading JSON with serde:**

```rust
use serde_json;

fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    match fs::read_to_string("todos.json") {
        Ok(content) => {
            let tasks: Vec<Task> = serde_json::from_str(&content)?;
            Ok(tasks)
        }
        Err(_) => Ok(Vec::new()),  // Missing file = empty list
    }
}
```

**Writing JSON with serde:**

```rust
fn save_tasks(tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(tasks)?;
    fs::write("todos.json", json)?;
    Ok(())
}
```

**Benefits of JSON format:**

- ✅ **Standard** - universal format
- ✅ **Automatic** - serde handles everything
- ✅ **Validated** - type checking and structure validation
- ✅ **Readable** - pretty format for debugging
- ✅ **Extensible** - add fields easily

## File Format Migration

**When changing file formats:**

```rust
// From custom text to JSON (v1.3.0)
fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    match fs::read_to_string("todos.json") {  // New file name
        Ok(content) => {
            let tasks: Vec<Task> = serde_json::from_str(&content)?;
            Ok(tasks)
        }
        Err(_) => Ok(Vec::new()),  // Graceful for missing file
    }
}
```

**Migration strategies:**

1. **Change file name** - `todos.txt` → `todos.json`
2. **Handle missing file** - start with empty list
3. **Backward compatibility** - use `#[serde(default)]` for new fields

## Error Handling for File Operations

**Common file errors and handling:**

```rust
fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    match fs::read_to_string("todos.json") {
        Ok(content) => {
            // Parse JSON
            match serde_json::from_str::<Vec<Task>>(&content) {
                Ok(tasks) => Ok(tasks),
                Err(e) => {
                    // File exists but JSON is invalid
                    Err(format!("Invalid JSON format: {}", e).into())
                }
            }
        }
        Err(e) => {
            // File doesn't exist or can't be read
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    // File doesn't exist - start with empty list
                    Ok(Vec::new())
                }
                _ => {
                    // Permission denied, etc.
                    Err(format!("Cannot read tasks file: {}", e).into())
                }
            }
        }
    }
}
```

## File Permissions and Security

**Considerations for file operations:**

```rust
// Check if file is readable
if fs::metadata("todos.json")?.permissions().readonly() {
    return Err("Tasks file is read-only".into());
}

// Safe file writing (atomic operation)
fn save_tasks_atomic(tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(tasks)?;
    
    // Write to temporary file first
    let temp_file = "todos.json.tmp";
    fs::write(temp_file, json)?;
    
    // Rename temp file to final file (atomic on most systems)
    fs::rename(temp_file, "todos.json")?;
    
    Ok(())
}
```

## Performance Considerations

**When file operations become expensive:**

```rust
// ❌ Bad: Read/write for every operation
"add" => {
    let tasks = load_tasks()?;  // Read all tasks
    tasks.push(new_task);        // Add one task
    save_tasks(&tasks)?;         // Write all tasks
}

// ✅ Good: Batch operations when possible
"add" => {
    // For single add, current approach is fine
    // For bulk operations, consider batching
}
```

**File size considerations:**

- **Small files (< 1MB)**: Load entire file into memory
- **Large files (> 10MB)**: Consider streaming or database
- **Our use case**: Hundreds of tasks = small file

## Best Practices Summary

1. **Use `OpenOptions`** for controlled file creation
2. **Handle missing files** gracefully with default values
3. **Validate before deleting** - check existence first
4. **Use standard formats** - JSON for structured data
5. **Consider atomic operations** for critical writes
6. **Handle permissions** - check if file is accessible
7. **Test file operations** - including error cases
8. **Use appropriate I/O patterns** - based on file size

---

**📚 See Also:**

- [Error Handling](error-handling.md) - File operation errors
- [JSON Serialization](../advanced/v1.3.0-json-serialization.md) - Serde patterns
- [Type Safety](type-safety.md) - Using types to prevent file format errors