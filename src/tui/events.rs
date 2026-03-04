//! Keyboard event handling for the TUI.
//!
//! # Keybinds — Normal mode
//! | Key      | Action                                  |
//! |----------|-----------------------------------------|
//! | `j/k`    | Navigate list                           |
//! | `↑/↓`    | Scroll details panel                    |
//! | `d`      | Toggle done / undone                    |
//! | `e`      | Open edit form                          |
//! | `/`      | Enter search mode                       |
//! | `Tab`    | Cycle status filter                     |
//! | `p`      | Cycle priority filter                   |
//! | `S`      | Toggle stats panel                      |
//! | `x`      | Enter confirm-delete mode               |
//! | `q`/Esc  | Quit                                    |
//!
//! # Keybinds — EditForm mode
//! | Key         | Action                               |
//! |-------------|--------------------------------------|
//! | `Tab`       | Next field                           |
//! | `Shift+Tab` | Previous field                       |
//! | `←/→`       | Cycle priority (on Priority field)   |
//! | `Enter`     | Save all changes                     |
//! | `Esc`       | Cancel edit                          |
//! | `Backspace` | Delete last char (text fields)       |
//! | char        | Append to focused text field         |

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::models::Task;
use crate::storage::Storage;
use crate::tag_normalizer::{collect_existing_tags, normalize_tags};

use super::app::{App, EditField, FocusedPanel, Mode};

pub fn handle(app: &mut App, storage: &impl Storage) -> Result<bool> {
    let ev = event::read()?;
    let Event::Key(key) = ev else {
        return Ok(false);
    };
    if key.kind != KeyEventKind::Press {
        return Ok(false);
    }

    match app.mode {
        Mode::Normal => {
            // Tab always toggles focus regardless of which panel is active
            if key.code == KeyCode::Tab {
                app.focused_panel = app.focused_panel.toggle();
                app.details_scroll = 0;
                app.status_msg = None;
                return Ok(false);
            }
            match app.focused_panel {
                FocusedPanel::Left => handle_left(app, storage, key.code, key.modifiers),
                FocusedPanel::Right => handle_right(app, key.code),
            }
        }
        Mode::ConfirmDelete => handle_confirm(app, storage, key.code),
        Mode::ConfirmClearAll => handle_clear_all(app, storage, key.code),
        Mode::Search => handle_search(app, key.code),
        Mode::EditForm => handle_edit_form(app, storage, key.code, key.modifiers),
        Mode::AddForm => handle_add_form(app, storage, key.code, key.modifiers),
        Mode::Help => handle_help(app, key.code),
    }
}

// ── normal mode ───────────────────────────────────────────────────────────────

