# Advanced Error Handling

**📚 Overview:**

This page covers advanced error handling patterns in Rust, focusing on the `anyhow` and `thiserror` crates for professional error management.

**🔗 Related Versions:**

- [v0.1.0](../getting-started/v0.1.0-basic-cli.md) - Basic `?` operator
- [v0.3.0](../getting-started/v0.3.0-remove-command.md) - Input validation
- [v0.4.2](../getting-started/v0.4.2-state-validations.md) - Precondition validation
- [v1.7.0](../advanced/v1.7.0-professional-error-handling.md) - **Professional error handling**

---

## The Error Handling Spectrum

### Level 1: Basic Error Propagation

**Using `Box<dyn Error>` for generic errors:**

```rust
fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    let content = fs::read_to_string("todos.json")?;
    let tasks: Vec<Task> = serde_json::from_str(&content)?;
    Ok(tasks)
}
```

**Characteristics:**

- ✅ Simple to implement
- ✅ Works with any error type
- ❌ No context about what failed
- ❌ Generic error messages
- ❌ Can't distinguish error types

### Level 2: Application Errors with `anyhow`

**Adding context to infrastructure errors:**

```rust
use anyhow::{Context, Result};

fn load_tasks() -> Result<Vec<Task>> {
    let content = fs::read_to_string("todos.json")
        .context("Failed to read todos.json")?;
    
    serde_json::from_str(&content)
        .context("Failed to parse todos.json - file may be corrupted")
}
```

**Characteristics:**

- ✅ Rich context at each layer
- ✅ Error chains show root causes
- ✅ Works with any error type
- ✅ Minimal boilerplate
- ❌ Can't pattern match on error types

### Level 3: Custom Domain Errors with `thiserror`

**Type-safe errors for business logic:**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Task ID {id} is invalid (valid range: 1-{max})")]
    InvalidTaskId { id: usize, max: usize },
    
    #[error("Task #{id} is already marked as {status}")]
    TaskAlreadyInStatus { id: usize, status: String },
}

fn validate_task_id(id: usize, max: usize) -> Result<(), TodoError> {
    if id == 0 || id > max {
        return Err(TodoError::InvalidTaskId { id, max });
    }
    Ok(())
}
```

**Characteristics:**

- ✅ Type-safe error variants
- ✅ Rich data in errors
- ✅ Pattern matching support
- ✅ Automatic Display implementation
- ✅ Professional error messages

---

## The `anyhow` Crate

### Core Concept: Error Context

**The `.context()` method:**

```rust
use anyhow::{Context, Result};

fn operation() -> Result<String> {
    fs::read_to_string("file.txt")
        .context("Failed to read configuration file")?
}
```

**What `.context()` does:**

1. Wraps the original error
2. Adds descriptive context
3. Preserves error chain
4. Returns `anyhow::Error`

**Error chain display:**

```bash
Error: Failed to read configuration file
Caused by: No such file or directory (os error 2)
```

### The `Result<T>` Type Alias

**Shorthand for common pattern:**

```rust
// Instead of writing:
fn operation() -> Result<Vec<Task>, anyhow::Error>

