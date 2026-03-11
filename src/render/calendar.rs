//! Calendar rendering for `todo calendar`.
use std::collections::HashMap;

use chrono::{Datelike, Duration, NaiveDate};
use colored::Colorize;

// ── Layout ────────────────────────────────────────────────────────────────────

const MONTH_WIDTH: usize = 26;

// ── DayInfo ───────────────────────────────────────────────────────────────────

/// Aggregated due/overdue information for a single calendar day.
#[derive(Default)]
pub struct DayInfo {
    pub count: usize,
    pub overdue: bool,
    pub holiday: bool,
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Renders three months side-by-side to stdout.
pub fn display_calendar(
    today: NaiveDate,
    target_month: u32,
    target_year: i32,
    density: &HashMap<NaiveDate, DayInfo>,
) {
    let months = [
        prev_month(target_year, target_month),
        (target_year, target_month),
        next_month(target_year, target_month),
    ];

    let grids: Vec<Vec<String>> = months
        .iter()
        .map(|&(y, m)| render_month(y, m, today, density))
        .collect();

    print_side_by_side(&grids);
    print_legend();
    println!();
}

// ── Legend ────────────────────────────────────────────────────────────────────

fn print_legend() {
    println!();
    print!("  Legend: ");
    print!("{}", "today".black().on_cyan());
    print!(", ");
    print!("{}", "due".black().on_yellow());
    print!(", ");
    print!("{}", "due-today".black().bold().on_yellow());
    print!(", ");
    print!("{}", "overdue".black().bold().on_red());
    print!(", ");
    print!("{}", "holiday".black().on_green());
    print!(", ");
    print!("{}", "weekend".black().on_blue());
    print!(", ");
    print!("{}", "weeknum".cyan());
    println!();
}

// ── Month renderer ────────────────────────────────────────────────────────────

fn render_month(
    year: i32,
    month: u32,
    today: NaiveDate,
    density: &HashMap<NaiveDate, DayInfo>,
) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();
    let is_target = today.year() == year && today.month() == month;

    // Title
    let title = format!("{} {}", month_name(month), year);
    rows.push(if is_target {
        title.bright_white().bold().to_string()
    } else {
        title.normal().to_string()
    });

    // Weekday header
    rows.push(format!("    {}", "Su Mo Tu We Th Fr Sa".dimmed()));

    // Week rows
    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let last = last_day_of_month(year, month);
    let offset = sun_offset(first);
    let total = last.day() as usize;
    let nrows = (offset + total).div_ceil(7);

    for row in 0..nrows {
        let first_real: Option<NaiveDate> = (0..7).find_map(|col| {
            let cell = row * 7 + col;
            if cell >= offset && cell < offset + total {
                NaiveDate::from_ymd_opt(year, month, (cell - offset + 1) as u32)
            } else {
                None
            }
        });

        let wk = first_real
            .map(|d| {
                let mon = d - Duration::days(d.weekday().num_days_from_monday() as i64);
                mon.iso_week().week()
            })
            .unwrap_or(0);

        let mut line = format!("{}", format!("{:>2}  ", wk).cyan());

        for col in 0..7 {
            let cell = row * 7 + col;
            let sep = if col < 6 { " " } else { "" };
            let is_weekend = col == 0 || col == 6;

            if cell < offset || cell >= offset + total {
                line.push_str("  ");
                line.push_str(sep);
                continue;
            }

            let day = (cell - offset + 1) as u32;
            let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
            let info = density.get(&date);
            let overdue = info.map(|i| i.overdue).unwrap_or(false);
            let has_due = info.map(|i| i.count > 0).unwrap_or(false);
            let is_holiday = info.map(|i| i.holiday).unwrap_or(false);
            let is_today = date == today;
            let is_past = date < today;

            let s = format!("{:>2}", day);

            // past/future < weekend < holiday < today < due < due-today < overdue
            let colored = if overdue {
                s.black().bold().on_red().to_string()
            } else if has_due && is_today {
                s.black().bold().on_yellow().to_string()
            } else if has_due {
                s.black().on_yellow().to_string()
            } else if is_today {
                s.black().on_cyan().to_string()
            } else if is_holiday {
                s.black().on_green().to_string()
            } else if is_weekend {
                s.black().on_blue().to_string()
            } else if is_past {
                s.bright_black().to_string()
            } else {
                s.normal().to_string()
            };

            line.push_str(&colored);
            line.push_str(sep);
        }

        rows.push(line);
    }

    rows
}

// ── Side-by-side printer ──────────────────────────────────────────────────────

fn print_side_by_side(grids: &[Vec<String>]) {
    let max_rows = grids.iter().map(|g| g.len()).max().unwrap_or(0);
    let gap = "    ";

    for row in 0..max_rows {
        let mut line = String::from("  ");

        for (ci, grid) in grids.iter().enumerate() {
            let cell = grid.get(row).map(String::as_str).unwrap_or("");
            let vlen = visible_len(cell);

            if row == 0 {
                let grid_width = MONTH_WIDTH - 4;
                let pad_total = grid_width.saturating_sub(vlen);
                let pad_left = 4 + pad_total / 2;
                let pad_right = pad_total - pad_total / 2;
                line.push_str(&" ".repeat(pad_left));
                line.push_str(cell);
                line.push_str(&" ".repeat(pad_right));
            } else if row == 1 {
                line.push_str(&" ".repeat(MONTH_WIDTH));
            } else {
                if !cell.is_empty() {
                    line.push_str(cell);
                }
                if vlen < MONTH_WIDTH {
                    line.push_str(&" ".repeat(MONTH_WIDTH - vlen));
                }
            }

            if ci < grids.len() - 1 {
                line.push_str(gap);
            }
        }

        println!("{}", line);
    }
}

/// Count visible (non-ANSI) characters in a string.
fn visible_len(s: &str) -> usize {
    let mut len = 0usize;
    let mut in_esc = false;
    for ch in s.chars() {
        match (in_esc, ch) {
            (false, '\x1b') => in_esc = true,
            (true, 'm') => in_esc = false,
            (false, _) => len += 1,
            _ => {}
        }
    }
    len
}

// ── Date helpers ──────────────────────────────────────────────────────────────

fn sun_offset(d: NaiveDate) -> usize {
    (d.weekday().num_days_from_monday() as usize + 1) % 7
}

fn prev_month(year: i32, month: u32) -> (i32, u32) {
    if month == 1 {
        (year - 1, 12)
    } else {
        (year, month - 1)
    }
}

fn next_month(year: i32, month: u32) -> (i32, u32) {
    if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    }
}

fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
    let (ny, nm) = next_month(year, month);
    NaiveDate::from_ymd_opt(ny, nm, 1).unwrap() - Duration::days(1)
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "???",
    }
}