fn handle_left(
    app: &mut App,
    storage: &impl Storage,
    key: KeyCode,
    _mods: KeyModifiers,
) -> Result<bool> {
    match key {
        KeyCode::Char('j') => {
            app.move_down();
            app.status_msg = None;
        }
        KeyCode::Char('k') => {
            app.move_up();
            app.status_msg = None;
        }
        KeyCode::Down => app.scroll_details_down(),
        KeyCode::Up => app.scroll_details_up(),

        // Jump to first / last
        KeyCode::Char('g') => {
            app.selected = 0;
            app.details_scroll = 0;
            app.status_msg = None;
        }
        KeyCode::Char('G') => {
            app.selected = app.filtered_indices.len().saturating_sub(1);
            app.details_scroll = 0;
            app.status_msg = None;
        }

        // Help popup
        KeyCode::Char('?') => {
            app.help_selected = 0;
            app.mode = Mode::Help;
            app.status_msg = None;
        }

        KeyCode::Char('d') => toggle_done(app, storage)?,
        KeyCode::Char('e') => app.open_edit_form(),
        KeyCode::Char('a') => app.open_add_form(),

        KeyCode::Char('/') => {
            app.input = String::new();
            app.mode = Mode::Search;
            app.status_msg = None;
            app.refilter();
        }

        KeyCode::Char('f') => {
            app.cycle_status_filter();
            app.status_msg = Some(format!("Filter: {}", app.list_filter.label()));
        }

        KeyCode::Char('p') => {
            app.cycle_priority_filter();
            app.status_msg = Some(format!("Priority: {}", app.priority_filter.label()));
        }

        // Cycle right panel tabs even from left panel
        KeyCode::Char(']') => {
            app.right_panel = app.right_panel.next();
            app.details_scroll = 0;
            app.status_msg = None;
        }
        KeyCode::Char('[') => {
            app.right_panel = app.right_panel.prev();
            app.details_scroll = 0;
            app.status_msg = None;
        }

        KeyCode::Char('X') => {
            let count = app.filtered_indices.len();
            if count > 0 {
                app.mode = Mode::ConfirmClearAll;
                app.status_msg = Some(format!("Clear all {} tasks? [y/n]", count));
            }
        }

        KeyCode::Char('x') => {
            if !app.tasks.is_empty() {
                app.mode = Mode::ConfirmDelete;
                let preview = app
                    .selected_task()
                    .map(|t| truncate_str(&t.text, 30))
                    .unwrap_or_default();
                app.status_msg = Some(format!("Delete \"{}\"? [y/n]", preview));
            }
        }

        KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
        _ => {}
    }
    Ok(false)
}

// ── right panel (focused) ─────────────────────────────────────────────────────

fn handle_right(app: &mut App, key: KeyCode) -> Result<bool> {
    match key {
        // Scroll the right panel content
        KeyCode::Char('j') | KeyCode::Down => app.scroll_details_down(),
        KeyCode::Char('k') | KeyCode::Up => app.scroll_details_up(),

        // Cycle tabs
        KeyCode::Char(']') => {
            app.right_panel = app.right_panel.next();
            app.details_scroll = 0;
        }
        KeyCode::Char('[') => {
            app.right_panel = app.right_panel.prev();
            app.details_scroll = 0;
        }

        // Help popup accessible from both panels
        KeyCode::Char('?') => {
            app.help_selected = 0;
            app.mode = Mode::Help;
            app.status_msg = None;
        }

        KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
        _ => {}
    }
    Ok(false)
}

// ── confirm-delete mode ───────────────────────────────────────────────────────

fn handle_confirm(app: &mut App, storage: &impl Storage, key: KeyCode) -> Result<bool> {
    match key {
        KeyCode::Char('y') | KeyCode::Enter => {
            delete_selected(app, storage)?;
            app.mode = Mode::Normal;
        }
        KeyCode::Char('n') | KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.status_msg = Some("Deletion cancelled.".into());
        }
        _ => {}
    }
    Ok(false)
}

// ── confirm-clear-all mode ────────────────────────────────────────────────────

fn handle_clear_all(app: &mut App, storage: &impl Storage, key: KeyCode) -> Result<bool> {
    match key {
        KeyCode::Char('y') | KeyCode::Enter => {
            let count = app.filtered_indices.len();
            // Delete from highest visible id to lowest to avoid index shifting
            for vis_id in (1..=count).rev() {
                let _ = crate::commands::remove::execute_silent(storage, vis_id);
            }
            app.reload(storage)?;
            app.mode = Mode::Normal;
            app.status_msg = Some(format!("Cleared {} task(s).", count));
        }
        KeyCode::Char('n') | KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.status_msg = Some("Cancelled.".into());
        }
        _ => {}
    }
    Ok(false)
}

// ── search mode ───────────────────────────────────────────────────────────────

fn handle_search(app: &mut App, key: KeyCode) -> Result<bool> {
    match key {
        KeyCode::Esc => {
            app.input = String::new();
            app.mode = Mode::Normal;
            app.refilter();
            app.status_msg = None;
        }
        KeyCode::Enter => {
            let query = app.input.clone();
            app.mode = Mode::Normal;
            app.status_msg = if query.is_empty() {
                None
            } else {
                Some(format!(
                    "Search: \"{}\" — {} result(s)",
                    query,
                    app.filtered_indices.len()
                ))
            };
        }
        KeyCode::Backspace => {
            app.input.pop();
            app.refilter();
        }
        KeyCode::Char(c) => {
            app.input.push(c);
            app.selected = 0;
            app.refilter();
        }
        _ => {}
    }
    Ok(false)
}

