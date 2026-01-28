# 📚 Learning Journey

> How I learned Rust by building this CLI, version by version

## 🎯 Project Goal

Learn Rust in practice by building something useful, documenting each decision and concept learned along the way.

## 📖 Navigation

- [Version History](#version-history)
- [Concepts Learned](#concepts-learned)
- [Design Decisions](#design-decisions)

---

## Version History

### v0.1.0 - Basic CLI

**🎯 Goal:** Create the foundation of a CLI with add/list functionality

**📦 Implementation:**

```rust
"add" => {
    let task = &args[2];
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("todos.txt")?;
    writeln!(file, "[ ] {}", task)?;
}
```

**🧠 Key Concepts:**

#### Why `OpenOptions` instead of `File::create()`?

```rust
// ❌ This OVERWRITES the entire file!
let file = File::create("todos.txt")?;

// ✅ OpenOptions gives granular control
let file = OpenOptions::new()
    .create(true)    // Create if doesn't exist
    .append(true)    // Don't overwrite, add to end
    .open("todos.txt")?;
```

#### The `?` operator

```rust
let file = open_file()?;  // Automatically propagates error
```

Equivalent verbose version:

```rust
let file = match open_file() {
    Ok(f) => f,
    Err(e) => return Err(e.into()),
};
```

**Why `?` is better:**

- Less code
- Clear intention
- Idiomatic Rust

#### Pattern matching for subcommands

```rust
match args[1].as_str() {
    "add" => { /* ... */ }
    "list" => { /* ... */ }
    _ => { eprintln!("Unknown command"); }
}
```

**🔗 Resources:**

- [Code v0.1.0](https://github.com/joaofelipegalvao/todo-cli/tree/v0.1.0)
- [Full diff](https://github.com/joaofelipegalvao/todo-cli/compare/v0.0.0...v0.1.0)

---

### v0.2.0 - Done Command

**🎯 Goal:** Mark tasks as completed

**📦 Implementation:**

```rust
"done" => {
    if args.len() < 3 {
        return Err("Usage: todo done <number>".into());
    }
    
    let number: usize = args[2].parse()?;
    let content = fs::read_to_string("todos.txt")?;
    
    let mut lines: Vec<String> = content
        .lines()
        .map(|l| l.to_string())
        .collect();
    
    let index = number - 1;
    lines[index] = lines[index].replace("[ ]", "[x]");
    
    fs::write("todos.txt", lines.join("\n") + "\n")?;
    println!("✓ Task marked as completed");
}
```

**🧠 Key Concepts:**

#### `.map().collect()` pattern

```rust
let mut lines: Vec<String> = content
    .lines()           // Iterator of &str
    .map(|l| l.to_string())  // Transform each &str → String
    .collect();        // Gather into Vec<String>
```

This is the **iterator pattern** - lazy evaluation until `collect()`.

**Why this pattern?**

- `.lines()` returns an iterator (doesn't allocate yet)
- `.map()` transforms each item (still lazy)
- `.collect()` forces evaluation and builds the Vec

#### Why `.to_string()` is needed

```rust
content.lines()  // Returns Iterator<Item = &str>
```

Each line is a **borrowed reference** (`&str`) to data in `content`. We need **owned** `String` values so we can:

1. Modify them (`.replace()`)
2. Store them in a Vec that outlives `content`

```rust
.map(|l| l.to_string())  // Converts &str → String (owned)
```

#### Parsing user input

```rust
let number: usize = args[2].parse()?;
```

**What's happening:**

- `args[2]` is a `String` (e.g., `"3"`)
- `.parse()` tries to convert it to `usize`
- `:usize` type annotation tells `parse()` what to produce
- `?` propagates error if user typed invalid number

**Error handling:**

```rust
// If user types "abc"
args[2].parse()?  // Returns Err → propagated to main
// User sees: "Error: invalid digit found in string"
```

#### Index conversion: 1-based → 0-based

```rust
let number: usize = args[2].parse()?;  // User sees: 1, 2, 3...
let index = number - 1;                 // Vec uses: 0, 1, 2...
lines[index] = lines[index].replace("[ ]", "[x]");
```

**Why this matters:**

- Users think in 1-based numbers (1st task, 2nd task)
- Rust Vecs are 0-indexed
- Must convert to avoid off-by-one errors

#### Writing back to file

```rust
fs::write("todos.txt", lines.join("\n") + "\n")?;
```

**Breaking it down:**

- `lines.join("\n")` - combines Vec elements with newlines between
- `+ "\n"` - adds final newline at end of file
- `fs::write()` - overwrites entire file with new content

**Why overwrite instead of append?**

- Can't "edit middle" of file efficiently
- Must read entire file, modify in memory, write back
- This is fine for small files (hundreds of tasks)

#### The `.replace()` method

```rust
lines[index] = lines[index].replace("[ ]", "[x]");
```

**What it does:**

- Finds first occurrence of `"[ ]"`
- Replaces it with `"[x]"`
- Returns a **new String** (doesn't modify original)

**Example:**

```rust
let task = "[ ] Buy milk";
let done = task.replace("[ ]", "[x]");
// done = "[x] Buy milk"
// task is unchanged (because String is immutable)
```

**Limitations not handled yet:**

- ❌ No validation if index is out of bounds
- ❌ No check if task is already done
- ❌ Empty file would panic

**These will be fixed in v0.3.0 and v0.4.2**

**🔗 Resources:**

- [Code v0.2.0](https://github.com/joaofelipegalvao/todo-cli/commit/a5626567103c1faa69c96caea1cab27ad6f89b14)
- [Full diff](https://github.com/joaofelipegalvao/todo-cli/compare/v0.1.0...v0.2.0)

---

### v0.3.0 - Remove Command

**🎯 Goal:** Delete specific tasks

**📦 Implementation:**

```rust
"remove" => {
    if args.len() < 3 {
        return Err("Usage: todo remove <number>".into());
    }
    
    let number: usize = args[2].parse()?;
    let content = fs::read_to_string("todos.txt")?;
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    
    if number == 0 || number > lines.len() {
        return Err("Invalid task number".into());
    }
    
    let index = number - 1;
    lines.remove(index);
    
    fs::write("todos.txt", lines.join("\n") + "\n")?;
    println!("✓ Task removed");
}
```

**🧠 Key Concepts:**

#### Index validation

```rust
if number == 0 || number > lines.len() {
    return Err("Invalid task number".into());
}
```

**Why this validation is critical:**

**Case 1: Zero index**

```rust
// User types: todo remove 0
number == 0  // Invalid! Users see tasks numbered 1, 2, 3...
```

**Case 2: Out of bounds**

```rust
// File has 5 tasks, user types: todo remove 10
number > lines.len()  // Would panic at lines.remove(9)
```

**Without validation:**

```rust
lines.remove(9)  // panics: "index out of bounds"
// User sees: "thread 'main' panicked at..." ❌ Bad UX
```

**With validation:**

```rust
return Err("Invalid task number".into());
// User sees: "Error: Invalid task number" ✅ Clear message
```

#### `Vec::remove()` method

```rust
lines.remove(index);
```

**What it does:**

- Removes element at index `index`
- Shifts all following elements left
- Reduces Vec length by 1

**Example:**

```rust
let mut tasks = vec!["Task 1", "Task 2", "Task 3", "Task 4"];
tasks.remove(1);  // Remove "Task 2"
// Result: ["Task 1", "Task 3", "Task 4"]
//                     ↑ shifted left
```

**Performance note:**

- O(n) operation - must shift all elements after removed index
- For small lists (hundreds of tasks), this is fine
- For large lists (thousands), might need different data structure

#### Comparison: `remove()` vs `replace()`

```rust
// done command (v0.2.0) - modifies content
lines[index] = lines[index].replace("[ ]", "[x]");

// remove command (v0.3.0) - deletes entire element
lines.remove(index);
```

**Key difference:**

- `replace()` - task still exists, just changed
- `remove()` - task is gone, indices shift

**This affects user experience:**

```
Before remove(1):        After remove(1):
1. Task A                1. Task B  ← was 2, now 1
2. Task B                2. Task C  ← was 3, now 2
3. Task C
```

Users need to `list` again to see new numbers.

#### Validation added to `done` command

**Notice the improvement in v0.3.0:**

```rust
"done" => {
    // ... parsing ...
    
    // ✅ NEW: Validation added!
    if number == 0 || number > lines.len() {
        return Err("Invalid task number".into());
    }
    
    let index = number - 1;
    lines[index] = lines[index].replace("[ ]", "[x]");
    // ...
}
```

**Before v0.3.0:** `done` command could panic on invalid index  
**After v0.3.0:** Both `done` and `remove` have proper validation

**This is iterative improvement** - recognizing a pattern (validation) and applying it consistently.

**🔗 Resources:**

- [Code v0.3.0](https://github.com/joaofelipegalvao/todo-cli/tree/v0.3.0)
- [Full diff](https://github.com/joaofelipegalvao/todo-cli/compare/v0.2.0...v0.3.0)

---

### v0.4.0 - Undone Command

**🎯 Goal:** Unmark completed tasks (reverse of `done`)

**📦 Implementation:**

```rust
"undone" => {
    if args.len() < 3 {
        return Err("Usage: todo undone <number>".into());
    }
    
    let number: usize = args[2].parse()?;
    let content = fs::read_to_string("todos.txt")?;
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    
    if number == 0 || number > lines.len() {
        return Err("Invalid task number".into());
    }
    
    let index = number - 1;
    lines[index] = lines[index].replace("[x]", "[ ]");  // ← Reverse!
    
    fs::write("todos.txt", lines.join("\n") + "\n")?;
    println!("✓ Task unmarked");
}
```

**🧠 Key Concepts:**

#### Inverse operations

```rust
// done: marks as completed
lines[index].replace("[ ]", "[x]")

// undone: marks as pending  
lines[index].replace("[x]", "[ ]")
```

**This is the power of simple data representation:**

- State is just text: `"[ ]"` or `"[x]"`
- Changing state is just string replacement
- Inverse operation is trivial

**Alternative design (that would be more complex):**

```rust
// If we had stored status separately:
struct Task {
    text: String,
    completed: bool,  // Now we need struct, serialization, etc.
}
```

**Our choice:** Keep it simple - plain text format enables easy state changes.

#### Code duplication notice

**Look at the pattern:**

```rust
// done, undone, and remove all have:
if args.len() < 3 { return Err(...); }
let number: usize = args[2].parse()?;
let content = fs::read_to_string("todos.txt")?;
let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
if number == 0 || number > lines.len() { return Err(...); }
let index = number - 1;
// ... specific operation ...
fs::write("todos.txt", lines.join("\n") + "\n")?;
```

**This duplication is intentional at this stage:**

- ✅ Learning step-by-step
- ✅ Each command is self-contained
- ✅ Easy to understand

**Later (v1.0.0+), this will be refactored** into helper functions. For now, repetition helps learning.

#### Boolean logic in action

The command structure now forms a **state machine:**

```
      ┌─────────┐
      │ Pending │
      │  [ ]    │
      └────┬────┘
           │
    done   │   undone
    ───────┼───────
           │
      ┌────▼────┐
      │Completed│
      │  [x]    │
      └─────────┘
```

Tasks can toggle between states, and `remove` deletes from any state.

**🔗 Resources:**

- [Code v0.4.0](https://github.com/joaofelipegalvao/todo-cli/tree/v0.4.0)
- [Full diff](https://github.com/joaofelipegalvao/todo-cli/compare/v0.3.0...v0.4.0)

---

### v0.4.1 - List Bug Fix

**🎯 Goal:** Handle empty lines properly in the list command

**📦 The Bug:**

**Before v0.4.1:**

```rust
"list" => match fs::read_to_string("todos.txt") {
    Ok(content) => {
        for (i, line) in content.lines().enumerate() {
            println!("{}. {}", i + 1, line);  // Shows empty lines!
        }
    }
    // ...
}
```

**Problem:**

```
File content:          Display output:
[ ] Task 1             1. [ ] Task 1
                       2.              ← Empty line!
[ ] Task 2             3. [ ] Task 2
```

**After v0.4.1:**

```rust
"list" => match fs::read_to_string("todos.txt") {
    Ok(content) => {
        let valid_lines: Vec<&str> = content
            .lines()
            .filter(|l| !l.trim().is_empty())  // ← The fix!
            .collect();
        
        if valid_lines.is_empty() {
            println!("No tasks");
        } else {
            for (i, line) in valid_lines.iter().enumerate() {
                println!("{}. {}", i + 1, line);
            }
        }
    }
    Err(_) => {
        println!("No tasks");
    }
}
```

**🧠 Key Concepts:**

#### Why `.trim()` is necessary

**Where do empty lines come from?**

```rust
// When we write file:
fs::write("todos.txt", lines.join("\n") + "\n")?;
//                                          ↑ adds final newline

// File content:
"[ ] Task 1\n[ ] Task 2\n"
//                      ↑ this creates an empty line when splitting
```

**Without trim:**

```rust
"[ ] Task 1\n[ ] Task 2\n".lines()
// Results in: ["[ ] Task 1", "[ ] Task 2", ""]
//                                            ↑ empty string!
```

**With trim:**

```rust
.filter(|l| !l.trim().is_empty())
// "".trim() = "" (still empty) → filtered out
// "   ".trim() = "" → also filtered out (whitespace-only lines)
```

#### The `.filter()` method

```rust
content.lines()
    .filter(|l| !l.trim().is_empty())
    .collect()
```

**How it works:**

- Takes each line from iterator
- Applies predicate: `!l.trim().is_empty()`
- Keeps only lines where predicate is `true`

**Example:**

```rust
let lines = vec!["Task 1", "", "  ", "Task 2"];
let valid: Vec<_> = lines.iter()
    .filter(|l| !l.trim().is_empty())
    .collect();
// Result: ["Task 1", "Task 2"]
```

#### Improved empty file handling

**Also added check for empty list:**

```rust
if valid_lines.is_empty() {
    println!("No tasks");
} else {
    // display tasks
}
```

**Why this matters:**

- File exists but has only empty lines
- Better UX than showing nothing

#### Edge cases now handled

✅ Empty file  
✅ File with only whitespace  
✅ File with trailing newlines  
✅ File with blank lines between tasks  

**Bug lesson:** Always test with "weird" input (empty files, extra newlines, whitespace).

**🔗 Resources:**

- [Code v0.4.1](https://github.com/joaofelipegalvao/todo-cli/tree/v0.4.1)
- [Full diff](https://github.com/joaofelipegalvao/todo-cli/compare/v0.4.0...v0.4.1)

---

### v0.4.2 - State Validations

**🎯 Goal:** Prevent invalid state transitions with specific error messages

**📦 Implementation:**

**Validation in `done` command:**

```rust
"done" => {
    // ... parsing and validation ...
    
    let index = number - 1;
    
    // ✅ NEW: Check if already completed
    if lines[index].contains("[x]") {
        return Err("Task is already marked as completed".into());
    }
    
    lines[index] = lines[index].replace("[ ]", "[x]");
    // ...
}
```

**Validation in `undone` command:**

```rust
"undone" => {
    // ... parsing and validation ...
    
    let index = number - 1;
    
    // ✅ NEW: Check if already pending
    if lines[index].contains("[ ]") {
        return Err("Task is already unmarked".into());
    }
    
    lines[index] = lines[index].replace("[x]", "[ ]");
    // ...
}
```

**Improved error message in `remove`:**

```rust
"remove" => {
    // ...
    if number == 0 || number > lines.len() {
        // ✅ More specific message
        return Err("This task doesn't exist or was already removed".into());
    }
    // ...
}
```

**Consistent filtering in all commands:**

```rust
// done, undone, and remove now all filter empty lines
let mut lines: Vec<String> = content
    .lines()
    .filter(|l| !l.trim().is_empty())  // ← Applied everywhere
    .map(|l| l.to_string())
    .collect();
```

**🧠 Key Concepts:**

#### Precondition validation

**What are preconditions?**

- Conditions that must be true before an operation
- Checked before performing the action
- Prevent invalid state transitions

**Example:**

```rust
// Precondition: task must be pending ([ ])
if lines[index].contains("[x]") {
    return Err("Task is already marked as completed".into());
}
// If we reach here, precondition is satisfied
lines[index] = lines[index].replace("[ ]", "[x]");
```

#### Specific vs generic error messages

**Before v0.4.2:**

```rust
// Generic - doesn't tell user what's wrong
if number > lines.len() {
    return Err("Invalid task number".into());
}
```

**After v0.4.2:**

```rust
// Specific - explains the actual problem
if lines[index].contains("[x]") {
    return Err("Task is already marked as completed".into());
}
```

**Why this matters:**

**User experience comparison:**

```bash
# Generic error
$ todo done 1
Error: Invalid task number
# User thinks: "But 1 is valid! What's wrong?"

# Specific error  
$ todo done 1
Error: Task is already marked as completed
# User thinks: "Oh, I already did this one!"
```

**Good error messages:**

1. ✅ Tell user what went wrong
2. ✅ Explain why it's wrong
3. ✅ (Ideally) Suggest how to fix it

#### State machine enforcement

**Valid transitions:**

```
[ ] ──done──> [x]
[x] ──undone──> [ ]
```

**Invalid transitions (now prevented):**

```
[x] ──done──> [x]  ❌ "Task is already marked as completed"
[ ] ──undone──> [ ]  ❌ "Task is already unmarked"
```

**This is defensive programming:**

- Assume user will make mistakes
- Validate before acting
- Provide helpful feedback

#### Consistency across commands

**Pattern established:**

All mutation commands now follow:

1. Parse arguments
2. Validate arguments (bounds)
3. Read file with empty line filtering
4. Validate preconditions (state)
5. Perform operation
6. Write file
7. Confirm to user

**This consistency:**

- ✅ Makes code predictable
- ✅ Easier to maintain
- ✅ Easier to add new commands

**🔗 Resources:**

- [Code v0.4.2](https://github.com/joaofelipegalvao/todo-cli/tree/v0.4.2)
- [Full diff](https://github.com/joaofelipegalvao/todo-cli/compare/v0.4.1...v0.4.2)

---

### v0.5.0 - Clear Command

**🎯 Goal:** Remove all tasks at once

**📦 Implementation:**

```rust
"clear" => {
    if fs::metadata("todos.txt").is_ok() {
        fs::remove_file("todos.txt")?;
        println!("✓ All tasks have been removed");
    } else {
        println!("No tasks to remove");
    }
}
```

**🧠 Key Concepts:**

#### `fs::metadata()` for file existence check

```rust
if fs::metadata("todos.txt").is_ok() {
    // File exists
} else {
    // File doesn't exist
}
```

**Why not just try to delete?**

```rust
// Without check
fs::remove_file("todos.txt")?;  
// If file doesn't exist → Error propagated to user
// User sees: "Error: No such file or directory" ❌
```

**With check:**

```rust
// With check
if fs::metadata("todos.txt").is_ok() {
    fs::remove_file("todos.txt")?;
    println!("✓ All tasks have been removed");
} else {
    println!("No tasks to remove");  // ✅ Friendly message
}
```

**User experience:**

```bash
# First time - file exists
$ todo clear
✓ All tasks have been removed

# Second time - already cleared
$ todo clear
No tasks to remove  # Not an error!
```

#### Why `.is_ok()` instead of `.unwrap()`?

```rust
fs::metadata("todos.txt").is_ok()  // Returns bool: true or false
```

**We don't care about the metadata**, only if the file exists.

**Alternatives:**

```rust
// ❌ Overkill
let metadata = fs::metadata("todos.txt")?;
// Why get metadata if we're just going to delete it?

// ✅ Simple
fs::metadata("todos.txt").is_ok()
```

#### `fs::remove_file()` behavior

```rust
fs::remove_file("todos.txt")?;
```

**What it does:**

- Deletes the file from filesystem
- **Permanent** - no undo!
- Returns `Result` - can fail if no permissions

**Different from clearing contents:**

```rust
// This would clear contents but keep file
fs::write("todos.txt", "")?;

// This deletes the file entirely
fs::remove_file("todos.txt")?;
```

**Why delete instead of clear?**

- Cleaner - file truly gone
- `list` command already handles missing file
- Consistent with "no tasks" state

#### Idempotent operations

**What's idempotency?**
> An operation that can be called multiple times with the same result

**clear is idempotent:**

```bash
todo clear  # Deletes file
todo clear  # No file to delete, but still succeeds
todo clear  # Still succeeds
```

**Why this matters:**

- ✅ No confusing errors
- ✅ Scripts can call it safely
- ✅ User doesn't have to check first

**🔗 Resources:**

- [Code v0.5.0](https://github.com/joaofelipegalvao/todo-cli/tree/v0.5.0)
- [Full diff](https://github.com/joaofelipegalvao/todo-cli/compare/v0.4.2...v0.5.0)

---

### v0.6.0 - Visual Interface with Colors

**🎯 Goal:** Add colorful visual hierarchy and progress statistics

**📦 Implementation:**

**Import colored crate:**

```rust
use colored::Colorize;
```

**Enhanced list display:**

```rust
"list" => match fs::read_to_string("todos.txt") {
    Ok(content) => {
        let valid_lines: Vec<&str> = content
            .lines()
            .filter(|l| !l.trim().is_empty())
            .collect();

        if valid_lines.is_empty() {
            println!("No tasks");
        } else {
            println!("\n📋 Tasks:\n");

            let mut completed = 0;
            let total = valid_lines.len();

            for (i, line) in valid_lines.iter().enumerate() {
                let number = format!("{}.", i + 1);

                if line.contains("[x]") {
                    let text = line.replace("[x]", "").trim().to_string();
                    
                    println!(
                        "{} {} {}",
                        number.dimmed(),
                        "✅".green(),
                        text.green().strikethrough()
                    );
                    completed += 1;
                } else {
                    let text = line.replace("[ ]", "").trim().to_string();
                    println!(
                        "{} {} {}",
                        number.dimmed(),
                        "⏳".yellow(),
                        text.bright_white()
                    );
                }
            }

            println!("\n{}", "─".repeat(30).dimmed());

            let percentage = (completed as f32 / total as f32 * 100.0) as u32;
            let stats = format!("{} of {} completed ({}%)", completed, total, percentage);

            if percentage == 100 {
                println!("{}", stats.green().bold());
            } else if percentage >= 50 {
                println!("{}", stats.yellow());
            } else {
                println!("{}", stats.red());
            }

            println!();
        }
    }
    Err(_) => {
        println!("No tasks");
    }
}
```

**🧠 Key Concepts:**

#### The `colored` crate

```rust
use colored::Colorize;

"text".red()            // Red text
"text".green()          // Green text
"text".yellow()         // Yellow text
"text".dimmed()         // Dim/gray text
"text".bold()           // Bold text
"text".strikethrough()  // Strike̶t̶h̶r̶o̶u̶g̶h̶
```

**These are chainable:**

```rust
"text".green().bold()           // Bold green
"text".red().strikethrough()    // Red with strikethrough
```

#### Visual hierarchy

```rust
number.dimmed()          // De-emphasize task numbers
"✅".green()              // Completed indicator
text.green().strikethrough()  // Completed task
"⏳".yellow()             // Pending indicator
text.bright_white()     // Pending task (prominent)
```

**Design principle:**

1. **Numbers are dimmed** - helper info, not main content
2. **Icons are colored** - quick visual scan
3. **Completed tasks strikethrough** - clearly done
4. **Pending tasks bright** - what needs attention

#### Progress statistics with percentage

```rust
let completed = 0;
let total = valid_lines.len();

// Count completed during loop
if line.contains("[x]") {
    completed += 1;
}

// Calculate percentage
let percentage = (completed as f32 / total as f32 * 100.0) as u32;
```

**Type conversions explained:**

```rust
completed as f32   // usize → f32 (for division)
total as f32       // usize → f32
* 100.0            // f32 result
as u32             // f32 → u32 (truncate decimals)
```

**Why this chain?**

```rust
// Without conversion
let percentage = completed / total * 100;
// 2 / 5 * 100 = 0 * 100 = 0  ❌ Integer division!

// With f32
let percentage = (2 as f32 / 5 as f32 * 100.0) as u32;
// 2.0 / 5.0 * 100.0 = 40.0 → 40  ✅ Correct!
```

#### Dynamic color based on progress

```rust
if percentage == 100 {
    println!("{}", stats.green().bold());     // 🎉 All done!
} else if percentage >= 50 {
    println!("{}", stats.yellow());            // 📊 Making progress
} else {
    println!("{}", stats.red());               // 🔴 Just started
}
```

**Psychology:**

- **Green** = Achievement, success, positive reinforcement
- **Yellow** = In progress, keep going
- **Red** = Attention needed, urgency

#### Separator line

```rust
println!("\n{}", "─".repeat(30).dimmed());
```

**Creates visual separation:**

```
1. ⏳ Task 1
2. ✅ Task 2
──────────────────────────────  ← separator
2 of 2 completed (100%)
```

**Why dimmed?**

- Visual break without being loud
- Guides eye but doesn't distract

#### Colored feedback messages

```rust
println!("{}", "✓ Task added".green());                 // Success
println!("{}", "✓ Task marked as completed".green());   // Success
println!("{}", "✓ Task unmarked".yellow());             // Neutral
println!("{}", "✓ Task removed".red());                  // Destructive
println!("{}", "✓ All tasks have been removed".red().bold());  // Very destructive
```

**Semantic coloring:**

- **Green** - Creating, completing (positive actions)
- **Yellow** - Undoing (neutral/reversible)
- **Red** - Deleting (destructive/permanent)
- **Red + Bold** - Very destructive (clear all)

**🔗 Resources:**

- [Code v0.6.0](https://github.com/joaofelipegalvao/todo-cli/tree/v0.6.0)
- [Full diff](https://github.com/joaofelipegalvao/todo-cli/compare/v0.5.0...v0.6.0)
- [colored crate docs](https://docs.rs/colored/)

---

### v0.7.0 - Advanced Filters (--pending, --done)

**🎯 Goal:** Filter tasks by status with helper function for display

**📦 Implementation:**

**Helper function extracted:**

```rust
fn display_tasks(tasks: &[&str], title: &str) {
    println!("\n📋 {}:\n", title);

    let mut completed = 0;
    let total = tasks.len();

    for (i, line) in tasks.iter().enumerate() {
        // ... display logic (same as v0.6.0) ...
    }

    // ... statistics ...
}
```

**Filter logic in list command:**

```rust
"list" => {
    let filter = if args.len() > 2 {
        args[2].as_str()
    } else {
        "all"
    };

    match fs::read_to_string("todos.txt") {
        Ok(content) => {
            let valid_lines: Vec<&str> = content
                .lines()
                .filter(|l| !l.trim().is_empty())
                .collect();

            if valid_lines.is_empty() {
                println!("No tasks");
                return Ok(());
            }

            match filter {
                "--pending" => {
                    let pending: Vec<&str> = valid_lines
                        .iter()
                        .filter(|line| line.contains("[ ]"))
                        .copied()
                        .collect();

                    if pending.is_empty() {
                        println!("No pending tasks");
                    } else {
                        display_tasks(&pending, "Pending tasks");
                    }
                }

                "--done" => {
                    let completed: Vec<&str> = valid_lines
                        .iter()
                        .filter(|line| line.contains("[x]"))
                        .copied()
                        .collect();

                    if completed.is_empty() {
                        println!("No completed tasks");
                    } else {
                        display_tasks(&completed, "Completed tasks");
                    }
                }

                "all" => {
                    display_tasks(&valid_lines, "Tasks");
                }

                _ => {
                    return Err(format!(
                        "Invalid filter: {}. Use --pending or --done",
                        filter
                    ).into());
                }
            }
        }
        Err(_) => {
            println!("No tasks");
        }
    }
}
```

*See LEARNING.md for complete detailed explanation of DRY principle, slices, and filter patterns.*

**🔗 Resources:**

- [Code v0.7.0](https://github.com/joaofelipegalvao/todo-cli/tree/v0.7.0)
- [Full diff](https://github.com/joaofelipegalvao/todo-cli/compare/v0.6.0...v0.7.0)

---

### v0.8.0 - Priority System + Priority Filters

**🎯 Goal:** Add three-level priority system with visual indicators and filtering

**📦 Key Implementations:**

**Priority extraction function:**

```rust
fn extract_priority(line: &str) -> (&str, String) {
    let without_checkbox = line
        .replace("[ ]", "")
        .replace("[x]", "")
        .trim()
        .to_string();

    if without_checkbox.contains("(high)") {
        let text = without_checkbox.replace("(high)", "").trim().to_string();
        ("high", text)
    } else if without_checkbox.contains("(low)") {
        let text = without_checkbox.replace("(low)", "").trim().to_string();
        ("low", text)
    } else {
        ("medium", without_checkbox)
    }
}
```

**Priority emoji function:**

```rust
fn priority_emoji(priority: &str) -> String {
    match priority {
        "high" => "🔴".red().to_string(),
        "low" => "🟢".green().to_string(),
        _ => "🟡".yellow().to_string(),
    }
}
```

**Adding tasks with priority:**

```rust
"add" => {
    // ... validation ...
    
    let line = match args.len() {
        3 => format!("[ ] {}", task),  // No flag = medium
        
        4 => {
            let flag = args[3].as_str();
            match flag {
                "--high" => format!("[ ] (high) {}", task),
                "--low" => format!("[ ] (low) {}", task),
                _ => return Err(format!("Invalid flag '{}'. Use --high or --low", flag).into()),
            }
        }
        
        _ => return Err("Usage: todo add <task> [--high | --low]. Only one flag allowed".into()),
    };
    
    // Write to file...
}
```

**Multi-flag parsing in list:**

```rust
"list" => {
    let mut status_filter = "all";
    let mut priority_filter: Option<&str> = None;

    for arg in &args[2..] {
        match arg.as_str() {
            "--pending" => {
                if status_filter != "all" {
                    return Err("Use only one status filter (--pending or --done)".into());
                }
                status_filter = "pending";
            }
            "--done" => {
                if status_filter != "all" {
                    return Err("Use only one status filter".into());
                }
                status_filter = "done";
            }
            "--high" => {
                if priority_filter.is_some() {
                    return Err("Use only one priority filter".into());
                }
                priority_filter = Some("high");
            }
            "--low" => {
                if priority_filter.is_some() {
                    return Err("Use only one priority filter".into());
                }
                priority_filter = Some("low");
            }
            _ => return Err(format!("Invalid filter: {}", arg).into()),
        }
    }
    
    // Apply filters sequentially...
    
    // First filter by status
    valid_lines = match status_filter {
        "pending" => valid_lines.iter().filter(|l| l.contains("[ ]")).copied().collect(),
        "done" => valid_lines.iter().filter(|l| l.contains("[x]")).copied().collect(),
        _ => valid_lines,
    };

    // Then filter by priority (if specified)
    if let Some(pri) = priority_filter {
        valid_lines = valid_lines
            .iter()
            .filter(|line| {
                let (priority, _) = extract_priority(line);
                priority == pri
            })
            .copied()
            .collect();
    }
    
    // Dynamic title based on filters
    let title = match (status_filter, priority_filter) {
        ("pending", Some("high")) => "High priority pending tasks",
        ("pending", Some("low")) => "Low priority pending tasks",
        ("pending", None) => "Pending tasks",
        ("done", Some("high")) => "High priority completed tasks",
        ("done", Some("low")) => "Low priority completed tasks",
        ("done", None) => "Completed tasks",
        (_, Some("high")) => "High priority",
        (_, Some("low")) => "Low priority",
        _ => "Tasks",
    };
}
```

*This version added 200+ lines implementing the complete priority system with parsing, filtering, validation, and display logic. See original explanation for detailed breakdown of Option<T>, pattern matching with tuples, and design decisions.*

**🔗 Resources:**

- [Code v0.8.0](https://github.com/joaofelipegalvao/todo-cli/tree/v0.8.0)
- [Full diff](https://github.com/joaofelipegalvao/todo-cli/compare/v0.7.0...v0.8.0)

---

### v0.9.0 - Priority Sorting

**🎯 Goal:** Sort tasks by priority (high → medium → low)

**📦 Key Implementation:**

**Priority mapping function:**

```rust
fn priority_order(pri: &str) -> u8 {
    match pri {
        "high" => 0,      // First
        "medium" => 1,    // Middle
        "low" => 2,       // Last
        _ => 3,           // Unknown (end)
    }
}
```

**Sort flag handling:**

```rust
"list" => {
    let mut status_filter = "all";
    let mut priority_filter: Option<&str> = None;
    let mut sort = false;  // ← NEW

    for arg in &args[2..] {
        match arg.as_str() {
            // ... other flags ...
            "--sort" => {
                if sort {
                    return Err("Use --sort only once.".into());
                }
                sort = true;
            }
            _ => return Err(format!("Invalid filter: {}", arg).into()),
        }
    }

    // ... filter tasks ...

    // Sort AFTER filtering (optimization!)
    if sort {
        valid_lines.sort_by(|a, b| {
            let (pri_a, _) = extract_priority(a);
            let (pri_b, _) = extract_priority(b);
            priority_order(pri_a).cmp(&priority_order(pri_b))
        });
    }
    
    // ... display ...
}
```

*Key insight: Sort happens AFTER filtering for performance (O(5 log 5) vs O(50 log 50)). See original explanation for detailed breakdown of .sort_by(), Ordering enum, and optimization rationale.*

**🔗 Resources:**

- [Code v0.9.0](https://github.com/joaofelipegalvao/todo-cli/tree/v0.9.0)
- [Full diff](https://github.com/joaofelipegalvao/todo-cli/compare/v0.8.0...v0.9.0)

---

### v1.0.0 - Search + Display Refactoring

**🎯 Goal:** Add search command and refactor display into atomic/orchestrated functions

**📦 Key Implementations:**

**Atomic rendering function:**

```rust
fn display_task(number: usize, line: &str) {
    let number_fmt = format!("{}.", number);
    let completed = line.contains("[x]");

    let (priority, text) = extract_priority(line);
    let emoji = priority_emoji(priority);

    if completed {
        println!(
            "{} {} {} {}",
            number_fmt.dimmed(),
            emoji,
            "✅".green(),
            text.green().strikethrough()
        );
    } else {
        println!(
            "{} {} {} {}",
            number_fmt.dimmed(),
            emoji,
            "⏳".yellow(),
            text.bright_white()
        );
    }
}
```

**Orchestrated rendering function (renamed):**

```rust
fn display_lists(tasks: &[&str], title: &str) {
    println!("\n📋 {}:\n", title);

    let mut completed = 0;
    let total = tasks.len();

    for (i, line) in tasks.iter().enumerate() {
        display_task(i + 1, line);  // ← Uses atomic function

        if line.contains("[x]") {
            completed += 1;
        }
    }

    // ... statistics ...
}
```

**Search command:**

```rust
"search" => {
    if args.len() < 3 {
        return Err("Usage: todo search <term>".into());
    }

    let term = &args[2];

    match fs::read_to_string("todos.txt") {
        Ok(content) => {
            let valid_lines: Vec<&str> = content
                .lines()
                .filter(|l| !l.trim().is_empty())
                .collect();

            if valid_lines.is_empty() {
                println!("No tasks");
                return Ok(());
            }

            let mut results: Vec<(usize, &str)> = Vec::new();

            for (i, line) in valid_lines.iter().enumerate() {
                if line.to_lowercase().contains(&term.to_lowercase()) {
                    results.push((i + 1, line));  // Keep original number!
                }
            }

            if results.is_empty() {
                println!("No results for '{}'", term);
            } else {
                println!("\n📋 Results for \"{}\":\n", term);

                for (number, line) in &results {
                    display_task(*number, line);  // ← Uses atomic function
                }

                println!("\n{} result(s) found\n", results.len());
            }
        }
        Err(_) => {
            println!("No tasks");
        }
    }
}
```

**🧠 Key Design:**

**Atomic vs Orchestrated:**

- `display_task()` - Renders ONE task, maintains original numbering
- `display_lists()` - Renders LIST with statistics, renumbers sequentially

**Why search uses atomic function:**

- Must keep original task numbers (so user can run `todo done 5`)
- No statistics needed (partial results)
- Different semantic meaning than full list

This is **mature CLI architecture** - not duplication, but proper separation of concerns.

**🔗 Resources:**

- [Code v1.0.0](https://github.com/joaofelipegalvao/todo-cli/tree/v1.0.0)
- [Full diff](https://github.com/joaofelipegalvao/todo-cli/compare/v0.9.0...v1.0.0)

---

## Concepts Learned

### File Manipulation

- `OpenOptions` with `.create()` and `.append()` to add without overwriting
- `writeln!` macro for formatted writing
- `fs::read_to_string()` for complete file reading
- `fs::write()` to overwrite entire file
- `fs::remove_file()` to delete files
- `fs::metadata()` to check existence without opening

### Strings and Collections

- `enumerate()` to get indices + values in loops
- `parse()` for string → number conversion with validation
- `.map().collect()` to transform iterators
- `.replace()` for text substitution
- `.contains()` for string searching
- `.trim()` to remove whitespace
- `.to_string()` to solve lifetimes (`&str` → `String`)
- `.join()` to concatenate with separator
- `.filter()` to select elements
- `.copied()` to convert `&&str` → `&str` in iterators
- `Vec::remove()` to delete by index
- `.repeat()` for repeated strings
- Slices `&[&str]` to pass data slices without copying
- Tuples `(T, U)` to return multiple values from functions
- `.sort_by()` for custom sorting with comparator
- `Ordering` enum for type-safe comparisons (Less, Equal, Greater)
- `.cmp()` to compare orderable values

### Control Flow and Errors

- Pattern matching with `match` for subcommands
- Nested match for multi-level decisions
- Pattern matching with tuples `(a, b)` to combine conditions
- Error handling with `?` operator (automatic propagation)
- `Result<T, E>` for functions that can fail
- `Box<dyn Error>` for generic errors
- `if let` for simplified pattern matching
- Input validation and preconditions
- Conflicting flags validation (fail fast)
- Specific, educational error messages
- `Option<T>` for optional values (avoids "magic values")

### CLI and UX

- `env::args()` to capture arguments
- Subcommands with pattern matching
- Optional flags (`--pending`, `--done`, `--high`, `--low`)
- Parsing arguments with multiple flags
- Filter combination (status + priority)
- Argument validation (quantity, type, state)
- `println!` vs `eprintln!` (stdout vs stderr)
- `process::exit()` for exit codes
- Visual hierarchy with colors and formatting
- Immediate feedback with semantic colors
- Visual breathing room (whitespace matters)
- Smart defaults (medium as default)
- Dynamic titles based on context
- User error prevention

### Design and Colors

- `colored` crate for cross-platform colors
- `.dimmed()`, `.bold()`, `.strikethrough()` for formatting
- Semantic colors (green = success, red = attention)
- Color psychology ( <img src="../assets/icons/circle-high.svg" width="11" > red = urgent, <img src="../assets/icons/circle-medium.svg" width="11"> yellow = normal, <img src="../assets/icons/circle-low.svg" width="11"> green = low)
- Visual priority system with emojis
- Visual hierarchy (dimmed numbers, highlighted content)
- Multiple signals (color + icon + strikethrough) for accessibility
- `as f32` conversion for percentage calculations
- `as u32` to truncate decimals
- Positive reinforcement (always green for completed)
- Reduced visual pollution (priority only on pending)

### Functions and Organization

- Helper functions to avoid code duplication (DRY)
- Parameters with slices (`&[&str]`) for efficiency
- Code reuse with specialized functions
- Separation of responsibilities (parsing vs display)
- Parsing functions (`extract_priority`)
- Tuple returns for multiple values
- Transformation pipeline (filters in sequence)
- Visual logic modularization (`priority_emoji`)
- Mapping functions (`priority_order`) for concept → number conversion
- Appropriate type choices (`u8` vs `i32`) based on semantics
- Atomic functions (`display_task`) for unit rendering
- Orchestrated functions (`display_tasks`) for complete display with statistics
- Clear separation: data parsing vs visual rendering
- Mature CLI design without code duplication

### Debug and Quality

- Finding bugs through manual testing
- Using `eprintln!` for debug prints
- File investigation with `cat` and `od`
- Precondition validation (avoid invalid states)
- Edge case thinking (empty file, invalid indices)
- Iterative refactoring without breaking functionality
- Consistency across commands (filter in all)
- Pipeline optimization (filter before sorting)
- Complexity analysis (Big-O) for performance decisions
- YAGNI principle (You Aren't Gonna Need It) - don't add unnecessary complexity
- Opt-in complexity - complex features are optional

### Lifetimes and Ownership

- Lifetime problem with `.trim()` returning `&str`
- Solution with `.to_string()` to create owned `String`
- Difference between temporary reference and owned value
- Compiler detecting invalid reference usage
- `.copied()` to work with double references (`&&str`)

---

## Design Decisions

### Why 3 Priority Levels?

**Alternatives considered:**

- **2 levels** (high/normal)
  - ❌ Doesn't capture nuances
  - ❌ "What about important but not urgent?"

- **5+ levels** (critical/high/medium/low/trivial)
  - ❌ Decision paralysis
  - ❌ "Is this high or critical?"
  - ❌ User spends time categorizing instead of doing

**Chosen: 3 levels (high/medium/low)**

- ✅ Traffic light rule (universal convention)
- ✅ Cognitively simple
- ✅ Forces real prioritization

**Psychology behind it:**
> "If everything is a priority, nothing is a priority"

With 3 levels, you **must choose** what really matters.

---

### Why Sort AFTER Filtering?

**Wrong order:**

```rust
sort(50 tasks)     // O(50 log 50) ≈ 280 operations
filter to 5 tasks  // Wasted work!
```

**Correct order:**

```rust
filter 50 → 5      // O(50)
sort 5             // O(5 log 5) ≈ 12 operations  
// 23x faster!
```

**Performance matters**, even for small datasets. Good habits scale.

---

### Why `(high)` Format?

**Considered:**

```
[high][ ] Task       (prefix)
[ ]:high: Task       (inline)
high|[ ]|Task        (separator)
```

**Chosen: `[ ] (high) Task`**

- ✅ Human-readable (open `todos.txt` in any editor)
- ✅ Easy to parse (`.contains("(high)")`)
- ✅ Doesn't pollute visually (parentheses are subtle)
- ✅ Extensible (could add `(urgent)`, `(someday)`)

**Trade-off:**

- ❌ More verbose than `[h]`
- ✅ But clarity > brevity for user data

---

### Why Separate `display_task()` and `display_tasks()`?

**Could have done:**

```rust
// One function for everything
fn display_everything(tasks: &[&str], mode: &str) {
    if mode == "search" { /* ... */ }
    else if mode == "list" { /* ... */ }
}
```

**Why separation is better:**

- ✅ **Single Responsibility** - each function does one thing
- ✅ **Reusability** - atomic function works anywhere
- ✅ **Testability** - easier to test small functions
- ✅ **Maintainability** - changes don't cascade

**This is mature software design.**

---

### Why `medium` as Default Priority?

**Could have forced:**

```bash
todo add "Task"
# Error: Specify priority with --high or --low
```

**Why default is better:**

- ✅ **Least friction** - most tasks are "normal"
- ✅ **Quick capture** - "Just want to add it quickly!"
- ✅ **Opt-in complexity** - complexity only when needed

**Design principle:**
> "Make the common case fast, the uncommon case possible"

---

## Conclusion

This project taught me:

1. **Rust fundamentals** through practical application
2. **CLI design patterns** that feel natural to users
3. **Incremental development** - each version builds on previous
4. **Error handling** that's helpful, not frustrating
5. **Visual design** that reduces cognitive load
6. **Code organization** that scales without duplication

**Most importantly:** Learning is a process. Each version had bugs, inefficiencies, and room for improvement. That's expected and valuable.

**The journey is the lesson.** 🦀
