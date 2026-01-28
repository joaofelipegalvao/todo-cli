use std::fs;
use std::io::Write;
use std::{env, error::Error, fs::OpenOptions, process};

use colored::{ColoredString, Colorize};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn priority_order(pri: &str) -> u8 {
    match pri {
        "high" => 0,
        "medium" => 1,
        "low" => 2,
        _ => 3,
    }
}

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

fn priority_emoji(priority: &str) -> ColoredString {
    match priority {
        "high" => "".red(),
        "low" => "".green(),
        _ => "".yellow(),
    }
}

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
            "󰄵".green(),
            text.green().strikethrough()
        );
    } else {
        println!(
            "{} {} {} {}",
            number_fmt.dimmed(),
            emoji,
            "".yellow(),
            text.bright_white()
        );
    }
}

fn display_lists(tasks: &[&str], title: &str) {
    println!("\n {}:\n", title);

    let mut completed = 0;
    let total = tasks.len();

    for (i, line) in tasks.iter().enumerate() {
        display_task(i + 1, line);

        if line.contains("[x]") {
            completed += 1;
        }
    }

    println!("\n{}", "─".repeat(30).dimmed());

    let percentage = if total > 0 {
        (completed as f32 / total as f32 * 100.0) as u32
    } else {
        0
    };

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

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(
            "Usage: todo <command> [arguments]\nCommands: add, list [--pending | --done], done, undone, remove, clear"
                .into(),
        );
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                return Err("Usage: todo add <task> [--high | --low]".into());
            }

            let task = &args[2];

            let line = match args.len() {
                3 => format!("[ ] {}", task),

                4 => {
                    let flag = args[3].as_str();
                    match flag {
                        "--high" => format!("[ ] (high) {}", task),
                        "--low" => format!("[ ] (low) {}", task),

                        _ => {
                            return Err(
                                format!("Invalid flag '{}'. Use --high or --low", flag).into()
                            );
                        }
                    }
                }

                _ => {
                    return Err(
                        "Usage: todo add <task> [--high | --low]. Only one flag is allowed".into(),
                    );
                }
            };

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("todos.txt")?;

            writeln!(file, "{}", line)?;

            println!("{}", "✓ Task added".green());
        }

        "list" => {
            let mut status_filter = "all";
            let mut priority_filter: Option<&str> = None;
            let mut sort = false;

            for arg in &args[2..] {
                match arg.as_str() {
                    "--pending" => {
                        if status_filter != "all" {
                            return Err(
                                "Use only one status filter (--pending or --done).\nValid example: todo list --pending --high".into()
                            );
                        }
                        status_filter = "pending";
                    }
                    "--done" => {
                        if status_filter != "all" {
                            return Err("Use only one status filter (--done or --pending).".into());
                        }
                        status_filter = "done";
                    }
                    "--high" => {
                        if priority_filter.is_some() {
                            return Err("Use only one priority filter (--high or --low)".into());
                        }
                        priority_filter = Some("high");
                    }
                    "--low" => {
                        if priority_filter.is_some() {
                            return Err("Use only one priority filter (--low or --high)".into());
                        }

                        priority_filter = Some("low");
                    }
                    "--sort" => {
                        if sort {
                            return Err("Use --sort only once.".into());
                        }
                        sort = true;
                    }
                    _ => return Err(format!("Invalid filter: {}", arg).into()),
                }
            }

            match fs::read_to_string("todos.txt") {
                Ok(content) => {
                    let mut valid_lines: Vec<&str> =
                        content.lines().filter(|l| !l.trim().is_empty()).collect();

                    if valid_lines.is_empty() {
                        println!("No tasks");
                        return Ok(());
                    }

                    valid_lines = match status_filter {
                        "pending" => valid_lines
                            .iter()
                            .filter(|l| l.contains("[ ]"))
                            .copied()
                            .collect(),
                        "done" => valid_lines
                            .iter()
                            .filter(|l| l.contains("[x]"))
                            .copied()
                            .collect(),
                        _ => valid_lines,
                    };

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

                    if valid_lines.is_empty() {
                        println!("No tasks found with these filters");
                        return Ok(());
                    }

                    if sort {
                        valid_lines.sort_by(|a, b| {
                            let (pri_a, _) = extract_priority(a);
                            let (pri_b, _) = extract_priority(b);
                            priority_order(pri_a).cmp(&priority_order(pri_b))
                        });
                    }

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

                    display_lists(&valid_lines, title);
                }

                Err(_) => {
                    println!("No tasks");
                }
            }
        }

        "done" => {
            if args.len() < 3 {
                return Err("Usage: todo done <number>".into());
            }

            let number: usize = args[2].parse()?;

            let content = fs::read_to_string("todos.txt")?;

            let mut lines: Vec<String> = content
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| l.to_string())
                .collect();

            if number == 0 || number > lines.len() {
                return Err("Invalid task number".into());
            }

            let index = number - 1;

            if lines[index].contains("[x]") {
                return Err("Task is already marked as completed".into());
            }

            lines[index] = lines[index].replace("[ ]", "[x]");

            fs::write("todos.txt", lines.join("\n") + "\n")?;

            println!("{}", "✓ Task marked as completed".green());
        }

        "undone" => {
            if args.len() < 3 {
                return Err("Usage: todo undone <number>".into());
            }

            let number: usize = args[2].parse()?;

            let content = fs::read_to_string("todos.txt")?;

            let mut lines: Vec<String> = content
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| l.to_string())
                .collect();

            if number == 0 || number > lines.len() {
                return Err("Invalid task number".into());
            }

            let index = number - 1;

            if lines[index].contains("[ ]") {
                return Err("Task is already unmarked".into());
            }

            lines[index] = lines[index].replace("[x]", "[ ]");

            fs::write("todos.txt", lines.join("\n") + "\n")?;

            println!("{}", "✓ Task unmarked".yellow());
        }

        "search" => {
            if args.len() < 3 {
                return Err("Usage: todo search <term>".into());
            }

            let term = &args[2];

            match fs::read_to_string("todos.txt") {
                Ok(content) => {
                    let valid_lines: Vec<&str> =
                        content.lines().filter(|l| !l.trim().is_empty()).collect();

                    if valid_lines.is_empty() {
                        println!("No tasks");
                        return Ok(());
                    }

                    let mut results: Vec<(usize, &str)> = Vec::new();

                    for (i, line) in valid_lines.iter().enumerate() {
                        if line.to_lowercase().contains(&term.to_lowercase()) {
                            results.push((i + 1, line));
                        }
                    }

                    if results.is_empty() {
                        println!("No results for '{}'", term);
                    } else {
                        println!("\n Results for \"{}\":\n", term);

                        for (number, line) in &results {
                            display_task(*number, line);
                        }

                        println!("\n{} result(s) found\n", results.len());
                    }
                }

                Err(_) => {
                    println!("No tasks");
                }
            }
        }

        "remove" => {
            if args.len() < 3 {
                return Err("Usage: todo remove <number>".into());
            }

            let number: usize = args[2].parse()?;

            let content = fs::read_to_string("todos.txt")?;

            let mut lines: Vec<String> = content
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| l.to_string())
                .collect();

            if number == 0 || number > lines.len() {
                return Err("This task doesn't exist or was already removed".into());
            }

            let index = number - 1;
            lines.remove(index);

            fs::write("todos.txt", lines.join("\n") + "\n")?;

            println!("{}", "✓ Task removed".red());
        }

        "clear" => {
            if fs::metadata("todos.txt").is_ok() {
                fs::remove_file("todos.txt")?;
                println!("{}", "✓ All tasks have been removed".red().bold());
            } else {
                println!("No tasks to remove");
            }
        }

        _ => {
            return Err(format!("Unknown command: {}", command).into());
        }
    }

    Ok(())
}

