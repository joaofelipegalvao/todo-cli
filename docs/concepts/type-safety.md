# Type Safety

**📚 Overview:**

This page covers type safety patterns used throughout the todo-cli project, from basic string handling to advanced type-safe enums and structs.

**🔗 Related Versions:**

- [v1.2.0](../advanced/v1.2.0-struct-refactoring.md) - Structs and enums
- [v1.3.0](../advanced/v1.3.0-json-serialization.md) - Serde type safety
- [v1.6.0](../advanced/v1.6.0-professional-cli-clap.md) - Clap type safety

---

## String vs Enum Types

### String-Based Approach (Early Versions)

**Error-prone string matching:**

```rust
// ❌ Can have typos that compile
let priority = "hihg";  // Typo! Compiles fine
if priority == "high" { }  // Won't match, silent bug

// ❌ No exhaustiveness checking
match priority.as_str() {
    "high" => "🔴",
    "medium" => "🟡",
    // Forgot "low"? ❌ No compiler error
    _ => "❓",  // Catch-all needed
}
```

### Enum-Based Approach (Mature Versions)

**Type-safe with compile-time checking:**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Priority {
    High,
    Medium,
    Low,
}

// ✅ Typos = compile error
let priority = Priority::Hihg;  // ERROR: no variant `Hihg`

// ✅ Exhaustive matching enforced
match priority {
    Priority::High => "🔴",
    Priority::Medium => "🟡",
    Priority::Low => "🟢",
    // Can't forget any variant - compiler error!
}
```

## When to Use Strings vs Enums

### Use Strings When:

- **User-provided data**: Task text, descriptions
- **Open-ended values**: Free-form input
- **Display purposes**: Formatting for output
- **Simple prototypes**: Quick development phase

### Use Enums When:

- **Fixed set of values**: Priorities, statuses, filters
- **Need compile-time validation**: Prevent invalid states
- **Want exhaustive matching**: Handle all cases
- **Values are fundamental to domain**: Core business concepts

**Our evolution:**

```rust
// v0.1-v1.1: Strings for everything (learning)
// v1.2+: Enums for priority, strings for task text (mature)
```

## Option Type for Optional Values

### Avoiding "Magic Values"

**❌ Using special values:**

```rust
struct Task {
    text: String,
    completed: bool,
    priority: String,  // "" could mean "no priority" or "medium"
    due_date: String,  // "" could mean "no due date"
}
```

**✅ Using Option for clarity:**

```rust
struct Task {
    text: String,
    completed: bool,
    priority: Priority,  // Always has a priority
    due_date: Option<NaiveDate>,  // None = no due date
}
```

### Pattern Matching on Option

**Safe handling of optional values:**

```rust
fn is_overdue(&self) -> bool {
    if let Some(due) = self.due_date {
        let today = Local::now().naive_local().date();
        due < today && !self.completed
    } else {
        false  // No due date = can't be overdue
    }
}
```

**Alternative with `map`:**

```rust
fn days_until_due(&self) -> Option<i64> {
    self.due_date.map(|due| {
        let today = Local::now().naive_local().date();
        (due - today).num_days()
    })
}
```

## Result Type for Error Handling

### Type-Safe Error Propagation

**Using Result for operations that can fail:**

```rust
fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    match fs::read_to_string("todos.json") {
        Ok(content) => {
            let tasks: Vec<Task> = serde_json::from_str(&content)?;
            Ok(tasks)
        }
        Err(_) => Ok(Vec::new()),
    }
}
```

**Chaining operations with `?`:**

```rust
fn save_tasks(tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(tasks)?;
    fs::write("todos.json", json)?;
    Ok(())
}
```

## Struct-Based Data Organization

### Grouping Related Data

**Before - scattered variables:**

```rust
fn display_task(number: usize, text: &str, completed: bool, priority: &str) {
    // Hard to pass around, easy to mix up parameters
}
```

**After - organized struct:**

```rust
#[derive(Debug, Clone)]
struct Task {
    text: String,
    completed: bool,
    priority: Priority,
    due_date: Option<NaiveDate>,
    tags: Vec<String>,
}

fn display_task(number: usize, task: &Task) {
    // Clear data organization, type-safe access
    if task.completed {
        println!("✓ {}", task.text);
    }
}
```

### Method Encapsulation

**Business logic on types:**

```rust
impl Task {
    // Constructor - ensures valid initial state
    fn new(text: String, priority: Priority, tags: Vec<String>, 
            due_date: Option<NaiveDate>) -> Self {
        Self {
            text,
            completed: false,  // Always starts incomplete
            priority,
            tags,
            due_date,
            created_at: Local::now().naive_local().date(),
        }
    }
    
    // State transitions - type-safe operations
    fn mark_done(&mut self) {
        self.completed = true;
    }
    