// Use shorthand:
fn operation() -> Result<Vec<Task>>
// Automatically means Result<T, anyhow::Error>
```

**Import:**

```rust
use anyhow::Result;  // Shadows std::result::Result
```

### Advanced Context Patterns

**Dynamic context with format!:**

```rust
fn load_user_data(id: u32) -> Result<User> {
    fs::read_to_string(format!("user_{}.json", id))
        .context(format!("Failed to load user {} data", id))?
}
// Error: Failed to load user 42 data
```

**Conditional context:**

```rust
match fs::read_to_string("todos.json") {
    Ok(content) => Ok(content),
    Err(e) if e.kind() == ErrorKind::NotFound => {
        // No context needed - expected case
        Ok(String::new())
    }
    Err(e) => {
        // Add context for unexpected errors
        Err(e).context("Failed to read todos.json")
    }
}
```

### Error Chain Traversal

**Walking the error chain:**

```rust
fn display_error_chain(err: &anyhow::Error) {
    eprintln!("Error: {}", err);
    
    let mut source = err.source();
    while let Some(cause) = source {
        eprintln!("Caused by: {}", cause);
        source = cause.source();
    }
}
```

**Example output:**

```bash
Error: Failed to parse todos.json - file may be corrupted
Caused by: EOF while parsing a value at line 8 column 3
```

---

## The `thiserror` Crate

### Core Concept: Custom Error Types

**The `#[derive(Error)]` macro:**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Task ID {id} is invalid (valid range: 1-{max})")]
    InvalidTaskId { id: usize, max: usize },
}
```

**What the macro generates:**

1. `std::error::Error` implementation
2. `Display` implementation using `#[error]` template
3. Field access for error data

### Error Variant Patterns

**Struct-like variants (named fields):**

```rust
#[error("Task ID {id} is invalid (valid range: 1-{max})")]
InvalidTaskId { id: usize, max: usize },

// Usage:
TodoError::InvalidTaskId { id: 15, max: 10 }
// Display: "Task ID 15 is invalid (valid range: 1-10)"
```

**Tuple variants (positional fields):**

```rust
#[error("Tag '{0}' not found in any task")]
TagNotFound(String),

// Usage:
TodoError::TagNotFound("urgent".to_string())
// Display: "Tag 'urgent' not found in any task"
```

**Unit variants (no fields):**

```rust
#[error("No tasks found matching the specified filters")]
NoTasksFound,

// Usage:
TodoError::NoTasksFound
// Display: "No tasks found matching the specified filters"
```

### Field Interpolation

**Accessing fields in error messages:**

```rust
#[error("Expected {expected}, found {found}")]
TypeMismatch { expected: String, found: String },

// Multiple field access patterns:
#[error("Position: ({x}, {y})")]
Position { x: i32, y: i32 },

// Positional access for tuples:
#[error("First: {0}, Second: {1}")]
TwoValues(String, String),
```

### Error Source Chaining

**Wrapping other errors:**

```rust
#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Failed to load tasks")]
    LoadError(#[from] std::io::Error),
    
    #[error("Failed to parse tasks")]
    ParseError(#[from] serde_json::Error),
}
```

**The `#[from]` attribute:**

- Automatically implements `From<IoError>` for `TodoError`
- Enables `?` operator to convert errors
- Preserves error chain

```rust
fn load_tasks() -> Result<Vec<Task>, TodoError> {
    let content = fs::read_to_string("todos.json")?;
    // IoError automatically converts to TodoError::LoadError
    
    let tasks = serde_json::from_str(&content)?;
    // JsonError automatically converts to TodoError::ParseError
    
    Ok(tasks)
}
```

---

## Combining `anyhow` and `thiserror`

### The Strategy

**Use both together for maximum benefit:**

1. **`thiserror`** for domain-specific errors
2. **`anyhow`** for application-level error handling
3. Convert between them with `.into()`

### Conversion Pattern

**Custom error → anyhow error:**

```rust
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Invalid task ID: {0}")]
    InvalidId(usize),
}

fn validate_id(id: usize) -> Result<()> {
    if id == 0 {
        return Err(TodoError::InvalidId(id).into());
        //                                    ↑ Convert to anyhow::Error
    }
    Ok(())
}
```

**Why this works:**

- `anyhow::Error` accepts any `std::error::Error`
- `thiserror` implements `std::error::Error`
- `.into()` performs the conversion

### Real-World Example

**Domain errors + application context:**