// ── help mode ─────────────────────────────────────────────────────────────────

fn handle_help(app: &mut App, key: KeyCode) -> Result<bool> {
    // Number of selectable (non-section) entries — keep in sync with help_entries() in ui.rs
    const SELECTABLE: usize = 14;
    match key {
        KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => {
            app.mode = Mode::Normal;
        }
        KeyCode::Char('j') | KeyCode::Down => {
            app.help_selected = (app.help_selected + 1).min(SELECTABLE - 1);
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.help_selected = app.help_selected.saturating_sub(1);
        }
        KeyCode::Char('g') => {
            app.help_selected = 0;
        }
        KeyCode::Char('G') => {
            app.help_selected = SELECTABLE - 1;
        }
        _ => {}
    }
    Ok(false)
}

// ── add form mode ─────────────────────────────────────────────────────────────

fn handle_add_form(
    app: &mut App,
    storage: &impl Storage,
    key: KeyCode,
    mods: KeyModifiers,
) -> Result<bool> {
    match key {
        KeyCode::Esc => {
            app.edit_form = None;
            app.mode = Mode::Normal;
            app.status_msg = Some("Add cancelled.".into());
        }
        KeyCode::Enter => {
            commit_add_form(app, storage)?;
        }
        KeyCode::Tab => {
            if let Some(ref mut form) = app.edit_form {
                if mods.contains(KeyModifiers::SHIFT) {
                    form.focused = form.focused.prev();
                } else {
                    form.focused = form.focused.next();
                }
            }
        }
        KeyCode::BackTab => {
            if let Some(ref mut form) = app.edit_form {
                form.focused = form.focused.prev();
            }
        }
        KeyCode::Left => {
            if let Some(ref mut form) = app.edit_form {
                match form.focused {
                    EditField::Priority => form.priority_prev(),
                    EditField::Recurrence => form.recurrence_prev(),
                    _ => {}
                }
            }
        }
        KeyCode::Right => {
            if let Some(ref mut form) = app.edit_form {
                match form.focused {
                    EditField::Priority => form.priority_next(),
                    EditField::Recurrence => form.recurrence_next(),
                    _ => {}
                }
            }
        }
        KeyCode::Backspace => {
            if let Some(ref mut form) = app.edit_form {
                if let Some(buf) = form.focused_buf_mut() {
                    buf.pop();
                }
            }
        }
        KeyCode::Char(c) => {
            if let Some(ref mut form) = app.edit_form {
                if let Some(buf) = form.focused_buf_mut() {
                    buf.push(c);
                }
            }
        }
        _ => {}
    }
    Ok(false)
}

fn commit_add_form(app: &mut App, storage: &impl Storage) -> Result<()> {
    let form = match app.edit_form.take() {
        Some(f) => f,
        None => return Ok(()),
    };

    if form.text.trim().is_empty() {
        app.edit_form = Some(form);
        app.status_msg = Some("Text cannot be empty.".into());
        return Ok(());
    }

    let tags_raw: Vec<String> = form
        .tags
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let existing_tags = collect_existing_tags(&app.tasks);
    let (tags, _) = normalize_tags(tags_raw, &existing_tags);

    let project = if form.project.trim().is_empty() {
        None
    } else {
        Some(form.project.trim().to_string())
    };

    let due = if form.due.trim().is_empty() {
        None
    } else {
        Some(form.due.trim().to_string())
    };

    let deps: Vec<usize> = form
        .deps
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .filter(|&id| id > 0)
        .collect();

    let args = crate::cli::AddArgs {
        text: form.text.trim().to_string(),
        priority: form.priority,
        tag: tags,
        project,
        due,
        recurrence: form.recurrence,
        depends_on: deps,
    };

    match crate::commands::add::execute_silent(storage, args) {
        Ok(_) => {
            let count = storage
                .load()
                .map(|t| t.iter().filter(|t| !t.is_deleted()).count())
                .unwrap_or(0);
            app.status_msg = Some(format!("Task #{} added.", count));
            app.mode = Mode::Normal;
            app.reload(storage)?;
        }
        Err(e) => {
            app.status_msg = Some(format!("Error: {}", e));
            app.mode = Mode::Normal;
        }
    }

    Ok(())
}