    fn mark_undone(&mut self) {
        self.completed = false;
    }
    
    // Queries - clear return types
    fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            let today = Local::now().naive_local().date();
            due < today && !self.completed
        } else {
            false
        }
    }
}
```

## Serde Type Safety

### Automatic Serialization Validation

**Type-safe JSON operations:**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    text: String,
    completed: bool,
    priority: Priority,
    tags: Vec<String>,
    due_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum Priority {
    High,
    Medium,
    Low,
}
```

**What serde provides automatically:**

1. **Structure validation** - JSON must match struct fields
2. **Type validation** - "completed" must be boolean
3. **Enum validation** - "priority" must be valid variant
4. **Optional field handling** - `Option<T>` fields can be missing

**Error examples:**

```json
// ❌ Missing required field
{
  "text": "Task",
  "completed": false
  // Missing "priority"
}
// Error: missing field `priority`

// ❌ Wrong type
{
  "text": "Task",
  "completed": "yes",  // Should be boolean
  "priority": "High"
}
// Error: invalid type: string "yes", expected boolean

// ❌ Invalid enum value
{
  "text": "Task",
  "completed": false,
  "priority": "urgent"  // Not a valid variant
}
// Error: unknown variant `urgent`, expected one of `High`, `Medium`, `Low`
```

## Clap Type Safety

### Type-Safe CLI Parsing

**ValueEnum for CLI values:**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum StatusFilter {
    /// Show only pending tasks
    Pending,
    /// Show only completed tasks
    Done,
    /// Show all tasks (default)
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum DueFilter {
    /// Tasks past their due date
    Overdue,
    /// Tasks due in the next 7 days
    Soon,
    /// Tasks with any due date set
    WithDue,
    /// Tasks without a due date
    NoDue,
}
```

**Usage in CLI:**

```rust
List {
    #[arg(long, value_enum, default_value_t = StatusFilter::All)]
    status: StatusFilter,
    
    #[arg(long, value_enum)]
    due: Option<DueFilter>,
}
```

**Benefits:**

- ✅ **Compile-time safety** - invalid values rejected
- ✅ **Auto-completion** - shell knows valid values
- ✅ **Help generation** - values shown in help
- ✅ **Mutual exclusion** - `Option<DueFilter>` prevents conflicts

### Custom Type Parsers

**Automatic type parsing with value_parser:**

```rust
#[derive(Args)]
struct AddArgs {
    #[arg(long, value_parser = clap::value_parser!(NaiveDate))]
    due: Option<NaiveDate>,
}
```

**How it works:**

1. Clap takes string from CLI
2. Calls `NaiveDate::from_str()`
3. Returns parsed value or error
4. Error includes context and suggestions

**Error example:**

```bash
$ todo add "Task" --due 2026-13-50
error: invalid value '2026-13-50' for '--due <DATE>': input is out of range
```

## Type Safety Benefits

### Compile-Time Error Prevention

**Strings allow runtime errors:**

```rust
// This compiles but fails at runtime
let priority = "hihg";  // Typo
if priority == "high" { }  // Silent bug
```

**Enums prevent runtime errors:**

```rust
// This doesn't compile - catches bug immediately
let priority = Priority::Hihg;  // Compile error!
```

### Self-Documenting Code

**Strings require external knowledge:**

```rust
// What values are valid? Need to read source code
let status = "pending";  // Is this valid? Who knows
```

**Enums are self-documenting:**

```rust
// IDE shows all valid options
let status = StatusFilter::Pending;  // Only valid values possible
```

### Refactoring Safety

**Changing string values is risky:**

```rust
// Change "high" to "critical"?
// Must search entire codebase for "high"
// Easy to miss some occurrences
if priority == "high" { }  // Might miss this one
```

**Changing enum values is safe:**

```rust
// Rename High to Critical?
// Compiler finds all occurrences automatically
if priority == Priority::High { }  // Compile error forces update
```

## Best Practices Summary

1. **Use enums for fixed domains** - priorities, statuses, filters
2. **Use Option for optional data** - due dates, optional fields
3. **Use Result for fallible operations** - file I/O, parsing
4. **Group related data in structs** - clear organization
5. **Use method encapsulation** - business logic on types
6. **Leverage serde for serialization** - automatic validation
7. **Use clap for CLI type safety** - ValueEnum patterns
8. **Prefer compile-time checks** over runtime validation

---

**📚 See Also:**

- [Struct Refactoring](../advanced/v1.2.0-struct-refactoring.md) - Complete type safety migration
- [JSON Serialization](../advanced/v1.3.0-json-serialization.md) - Serde type patterns
- [Professional CLI](../advanced/v1.6.0-professional-cli-clap.md) - Clap type safety