```rust
#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Task ID {id} is invalid (valid range: 1-{max})")]
    InvalidTaskId { id: usize, max: usize },
}

fn done_command(id: usize) -> Result<()> {
    let mut tasks = load_tasks()
        .context("Failed to load tasks from file")?;  // anyhow context
    
    validate_task_id(id, tasks.len())?;  // TodoError propagates
    
    let index = id - 1;
    if tasks[index].completed {
        return Err(TodoError::TaskAlreadyInStatus {
            id,
            status: "completed".to_owned(),
        }
        .into());  // Convert TodoError → anyhow::Error
    }
    
    tasks[index].mark_done();
    save_tasks(&tasks)
        .context("Failed to save tasks to file")?;  // anyhow context
    
    Ok(())
}
```

**Error output:**

```bash
# For infrastructure error:
Error: Failed to load tasks from file
Caused by: Permission denied (os error 13)

# For domain error:
Error: Task #3 is already marked as completed
```

---

## Error Handling Patterns

### Pattern 1: Granular Error Matching

**Handle specific error types differently:**

```rust
match fs::read_to_string("todos.json") {
    Ok(content) => parse_tasks(content),
    Err(e) if e.kind() == ErrorKind::NotFound => {
        // File missing is expected - start with empty list
        Ok(Vec::new())
    }
    Err(e) if e.kind() == ErrorKind::PermissionDenied => {
        // Permission error needs different handling
        Err(e).context("No permission to read todos.json - check file permissions")
    }
    Err(e) => {
        // Other errors
        Err(e).context("Failed to read todos.json")
    }
}
```

### Pattern 2: Early Return with Context

**Validate and add context at each step:**

```rust
fn process_task(id: usize) -> Result<()> {
    // Validation
    validate_task_id(id, 10)
        .context("Task ID validation failed")?;
    
    // Load with context
    let mut tasks = load_tasks()
        .context("Failed to load task list")?;
    
    // Business logic
    let task = tasks.get_mut(id - 1)
        .ok_or_else(|| anyhow!("Task {} not found", id))?;
    
    task.mark_done();
    
    // Save with context
    save_tasks(&tasks)
        .context("Failed to save updated tasks")?;
    
    Ok(())
}
```

### Pattern 3: Fallible Iterators

**Handle errors in collection operations:**

```rust
fn load_all_tasks() -> Result<Vec<Task>> {
    let paths = fs::read_dir("tasks")?;
    
    let tasks: Result<Vec<Task>> = paths
        .map(|entry| {
            let entry = entry?;
            let content = fs::read_to_string(entry.path())
                .context(format!("Failed to read {:?}", entry.path()))?;
            serde_json::from_str(&content)
                .context(format!("Failed to parse {:?}", entry.path()))
        })
        .collect();
    
    tasks
}
```

### Pattern 4: Custom Error Construction

**Building errors with rich context:**

```rust
fn find_task_by_tag(tag: &str) -> Result<Task> {
    let tasks = load_tasks()?;
    
    tasks
        .into_iter()
        .find(|t| t.tags.contains(&tag.to_string()))
        .ok_or_else(|| {
            anyhow!(
                "No task found with tag '{}'. Available tags: {}",
                tag,
                get_all_tags().join(", ")
            )
        })
}
```

---

## Best Practices

### 1. Context at Every Layer

**Add context at each abstraction level:**

```rust
// ✅ Good: Context at each layer
fn high_level() -> Result<()> {
    mid_level()
        .context("High-level operation failed")?;
    Ok(())
}

fn mid_level() -> Result<()> {
    low_level()
        .context("Mid-level operation failed")?;
    Ok(())
}

fn low_level() -> Result<()> {
    fs::read_to_string("file.txt")
        .context("Failed to read file")?;
    Ok(())
}
```

**Error output:**

```bash
Error: High-level operation failed
Caused by: Mid-level operation failed
Caused by: Failed to read file
Caused by: No such file or directory (os error 2)
```

### 2. Specific Error Messages

**Include relevant details in messages:**

