# Error Handling

**📚 Overview:**

This page covers error handling patterns used throughout the todo-cli project, from basic string errors to professional type-safe validation.

**🔗 Related Versions:**

- [v0.1.0](../getting-started/v0.1.0-basic-cli.md) - Basic `?` operator
- [v0.3.0](../getting-started/v0.3.0-remove-command.md) - Input validation
- [v0.4.2](../getting-started/v0.4.2-state-validations.md) - Precondition validation
- [v1.6.0](../advanced/v1.6.0-professional-cli-clap.md) - Clap error handling

---

## The `?` Operator

**Basic error propagation:**

```rust
fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    let content = fs::read_to_string("todos.json")?;  // Auto-propagate
    // ...
}
```

**What it does:**

- If `Ok(value)` → extracts value
- If `Err(error)` → returns error from current function

**Equivalent verbose version:**

```rust
let content = match fs::read_to_string("todos.json") {
    Ok(content) => content,
    Err(e) => return Err(e.into()),
};
```

## `Box<dyn Error>` for Generic Errors

**Why use `Box<dyn Error>`?**

```rust
fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    //                                    ↑ Can hold any error type
}
```

**Benefits:**

- ✅ Accepts any error type (`std::io::Error`, `serde_json::Error`, etc.)
- ✅ No need to specify exact error type
- ✅ Users get meaningful error messages

**Alternative approaches:**

```rust
// ❌ Too specific - only works for IO errors
fn load_tasks() -> Result<Vec<Task>, std::io::Error>

// ❌ Too complex - need to combine all error types
fn load_tasks() -> Result<Vec<Task>, CombinedError>

// ✅ Just right - accepts any error
fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>>
```

## Input Validation

**Validate user input before processing:**

```rust
"done" => {
    if args.len() < 3 {
        return Err("Usage: todo done <number>".into());
    }
    
    let number: usize = args[2].parse()?;
    
    if number == 0 || number > tasks.len() {
        return Err("Invalid task number".into());
    }
    
    // ... proceed with valid input
}
```

**Validation layers:**

1. **Argument count** - Right number of arguments
2. **Type parsing** - String → number conversion
3. **Range validation** - Number within valid range
4. **State validation** - Task is in correct state

## Precondition Validation

**Check business rules before acting:**

```rust
"done" => {
    // ... parsing and range validation ...
    
    let index = number - 1;
    
    // Precondition: task must be pending
    if tasks[index].completed {
        return Err("Task is already marked as completed".into());
    }
    
    // If we reach here, precondition is satisfied
    tasks[index].mark_done();
}
```

**Why preconditions matter:**

- Prevent invalid state transitions
- Provide specific, helpful error messages
- Maintain data integrity

## Error Message Design

**Good error messages:**

1. ✅ **Tell user what went wrong**
2. ✅ **Explain why it's wrong**  
3. ✅ **Suggest how to fix it**

**Examples:**

```rust
// ❌ Generic and unhelpful
return Err("Invalid input".into());

// ✅ Specific and helpful
return Err("Invalid task number. Use a number between 1 and 5".into());

// ✅ Even better - shows the actual problem
return Err(format!("Task {} doesn't exist. Available tasks: 1-5", number).into());
```

## Clap Error Handling

**Professional CLI errors with clap:**

```rust
#[derive(Args)]
struct AddArgs {
    #[arg(value_name = "DESCRIPTION")]
    text: String,
    
    #[arg(long, value_parser = clap::value_parser!(NaiveDate))]
    due: Option<NaiveDate>,
}
```

**Automatic error handling:**

```bash
$ todo add "Task" --due 2026-13-50
error: invalid value '2026-13-50' for '--due <DATE>': input is out of range

$ todo add "Task" --due tomorrow  
error: invalid value 'tomorrow' for '--due <DATE>': premature end of input
```

**Benefits:**

- ✅ Context-aware errors
- ✅ Shows expected format
- ✅ Highlights the problematic value
- ✅ No manual error writing needed

## Error Recovery Strategies

**Graceful degradation:**

```rust
fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    match fs::read_to_string("todos.json") {
        Ok(content) => {
            let tasks: Vec<Task> = serde_json::from_str(&content)?;
            Ok(tasks)
        }
        Err(_) => {
            // File doesn't exist or can't be read
            // Start with empty list instead of crashing
            Ok(Vec::new())
        }
    }
}
```

**Idempotent operations:**

```rust
"clear" => {
    if fs::metadata("todos.json").is_ok() {
        fs::remove_file("todos.json")?;
        println!("✓ All tasks have been removed");
    } else {
        println!("No tasks to remove");  // Not an error!
    }
}
```

## Testing Error Cases

**Test error conditions:**

```rust
#[test]
fn test_invalid_task_number() {
    let mut tasks = vec![
        Task::new("Task 1".into(), Priority::Medium, vec![]),
        Task::new("Task 2".into(), Priority::Medium, vec![]),
    ];
    
    // Try to mark task 3 as done (only 2 exist)
    let result = mark_done(&mut tasks, 3);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Invalid task number");
}
```

## Best Practices Summary

1. **Use `?` operator** for simple error propagation
2. **Validate early** - fail fast with clear messages
3. **Be specific** - tell users exactly what's wrong
4. **Use types** - `Option<T>` and `Result<T, E>` for safety
5. **Handle gracefully** - don't crash on recoverable errors
6. **Test errors** - ensure error paths work correctly
7. **Use frameworks** - clap for CLI, serde for serialization

---

**📚 See Also:**

- [File Operations](file-operations.md) - File-related error handling
- [CLI Design](cli-design.md) - Command-line interface patterns
- [Type Safety](type-safety.md) - Using types to prevent errors