// ── edit form mode ────────────────────────────────────────────────────────────

fn handle_edit_form(
    app: &mut App,
    storage: &impl Storage,
    key: KeyCode,
    mods: KeyModifiers,
) -> Result<bool> {
    match key {
        KeyCode::Esc => {
            app.edit_form = None;
            app.mode = Mode::Normal;
            app.status_msg = Some("Edit cancelled.".into());
            return Ok(false);
        }

        KeyCode::Enter => {
            commit_edit_form(app, storage)?;
            return Ok(false);
        }

        // Tab / Shift+Tab — cycle fields
        KeyCode::Tab => {
            if let Some(ref mut form) = app.edit_form {
                if mods.contains(KeyModifiers::SHIFT) {
                    form.focused = form.focused.prev();
                } else {
                    form.focused = form.focused.next();
                }
            }
        }
        KeyCode::BackTab => {
            if let Some(ref mut form) = app.edit_form {
                form.focused = form.focused.prev();
            }
        }

        // ←/→ — cycle priority or recurrence depending on focused field
        KeyCode::Left => {
            if let Some(ref mut form) = app.edit_form {
                match form.focused {
                    EditField::Priority => form.priority_prev(),
                    EditField::Recurrence => form.recurrence_prev(),
                    _ => {}
                }
            }
        }
        KeyCode::Right => {
            if let Some(ref mut form) = app.edit_form {
                match form.focused {
                    EditField::Priority => form.priority_next(),
                    EditField::Recurrence => form.recurrence_next(),
                    _ => {}
                }
            }
        }

        // Backspace — delete from focused text field
        KeyCode::Backspace => {
            if let Some(ref mut form) = app.edit_form {
                if let Some(buf) = form.focused_buf_mut() {
                    buf.pop();
                }
            }
        }

        // Any printable char — append to focused text field
        KeyCode::Char(c) => {
            if let Some(ref mut form) = app.edit_form {
                if let Some(buf) = form.focused_buf_mut() {
                    buf.push(c);
                }
            }
        }

        _ => {}
    }
    Ok(false)
}

// ── stats mode ────────────────────────────────────────────────────────────────

// ── actions ───────────────────────────────────────────────────────────────────

fn toggle_done(app: &mut App, storage: &impl Storage) -> Result<()> {
    if app.filtered_indices.is_empty() {
        return Ok(());
    }

    let vis_id = match app.selected_visible_id() {
        Some(id) => id,
        None => return Ok(()),
    };
    let completed = app.selected_task().map(|t| t.completed).unwrap_or(false);

    let msg = if completed {
        crate::commands::undone::execute_silent(storage, vis_id)?
    } else {
        crate::commands::done::execute_silent(storage, vis_id)?
    };

    app.status_msg = Some(msg);
    app.reload(storage)?;
    Ok(())
}