```rust
// ❌ Bad: Generic
.context("Operation failed")

// ✅ Good: Specific
.context("Failed to read todos.json from current directory")

// ✅ Better: With dynamic data
.context(format!("Failed to load user {} data", user_id))
```

### 3. Use `thiserror` for Domain Logic

**Domain errors deserve custom types:**

```rust
// ✅ Good: Custom error for business rules
#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Task ID {id} is invalid (valid range: 1-{max})")]
    InvalidTaskId { id: usize, max: usize },
}

// ❌ Bad: Generic error for business rules
if id > max {
    return Err(anyhow!("Invalid ID"));
}
```

### 4. Use `anyhow` for Infrastructure

**Infrastructure errors don't need custom types:**

```rust
// ✅ Good: anyhow for file operations
fs::read_to_string("todos.json")
    .context("Failed to read todos.json")?

// ❌ Overkill: Custom error for file operations
#[derive(Error, Debug)]
enum FileError {
    #[error("Failed to read file")]
    ReadError,
}
```

### 5. Convert at API Boundaries

**Keep internal errors typed, convert at boundaries:**

```rust
// Internal function - returns custom error
fn validate_task_id(id: usize, max: usize) -> Result<(), TodoError> {
    if id == 0 || id > max {
        return Err(TodoError::InvalidTaskId { id, max });
    }
    Ok(())
}

// Public API - converts to anyhow
pub fn done_command(id: usize) -> Result<()> {
    validate_task_id(id, max)?;  // TodoError converts automatically
    // ... rest of command
    Ok(())
}
```

---

## Testing Error Cases

### Unit Testing Custom Errors

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_task_id() {
        let result = validate_task_id(0, 5);
        assert!(result.is_err());
        
        let err = result.unwrap_err();
        assert!(matches!(err, TodoError::InvalidTaskId { .. }));
    }

    #[test]
    fn test_error_message() {
        let err = TodoError::InvalidTaskId { id: 10, max: 5 };
        assert_eq!(
            err.to_string(),
            "Task ID 10 is invalid (valid range: 1-5)"
        );
    }
}
```

### Integration Testing Error Chains

```rust
#[test]
fn test_error_chain() {
    let result = load_tasks_from_corrupted_file();
    
    assert!(result.is_err());
    let err = result.unwrap_err();
    
    // Check main error
    let msg = err.to_string();
    assert!(msg.contains("Failed to parse"));
    
    // Check error chain
    let source = err.source().unwrap();
    assert!(source.to_string().contains("EOF while parsing"));
}
```

---

## Performance Considerations

### Error Creation Cost

**Errors have overhead - use appropriately:**

```rust
// ✅ Good: Error in error path (infrequent)
if id == 0 {
    return Err(TodoError::InvalidTaskId { id, max });
}

// ❌ Bad: Error in hot path (frequent)
for id in 1..1_000_000 {
    validate_task_id(id, max)?;  // Creating errors constantly
}

// ✅ Better: Validate before loop
validate_task_id_range(1, 1_000_000, max)?;
for id in 1..1_000_000 {
    // No validation needed
}
```

### String Allocation

**Context messages allocate strings:**

```rust
// ✅ Good: Static string (no allocation)
.context("Failed to read file")

// 🤔 OK: Dynamic string (allocates, but only on error)
.context(format!("Failed to read user {} data", id))

// ❌ Bad: Unnecessary allocation
.context(format!("Failed"))  // Just use static string!
```

---

## Resources

- [anyhow documentation](https://docs.rs/anyhow/)
- [thiserror documentation](https://docs.rs/thiserror/)
- [Rust Error Handling Book](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Error Handling Survey](https://blog.burntsushi.net/rust-error-handling/)

---

**📚 See Also:**

- [Error Handling Basics](error-handling.md) - Foundational patterns
- [Type Safety](type-safety.md) - Using types to prevent errors
- [Professional Error Handling](../advanced/v1.7.0-professional-error-handling.md) - Complete implementation
