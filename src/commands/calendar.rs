//! Handler for `todo calendar [MONTH] [YEAR]`.

use std::collections::HashMap;

use anyhow::Result;
use chrono::{Datelike, Local, NaiveDate};

use crate::config::Config;
use crate::render::calendar::{DayInfo, display_calendar};
use crate::services::holidays::HolidayCache;
use crate::storage::Storage;

pub fn execute(storage: &impl Storage, month: Option<u32>, year: Option<i32>) -> Result<()> {
    let today = Local::now().naive_local().date();
    let target_month = month.unwrap_or_else(|| today.month());
    let target_year = year.unwrap_or_else(|| today.year());

    if !(1..=12).contains(&target_month) {
        anyhow::bail!("Invalid month: {}. Must be between 1 and 12.", target_month);
    }

    let cfg = Config::load().unwrap_or_default();

    // Load holiday cache — silently fall back to empty if not configured or unavailable
    let holidays = if cfg.holidays_locale != "none" && !cfg.holidays_locale.is_empty() {
        HolidayCache::load(&cfg.holidays_locale, target_year).unwrap_or_default()
    } else {
        HolidayCache::default()
    };

    let (all_tasks, all_projects, _, _) = storage.load_all_with_resources()?;
    let tasks: Vec<_> = all_tasks.iter().filter(|t| !t.is_deleted()).collect();
    let projects: Vec<_> = all_projects.iter().filter(|p| !p.is_deleted()).collect();

    // ── Density map ───────────────────────────────────────────────────────────
    let mut density: HashMap<NaiveDate, DayInfo> = HashMap::new();

    for task in &tasks {
        if let Some(due) = task.due_date {
            let e = density.entry(due).or_default();
            e.count += 1;
            if due < today && !task.completed {
                e.overdue = true;
            }
        }
    }
    for project in &projects {
        if let Some(due) = project.due_date {
            let e = density.entry(due).or_default();
            e.count += 1;
            if due < today && !project.completed {
                e.overdue = true;
            }
        }
    }

    // ── Holiday map ───────────────────────────────────────────────────────────
    // Mark all days in the three visible months that are holidays
    for offset_month in [-1i32, 0, 1] {
        let (y, m) = add_months(target_year, target_month, offset_month);
        let days_in_month = days_in_month(y, m);
        for day in 1..=days_in_month {
            if let Some(date) = NaiveDate::from_ymd_opt(y, m, day)
                && holidays.is_holiday(date)
            {
                density.entry(date).or_default().holiday = true;
            }
        }
    }

    display_calendar(today, target_month, target_year, &density);

    Ok(())
}

fn add_months(year: i32, month: u32, offset: i32) -> (i32, u32) {
    let total = (month as i32 - 1) + offset;
    let y = year + total.div_euclid(12);
    let m = (total.rem_euclid(12) + 1) as u32;
    (y, m)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let (ny, nm) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    NaiveDate::from_ymd_opt(ny, nm, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
        .day()
}