fn commit_edit_form(app: &mut App, storage: &impl Storage) -> Result<()> {
    let vis_id = match app.selected_visible_id() {
        Some(id) => id,
        None => return Ok(()),
    };

    let form = match app.edit_form.take() {
        Some(f) => f,
        None => return Ok(()),
    };

    if form.text.trim().is_empty() {
        app.edit_form = Some(form);
        app.status_msg = Some("Text cannot be empty.".into());
        return Ok(());
    }

    // Parse due date
    let (due_str, clear_due) = if form.due.trim().is_empty() {
        (None, true)
    } else {
        (Some(form.due.trim().to_string()), false)
    };

    // Parse project
    let (project, clear_project) = if form.project.trim().is_empty() {
        (None, true)
    } else {
        (Some(form.project.trim().to_string()), false)
    };

    // Parse tags diff — normalize against existing tags first
    let tags_raw: Vec<String> = form
        .tags
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let existing_tags = collect_existing_tags(&app.tasks);
    let (tags_normalized, _) = normalize_tags(tags_raw, &existing_tags);
    let current_tags: Vec<String> = app
        .selected_task()
        .map(|t| t.tags.clone())
        .unwrap_or_default();
    let add_tag: Vec<String> = tags_normalized
        .iter()
        .filter(|t| !current_tags.contains(t))
        .cloned()
        .collect();
    let remove_tag: Vec<String> = current_tags
        .iter()
        .filter(|t| !tags_normalized.contains(t))
        .cloned()
        .collect();
    let clear_tags = tags_normalized.is_empty() && !current_tags.is_empty();

    // Parse deps — comma-separated IDs
    let deps_raw: Vec<usize> = form
        .deps
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect();
    let current_dep_uuids: Vec<uuid::Uuid> = app
        .selected_task()
        .map(|t| t.depends_on.clone())
        .unwrap_or_default();
    // Resolve current dep UUIDs → visible IDs for comparison
    let visible: Vec<&Task> = app.tasks.iter().filter(|t| !t.is_deleted()).collect();
    let current_dep_ids: Vec<usize> = current_dep_uuids
        .iter()
        .filter_map(|uuid| {
            let pos = visible.iter().position(|t| t.uuid == *uuid)?;
            Some(pos + 1)
        })
        .collect();
    let add_dep: Vec<usize> = deps_raw
        .iter()
        .filter(|id| !current_dep_ids.contains(id))
        .copied()
        .collect();
    let remove_dep: Vec<usize> = current_dep_ids
        .iter()
        .filter(|id| !deps_raw.contains(id))
        .copied()
        .collect();
    let clear_deps = deps_raw.is_empty() && !current_dep_ids.is_empty();

    let args = crate::cli::EditArgs {
        id: vis_id,
        text: Some(form.text.trim().to_string()),
        priority: Some(form.priority),
        due: due_str,
        clear_due,
        add_tag,
        remove_tag,
        clear_tags,
        project,
        clear_project,
        add_dep,
        remove_dep,
        clear_deps,
    };

    match crate::commands::edit::execute_silent(storage, args) {
        Ok(msg) => {
            // Handle recurrence separately — edit command doesn't cover it
            if let Some(real) = app.selected_real_index() {
                let mut tasks = storage.load()?;
                let task = &mut tasks[real];
                if task.recurrence != form.recurrence {
                    if form.recurrence.is_some() || task.recurrence.is_some() {
                        task.recurrence = form.recurrence;
                        task.touch();
                        storage.save(&tasks)?;
                    }
                }
            }
            app.status_msg = Some(msg);
            app.mode = Mode::Normal;
            app.reload(storage)?;
        }
        Err(e) => {
            app.status_msg = Some(format!("Error: {}", e));
            app.mode = Mode::Normal;
        }
    }

    Ok(())
}

fn delete_selected(app: &mut App, storage: &impl Storage) -> Result<()> {
    if app.filtered_indices.is_empty() {
        return Ok(());
    }

    let vis_id = match app.selected_visible_id() {
        Some(id) => id,
        None => return Ok(()),
    };

    let msg = crate::commands::remove::execute_silent(storage, vis_id)?;
    app.status_msg = Some(msg);
    app.reload(storage)?;
    Ok(())
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn truncate_str(s: &str, max: usize) -> String {
    let mut chars = s.chars();
    let mut result: String = chars.by_ref().take(max.saturating_sub(1)).collect();
    if chars.next().is_some() {
        result.push('…');
    }
    result
}
