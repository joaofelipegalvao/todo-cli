//! Ratatui rendering — lazygit-style layout.
//!
//! Left panel  `[1]`: single box with tab bar in the title — Tasks / Projects / Tags.
//!   `[`/`]` cycles tabs, `j`/`k` navigates within the active tab.
//!
//! Right panel `[0]`: single full-height box, content is contextual:
//!   • Tasks active   → rich task details (metadata + deps + history)
//!   • Projects active → list of tasks for selected project
//!   • Tags active    → list of tasks for selected tag
//!
//! Edit/Add form replaces the right panel content when active.

use chrono::Local;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, Clear, List, ListState, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table, Wrap,
    },
};

use crate::models::Task;

use super::app::{App, EditField, FocusedPanel, LeftPanel, Mode, PriorityFilter, TreeItem};

// ── palette ───────────────────────────────────────────────────────────────────

const COLOR_HIGH: Color = Color::Red;
const COLOR_MEDIUM: Color = Color::Yellow;
const COLOR_LOW: Color = Color::Green;
const COLOR_DONE: Color = Color::DarkGray;
const COLOR_BLOCKED: Color = Color::Rgb(150, 150, 150);
const COLOR_SELECTED_BG: Color = Color::Rgb(40, 40, 60);
const COLOR_ACCENT: Color = Color::Cyan;
const COLOR_SEARCH_BG: Color = Color::Rgb(30, 30, 50);
const COLOR_FOCUSED_BG: Color = Color::Rgb(30, 40, 60);
const COLOR_FOCUSED_BORDER: Color = Color::Cyan;

// ── entry point ───────────────────────────────────────────────────────────────

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(area);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(rows[0]);

    draw_left_panel(f, app, cols[0]);
    draw_right_panel(f, app, cols[1]);

    draw_footer(f, app, rows[1]);

    if app.mode == Mode::Search {
        draw_input_overlay(f, app, rows[1]);
    }
    if app.mode == Mode::Help {
        draw_help_popup(f, app, area);
    }
}

// ── left panel — single box, tab bar in title ─────────────────────────────────

fn draw_left_panel(f: &mut Frame, app: &mut App, area: Rect) {
    let is_focused = app.focused_panel == FocusedPanel::Left;
    let border_style = if is_focused {
        Style::default().fg(COLOR_ACCENT)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    // Title: [1]-Tasks - Projects - Tags  (active tab highlighted)
    let tabs = [LeftPanel::Tasks, LeftPanel::Projects, LeftPanel::Tags];
    let mut title_spans: Vec<Span> = vec![
        Span::styled("[1]", Style::default().fg(Color::DarkGray)),
        Span::styled("─", Style::default().fg(Color::DarkGray)),
    ];
    for (i, &tab) in tabs.iter().enumerate() {
        if i > 0 {
            title_spans.push(Span::styled(" - ", Style::default().fg(Color::DarkGray)));
        }
        if tab == app.left_panel {
            title_spans.push(Span::styled(
                tab.label(),
                Style::default()
                    .fg(COLOR_ACCENT)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            title_spans.push(Span::styled(
                tab.label(),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }
    title_spans.push(Span::raw(" "));

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Line::from(title_spans))
        .border_style(border_style);

    match app.left_panel {
        LeftPanel::Tasks => draw_tasks_tab(f, app, block, area),
        LeftPanel::Projects => draw_projects_tree(f, app, block, area),
        LeftPanel::Tags => draw_tags_list(f, app, block, area),
    }
}

// ── Tasks tab ─────────────────────────────────────────────────────────────────

fn draw_tasks_tab(f: &mut Frame, app: &mut App, block: Block, area: Rect) {
    let current = if app.filtered_indices.is_empty() {
        0
    } else {
        app.selected + 1
    };
    let total = app.filtered_indices.len();
    let counter = if app.priority_filter == PriorityFilter::All {
        format!(" ({}/{}) [{}] ", current, total, app.list_filter.label())
    } else {
        format!(
            " ({}/{}) [{} | P:{}] ",
            current,
            total,
            app.list_filter.label(),
            app.priority_filter.label()
        )
    };

    let lines: Vec<Line> = app
        .filtered_indices
        .iter()
        .map(|&i| task_line(&app.tasks[i], &app.tasks))
        .collect();

    let mut state = ListState::default();
    if !app.filtered_indices.is_empty() {
        state.select(Some(app.selected));
    }

    let block = block.title_bottom(Span::styled(counter, Style::default().fg(Color::DarkGray)));

    let list = List::new(lines).block(block).highlight_style(
        Style::default()
            .bg(COLOR_SELECTED_BG)
            .add_modifier(Modifier::BOLD),
    );

    f.render_stateful_widget(list, area, &mut state);
}

fn task_line<'a>(task: &'a Task, all_tasks: &'a [Task]) -> Line<'a> {
    let blocked = !task.completed && task.is_blocked(all_tasks);

    let (status_text, status_color) = if task.completed {
        ("D", Color::Green)
    } else if blocked {
        ("B", Color::Red)
    } else {
        ("P", Color::Blue)
    };

    let text_style = if task.completed {
        Style::default().fg(COLOR_DONE)
    } else if blocked {
        Style::default().fg(COLOR_BLOCKED)
    } else {
        Style::default().fg(Color::White)
    };

    Line::from(vec![
        Span::raw(" "),
        Span::styled(status_text, Style::default().fg(status_color)),
        Span::raw("  "),
        Span::styled(task.text.clone(), text_style),
    ])
}

// ── Projects tree tab ─────────────────────────────────────────────────────────

fn draw_projects_tree(f: &mut Frame, app: &App, block: Block, area: Rect) {
    if app.project_tree.is_empty() {
        f.render_widget(
            Paragraph::new("No projects")
                .style(Style::default().fg(Color::DarkGray))
                .block(block),
            area,
        );
        return;
    }

    let lines: Vec<Line> = app
        .project_tree
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let is_selected = i == app.tree_selected;
            match item {
                TreeItem::Project {
                    name,
                    task_count,
                    expanded,
                } => {
                    let arrow = if *expanded { "▼ " } else { "▶ " };
                    let label = name.as_deref().unwrap_or_default();
                    let count_str = format!(
                        "  {} task{}",
                        task_count,
                        if *task_count == 1 { "" } else { "s" }
                    );
                    let (name_color, arrow_color) = if is_selected {
                        (Color::White, COLOR_ACCENT)
                    } else {
                        (Color::Magenta, Color::DarkGray)
                    };
                    Line::from(vec![
                        Span::raw(" "),
                        Span::styled(arrow, Style::default().fg(arrow_color)),
                        Span::styled(
                            label.to_string(),
                            Style::default().fg(name_color).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(count_str, Style::default().fg(Color::DarkGray)),
                    ])
                }
                TreeItem::Task { task_idx } => {
                    let task = &app.tasks[*task_idx];
                    let blocked = !task.completed && task.is_blocked(&app.tasks);
                    let text_style = if task.completed {
                        Style::default().fg(COLOR_DONE)
                    } else if blocked {
                        Style::default().fg(COLOR_BLOCKED)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    Line::from(vec![
                        Span::raw("    "),
                        Span::styled(task.text.clone(), text_style),
                    ])
                }
            }
        })
        .collect();

    // Counter shows only named projects
    let project_count = app
        .project_tree
        .iter()
        .filter(|i| matches!(i, TreeItem::Project { name, .. } if name.is_some()))
        .count();
    let current_project = app.project_tree[..=app.tree_selected]
        .iter()
        .filter(|i| matches!(i, TreeItem::Project { name, .. } if name.is_some()))
        .count();
    let counter = format!(" {}/{} ", current_project.max(1), project_count);
    let block = block.title_bottom(Span::styled(counter, Style::default().fg(Color::DarkGray)));

    let mut state = ListState::default();
    state.select(Some(app.tree_selected));

    let list = List::new(lines).block(block).highlight_style(
        Style::default()
            .bg(COLOR_SELECTED_BG)
            .add_modifier(Modifier::BOLD),
    );

    f.render_stateful_widget(list, area, &mut state);
}

// ── Tags list tab ─────────────────────────────────────────────────────────────

fn draw_tags_list(f: &mut Frame, app: &App, block: Block, area: Rect) {
    let items = app.tags_list();

    if items.is_empty() {
        f.render_widget(
            Paragraph::new("No tags")
                .style(Style::default().fg(Color::DarkGray))
                .block(block),
            area,
        );
        return;
    }

    let lines: Vec<Line> = items
        .iter()
        .map(|name| {
            let count = app.tasks.iter().filter(|t| t.tags.contains(name)).count();
            Line::from(vec![
                Span::raw(" "),
                Span::styled(
                    name.clone(),
                    Style::default()
                        .fg(COLOR_ACCENT)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  {} task{}", count, if count == 1 { "" } else { "s" }),
                    Style::default().fg(Color::DarkGray),
                ),
            ])
        })
        .collect();

    let counter = format!(" {}/{} ", app.left_selected + 1, items.len());
    let block = block.title_bottom(Span::styled(counter, Style::default().fg(Color::DarkGray)));

    let mut state = ListState::default();
    state.select(Some(app.left_selected));

    let list = List::new(lines).block(block).highlight_style(
        Style::default()
            .bg(COLOR_SELECTED_BG)
            .add_modifier(Modifier::BOLD),
    );

    f.render_stateful_widget(list, area, &mut state);
}

// ── right panel — contextual ──────────────────────────────────────────────────

fn draw_right_panel(f: &mut Frame, app: &mut App, area: Rect) {
    match app.mode {
        Mode::EditForm => {
            draw_edit_form(f, app, area, false);
            return;
        }
        Mode::AddForm => {
            draw_edit_form(f, app, area, true);
            return;
        }
        _ => {}
    }

    let border_style = if app.focused_panel == FocusedPanel::Right {
        Style::default().fg(COLOR_ACCENT)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    match app.left_panel {
        LeftPanel::Tasks => draw_task_details(f, app, area, border_style),
        LeftPanel::Projects => draw_tree_details(f, app, area, border_style),
        LeftPanel::Tags => draw_context_panel(f, app, area, border_style, true),
    }
}

// ── [0] task details — rich single view ──────────────────────────────────────

fn draw_task_details(f: &mut Frame, app: &App, area: Rect, border_style: Style) {
    let title = match app.selected_task() {
        None => right_title("No task selected"),
        Some(task) => {
            let id = app.selected_visible_id().unwrap_or(0);
            let prefix_len = format!("[0]─ Task #{}: ", id).len();
            let max_text = (area.width as usize).saturating_sub(prefix_len + 2);
            Line::from(vec![
                Span::styled("[0]", Style::default().fg(Color::DarkGray)),
                Span::styled("─ Task #", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{}", id),
                    Style::default()
                        .fg(COLOR_ACCENT)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(": ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    truncate(&task.text, max_text),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
        }
    };

    let content = match app.selected_task() {
        None => vec![Line::from(Span::styled(
            "No tasks",
            Style::default().fg(Color::DarkGray),
        ))],
        Some(task) => build_task_details(task, &app.tasks, app.project_name_for(task)),
    };

    let inner_height = area.height.saturating_sub(2) as usize;
    let content_len = content.len();
    let max_scroll = content_len.saturating_sub(inner_height);
    let scroll = app.details_scroll.min(max_scroll);

    let para = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        )
        .wrap(Wrap { trim: true })
        .scroll((scroll as u16, 0));
    f.render_widget(para, area);

    if content_len > inner_height {
        let mut ss = ScrollbarState::new(max_scroll).position(scroll);
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area,
            &mut ss,
        );
    }
}

fn sep() -> Line<'static> {
    Line::from(Span::styled(
        "─────────────────────────────────────",
        Style::default().fg(Color::Rgb(50, 50, 50)),
    ))
}

fn build_task_details(
    task: &Task,
    all_tasks: &[Task],
    project_name: Option<&str>,
) -> Vec<Line<'static>> {
    let mut lines: Vec<Line> = Vec::new();

    // ── Metadata ─────────────────────────────────────────────────────────────
    lines.push(sep());

    let lbl = |s: &str| Span::styled(format!("{:<10}", s), Style::default().fg(Color::DarkGray));

    // Priority + Status
    let priority_span = match task.priority {
        crate::models::Priority::High => Span::styled(
            format!("{:<14}", "High"),
            Style::default().fg(COLOR_HIGH).add_modifier(Modifier::BOLD),
        ),
        crate::models::Priority::Medium => Span::styled(
            format!("{:<14}", "Medium"),
            Style::default().fg(COLOR_MEDIUM),
        ),
        crate::models::Priority::Low => {
            Span::styled(format!("{:<14}", "Low"), Style::default().fg(COLOR_LOW))
        }
    };
    let status_span = if task.completed {
        Span::styled("Done", Style::default().fg(COLOR_DONE))
    } else if task.is_blocked(all_tasks) {
        Span::styled(
            "Blocked",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled("Pending", Style::default().fg(Color::Green))
    };
    lines.push(Line::from(vec![
        lbl("Priority"),
        priority_span,
        lbl("Status"),
        status_span,
    ]));

    // Project + Due
    let has_project = project_name.is_some();
    let has_due = task.due_date.is_some();
    if has_project || has_due {
        let proj_val = Span::styled(
            format!("{:<14}", project_name.unwrap_or("")),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        );
        let due_str = task
            .due_date
            .map(|d| {
                let today = Local::now().naive_local().date();
                let days = (d - today).num_days();
                let suffix = match days {
                    d if d < 0 => format!(" ({}d late)", d.abs()),
                    0 => " (today)".into(),
                    d if d <= 7 => format!(" ({}d)", d),
                    _ => String::new(),
                };
                format!("{}{}", d.format("%Y-%m-%d"), suffix)
            })
            .unwrap_or_default();
        let due_color = task
            .due_date
            .map(|d| {
                let today = Local::now().naive_local().date();
                match (d - today).num_days() {
                    d if d < 0 => Color::Red,
                    d if d <= 7 => Color::Yellow,
                    _ => Color::White,
                }
            })
            .unwrap_or(Color::White);

        lines.push(Line::from(vec![
            lbl(if has_project { "Project" } else { "" }),
            if has_project {
                proj_val
            } else {
                Span::raw(format!("{:<14}", ""))
            },
            lbl(if has_due { "Due" } else { "" }),
            Span::styled(due_str, Style::default().fg(due_color)),
        ]));
    }

    // Tags + Created
    let tags_str = if task.tags.is_empty() {
        String::new()
    } else {
        task.tags.join(", ")
    };
    lines.push(Line::from(vec![
        lbl(if !tags_str.is_empty() { "Tags" } else { "" }),
        Span::styled(
            format!("{:<14}", truncate(&tags_str, 13)),
            Style::default().fg(COLOR_ACCENT),
        ),
        lbl("Created"),
        Span::styled(
            task.created_at.format("%Y-%m-%d").to_string(),
            Style::default().fg(Color::White),
        ),
    ]));

    // Recurrence
    if let Some(rec) = task.recurrence {
        lines.push(Line::from(vec![
            lbl("Recurs"),
            Span::styled(format!("{}", rec), Style::default().fg(Color::Cyan)),
        ]));
    }

    // Completed at
    if task.completed
        && let Some(done_at) = task.completed_at
    {
        lines.push(Line::from(vec![
            lbl("Completed"),
            Span::styled(
                done_at.format("%Y-%m-%d").to_string(),
                Style::default().fg(COLOR_DONE),
            ),
        ]));
    }

    // ── Dependencies ─────────────────────────────────────────────────────────
    let visible: Vec<&Task> = all_tasks.iter().filter(|t| !t.is_deleted()).collect();

    if !task.depends_on.is_empty() {
        lines.push(Line::from(""));
        lines.push(sep());
        lines.push(Line::from(Span::styled(
            "Dependencies",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        for dep_uuid in &task.depends_on {
            if let Some(pos) = visible.iter().position(|t| t.uuid == *dep_uuid) {
                let dep = visible[pos];
                let done = dep.completed;
                lines.push(Line::from(vec![
                    Span::styled(
                        if done {
                            "  └── [x] "
                        } else {
                            "  └── [ ] "
                        },
                        Style::default().fg(if done { Color::Green } else { Color::Red }),
                    ),
                    Span::styled(format!("#{}", pos + 1), Style::default().fg(COLOR_ACCENT)),
                    Span::raw("  "),
                    Span::styled(
                        truncate(&dep.text, 25),
                        Style::default().fg(if done { COLOR_DONE } else { Color::White }),
                    ),
                    if !done {
                        Span::styled("  (blocking you)", Style::default().fg(Color::Red))
                    } else {
                        Span::raw("")
                    },
                ]));
            }
        }
    }

    // ── Required by ───────────────────────────────────────────────────────────
    let downstream: Vec<(usize, &Task)> = visible
        .iter()
        .enumerate()
        .filter(|(_, t)| t.depends_on.contains(&task.uuid))
        .map(|(i, t)| (i, *t))
        .collect();

    if !downstream.is_empty() {
        lines.push(Line::from(""));
        lines.push(sep());
        lines.push(Line::from(Span::styled(
            "Required by",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        for (pos, dep) in &downstream {
            let done = dep.completed;
            lines.push(Line::from(vec![
                Span::styled(
                    if done {
                        "  └── [x] "
                    } else {
                        "  └── [ ] "
                    },
                    Style::default().fg(if done { Color::Green } else { Color::Yellow }),
                ),
                Span::styled(format!("#{}", pos + 1), Style::default().fg(COLOR_ACCENT)),
                Span::raw("  "),
                Span::styled(
                    truncate(&dep.text, 25),
                    Style::default().fg(if done { COLOR_DONE } else { Color::White }),
                ),
                if !done {
                    Span::styled("  (waiting on you)", Style::default().fg(Color::Yellow))
                } else {
                    Span::raw("")
                },
            ]));
        }
    }

    lines
}

// ── [0] tree details — always shows project summary ──────────────────────────

fn draw_tree_details(f: &mut Frame, app: &App, area: Rect, border_style: Style) {
    // Find the project name for whatever is currently selected
    // (walk backwards from tree_selected to find the parent project header)
    let project_name: Option<Option<&str>> = {
        let mut name = None;
        for item in app.project_tree[..=app
            .tree_selected
            .min(app.project_tree.len().saturating_sub(1))]
            .iter()
            .rev()
        {
            if let TreeItem::Project { name: n, .. } = item {
                name = Some(n.as_deref());
                break;
            }
        }
        name
    };

    let Some(proj_key) = project_name else {
        f.render_widget(
            Paragraph::new("").block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(right_title("Projects"))
                    .border_style(border_style),
            ),
            area,
        );
        return;
    };

    let label = proj_key.unwrap_or("");
    let proj_uuid = app
        .projects
        .iter()
        .find(|p| Some(p.name.as_str()) == proj_key && !p.is_deleted())
        .map(|p| p.uuid);
    let tasks: Vec<&Task> = app
        .tasks
        .iter()
        .filter(|t| proj_uuid.is_some() && t.project_id == proj_uuid)
        .collect();
    let pending = tasks.iter().filter(|t| !t.completed).count();
    let done = tasks.iter().filter(|t| t.completed).count();
    let blocked = tasks
        .iter()
        .filter(|t| {
            !t.completed
                && t.depends_on
                    .iter()
                    .any(|dep| app.tasks.iter().any(|t2| t2.uuid == *dep && !t2.completed))
        })
        .count();

    let mut lines: Vec<Line> = vec![
        Line::from(vec![
            Span::styled(
                format!("{} tasks", tasks.len()),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  ·  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} pending", pending),
                Style::default().fg(Color::Blue),
            ),
            if blocked > 0 {
                Span::styled(
                    format!("  ·  {} blocked", blocked),
                    Style::default().fg(Color::Red),
                )
            } else {
                Span::raw("")
            },
            Span::styled("  ·  ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{} done", done), Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        sep(),
        Line::from(""),
    ];

    for task in &tasks {
        let is_blocked = !task.completed
            && task
                .depends_on
                .iter()
                .any(|dep| app.tasks.iter().any(|t2| t2.uuid == *dep && !t2.completed));
        let text_style = if task.completed {
            Style::default().fg(COLOR_DONE)
        } else if is_blocked {
            Style::default().fg(COLOR_BLOCKED)
        } else {
            Style::default().fg(Color::White)
        };
        let pri = match task.priority {
            crate::models::Priority::High => Span::styled("H  ", Style::default().fg(COLOR_HIGH)),
            crate::models::Priority::Medium => {
                Span::styled("M  ", Style::default().fg(COLOR_MEDIUM))
            }
            crate::models::Priority::Low => Span::styled("L  ", Style::default().fg(COLOR_LOW)),
        };
        lines.push(Line::from(vec![
            pri,
            Span::styled(task.text.clone(), text_style),
        ]));
    }

    f.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(right_title(&format!("Project: {}", label)))
                    .border_style(border_style),
            )
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn draw_context_panel(f: &mut Frame, app: &App, area: Rect, border_style: Style, is_tags: bool) {
    let (label, tasks): (String, Vec<&Task>) = if is_tags {
        let tags = app.tags_list();
        match tags.get(app.left_selected) {
            Some(tag) => (format!("Tag: {}", tag), app.tasks_for_selected_tag()),
            None => {
                f.render_widget(
                    Paragraph::new("No tag selected").block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(right_title("Tag"))
                            .border_style(border_style),
                    ),
                    area,
                );
                return;
            }
        }
    } else {
        let projects = app.projects_list();
        match projects.get(app.left_selected) {
            Some(proj) => (
                format!("Project: {}", proj),
                app.tasks_for_selected_project(),
            ),
            None => {
                f.render_widget(
                    Paragraph::new("No project selected").block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(right_title("Project"))
                            .border_style(border_style),
                    ),
                    area,
                );
                return;
            }
        }
    };

    let pending = tasks.iter().filter(|t| !t.completed).count();
    let done = tasks.iter().filter(|t| t.completed).count();
    let blocked = tasks
        .iter()
        .filter(|t| {
            !t.completed
                && t.depends_on.iter().any(|dep_uuid| {
                    app.tasks
                        .iter()
                        .any(|t2| t2.uuid == *dep_uuid && !t2.completed)
                })
        })
        .count();

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(vec![
        Span::styled(
            format!("{} tasks", tasks.len()),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ·  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{} pending", pending),
            Style::default().fg(Color::Blue),
        ),
        if blocked > 0 {
            Span::styled(
                format!("  ·  {} blocked", blocked),
                Style::default().fg(Color::Red),
            )
        } else {
            Span::raw("")
        },
        Span::styled("  ·  ", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{} done", done), Style::default().fg(Color::Green)),
    ]));
    lines.push(Line::from(""));
    lines.push(sep());
    lines.push(Line::from(""));

    for task in &tasks {
        let is_blocked = !task.completed
            && task.depends_on.iter().any(|dep_uuid| {
                app.tasks
                    .iter()
                    .any(|t2| t2.uuid == *dep_uuid && !t2.completed)
            });
        let (s, s_color) = if task.completed {
            ("D", Color::Green)
        } else if is_blocked {
            ("B", Color::Red)
        } else {
            ("P", Color::Blue)
        };
        let extra = if is_tags {
            app.project_name_for(task).unwrap_or("").to_string()
        } else {
            task.tags.join(", ")
        };
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", s), Style::default().fg(s_color)),
            Span::styled(truncate(&task.text, 28), Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled(
                match task.priority {
                    crate::models::Priority::High => "H",
                    crate::models::Priority::Medium => "M",
                    crate::models::Priority::Low => "L",
                },
                Style::default().fg(match task.priority {
                    crate::models::Priority::High => COLOR_HIGH,
                    crate::models::Priority::Medium => COLOR_MEDIUM,
                    crate::models::Priority::Low => COLOR_LOW,
                }),
            ),
            Span::raw("  "),
            Span::styled(
                truncate(&extra, 12),
                Style::default()
                    .fg(if is_tags {
                        Color::Magenta
                    } else {
                        COLOR_ACCENT
                    })
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    let inner_height = area.height.saturating_sub(2) as usize;
    let content_len = lines.len();
    let max_scroll = content_len.saturating_sub(inner_height);
    let scroll = app.details_scroll.min(max_scroll);

    f.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(right_title(&label))
                    .border_style(border_style),
            )
            .wrap(Wrap { trim: false })
            .scroll((scroll as u16, 0)),
        area,
    );
}

fn right_title(label: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled("[0]", Style::default().fg(Color::DarkGray)),
        Span::styled("─ ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            label.to_string(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}

// ── edit / add form ───────────────────────────────────────────────────────────

fn draw_edit_form(f: &mut Frame, app: &App, area: Rect, is_add: bool) {
    let form = match &app.edit_form {
        Some(f) => f,
        None => return,
    };

    let title = if is_add {
        right_title("New Task")
    } else {
        right_title(&format!(
            "Edit — #{}",
            app.selected_visible_id().unwrap_or(0)
        ))
    };

    let all_fields = [
        EditField::Text,
        EditField::Priority,
        EditField::Due,
        EditField::Recurrence,
        EditField::Project,
        EditField::Tags,
        EditField::Deps,
    ];

    let outer = Block::default().borders(Borders::ALL).title(title);
    let inner_area = outer.inner(area);
    f.render_widget(outer, area);

    let constraints: Vec<Constraint> = all_fields.iter().map(|_| Constraint::Length(3)).collect();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner_area);

    for (i, &field) in all_fields.iter().enumerate() {
        let chunk = chunks[i];
        let focused = form.focused == field;

        let label_style = if focused {
            Style::default()
                .fg(COLOR_FOCUSED_BORDER)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let input_bg = if focused {
            COLOR_FOCUSED_BG
        } else {
            Color::Reset
        };
        let label_line = Line::from(Span::styled(format!(" {} ", field.label()), label_style));

        let input_line = if field == EditField::Priority {
            let (h_s, m_s, l_s) = match form.priority {
                crate::models::Priority::High => (
                    Style::default()
                        .fg(COLOR_HIGH)
                        .add_modifier(Modifier::BOLD | Modifier::REVERSED),
                    Style::default().fg(Color::DarkGray),
                    Style::default().fg(Color::DarkGray),
                ),
                crate::models::Priority::Medium => (
                    Style::default().fg(Color::DarkGray),
                    Style::default()
                        .fg(COLOR_MEDIUM)
                        .add_modifier(Modifier::BOLD | Modifier::REVERSED),
                    Style::default().fg(Color::DarkGray),
                ),
                crate::models::Priority::Low => (
                    Style::default().fg(Color::DarkGray),
                    Style::default().fg(Color::DarkGray),
                    Style::default()
                        .fg(COLOR_LOW)
                        .add_modifier(Modifier::BOLD | Modifier::REVERSED),
                ),
            };
            let arrow = if focused {
                Style::default().fg(COLOR_ACCENT)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            Line::from(vec![
                Span::styled(" ◀ ", arrow),
                Span::styled(" High ", h_s),
                Span::raw("  "),
                Span::styled(" Medium ", m_s),
                Span::raw("  "),
                Span::styled(" Low ", l_s),
                Span::styled(" ▶ ", arrow),
            ])
        } else if field == EditField::Recurrence {
            let options = ["None", "Daily", "Weekly", "Monthly"];
            let current = form.recurrence_label();
            let arrow = if focused {
                Style::default().fg(COLOR_ACCENT)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let mut spans = vec![Span::styled(" ◀ ", arrow)];
            for opt in &options {
                let s = if *opt == current {
                    Style::default()
                        .fg(COLOR_ACCENT)
                        .add_modifier(Modifier::BOLD | Modifier::REVERSED)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                spans.push(Span::styled(format!(" {} ", opt), s));
                spans.push(Span::raw("  "));
            }
            spans.push(Span::styled(" ▶ ", arrow));
            Line::from(spans)
        } else {
            let buf = match field {
                EditField::Text => &form.text,
                EditField::Due => &form.due,
                EditField::Project => &form.project,
                EditField::Tags => &form.tags,
                EditField::Deps => &form.deps,
                EditField::Priority | EditField::Recurrence => unreachable!(),
            };
            let cursor = if focused { "█" } else { "" };
            Line::from(Span::styled(
                format!(" {}{} ", buf, cursor),
                Style::default().fg(Color::White).bg(input_bg),
            ))
        };

        let border_style = if focused {
            Style::default().fg(COLOR_FOCUSED_BORDER)
        } else {
            Style::default().fg(Color::Rgb(50, 50, 50))
        };
        f.render_widget(
            Paragraph::new(vec![label_line, input_line]).block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(border_style),
            ),
            chunk,
        );
    }
}

// ── search overlay ────────────────────────────────────────────────────────────

fn draw_input_overlay(f: &mut Frame, app: &App, area: Rect) {
    f.render_widget(Clear, area);
    let content = Line::from(vec![
        Span::styled(
            "Search: ",
            Style::default()
                .fg(COLOR_ACCENT)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(app.input.clone(), Style::default().fg(Color::White)),
        Span::styled("█", Style::default().fg(COLOR_ACCENT)),
    ]);
    f.render_widget(
        Paragraph::new(content).style(Style::default().bg(COLOR_SEARCH_BG)),
        area,
    );
}

// ── footer ────────────────────────────────────────────────────────────────────

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    if app.mode == Mode::Search {
        return;
    }

    let text = if let Some(ref msg) = app.status_msg {
        let color = if app.mode == Mode::ConfirmDelete {
            Color::Yellow
        } else {
            Color::Green
        };
        Line::from(Span::styled(msg.clone(), Style::default().fg(color)))
    } else {
        match app.mode {
            Mode::ConfirmDelete => Line::from(vec![
                Span::styled(
                    "[y]",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" confirm  "),
                Span::styled("[n]", Style::default().fg(COLOR_ACCENT)),
                Span::raw(" cancel"),
            ]),
            Mode::EditForm | Mode::AddForm => Line::from(vec![
                Span::styled("[Tab]", Style::default().fg(COLOR_ACCENT)),
                Span::raw(" next  "),
                Span::styled("[S-Tab]", Style::default().fg(COLOR_ACCENT)),
                Span::raw(" prev  "),
                Span::styled("[←/→]", Style::default().fg(COLOR_ACCENT)),
                Span::raw(" cycle  "),
                Span::styled("[Enter]", Style::default().fg(Color::Green)),
                Span::raw(" save  "),
                Span::styled("[Esc]", Style::default().fg(Color::Red)),
                Span::raw(" cancel"),
            ]),
            _ => Line::from(vec![
                Span::styled("j/k", Style::default().fg(COLOR_ACCENT)),
                Span::raw(" nav  "),
                Span::styled("Tab", Style::default().fg(COLOR_ACCENT)),
                Span::raw(" focus  "),
                Span::styled("[ ]", Style::default().fg(COLOR_ACCENT)),
                Span::raw(" tab  "),
                Span::styled("a", Style::default().fg(Color::Green)),
                Span::raw(" add  "),
                Span::styled("e", Style::default().fg(Color::Yellow)),
                Span::raw(" edit  "),
                Span::styled("d", Style::default().fg(COLOR_ACCENT)),
                Span::raw(" done  "),
                Span::styled("x", Style::default().fg(Color::Red)),
                Span::raw(" del  "),
                Span::styled("/", Style::default().fg(COLOR_ACCENT)),
                Span::raw(" search  "),
                Span::styled("q", Style::default().fg(COLOR_ACCENT)),
                Span::raw(" quit  "),
                Span::styled("?", Style::default().fg(COLOR_ACCENT)),
                Span::raw(" help"),
            ]),
        }
    };

    f.render_widget(Paragraph::new(text), area);
}

// ── help popup ────────────────────────────────────────────────────────────────

struct HelpEntry {
    key: &'static str,
    action: &'static str,
    description: Option<&'static str>,
}

fn help_entries() -> Vec<HelpEntry> {
    vec![
        HelpEntry {
            key: "──── Navigation",
            action: "",
            description: None,
        },
        HelpEntry {
            key: "j / k",
            action: "Navigate list",
            description: None,
        },
        HelpEntry {
            key: "↑ / ↓",
            action: "Scroll right panel",
            description: Some("Scrolls the right panel without moving the selected item."),
        },
        HelpEntry {
            key: "g / G",
            action: "First / last item",
            description: None,
        },
        HelpEntry {
            key: "Tab",
            action: "Toggle panel focus",
            description: Some(
                "Switches focus between [1] left and [0] right panel. Focused panel has cyan border.",
            ),
        },
        HelpEntry {
            key: "[ / ]",
            action: "Cycle left panel tabs",
            description: Some("Switches the left panel [1] between Tasks, Projects, and Tags."),
        },
        HelpEntry {
            key: "──── Actions",
            action: "",
            description: None,
        },
        HelpEntry {
            key: "a",
            action: "Add new task",
            description: Some("Opens a blank form in [0]. Tab navigates fields, Enter saves."),
        },
        HelpEntry {
            key: "e",
            action: "Edit selected task",
            description: Some("Opens the edit form pre-filled. Tab navigates fields, Enter saves."),
        },
        HelpEntry {
            key: "d",
            action: "Toggle done / undone",
            description: Some("Marks task completed. Recurring tasks spawn a new occurrence."),
        },
        HelpEntry {
            key: "x",
            action: "Delete task",
            description: Some("Prompts [y/n]. Tasks are soft-deleted."),
        },
        HelpEntry {
            key: "X",
            action: "Clear all visible tasks",
            description: Some("Prompts [y/n]. Deletes all tasks matching active filters."),
        },
        HelpEntry {
            key: "──── Filters",
            action: "",
            description: None,
        },
        HelpEntry {
            key: "f",
            action: "Cycle status filter",
            description: Some("Pending → Done → All."),
        },
        HelpEntry {
            key: "p",
            action: "Cycle priority filter",
            description: Some("All → High → Medium → Low."),
        },
        HelpEntry {
            key: "/",
            action: "Search",
            description: Some("Live filter. @project and #tag tokens supported."),
        },
        HelpEntry {
            key: "──── General",
            action: "",
            description: None,
        },
        HelpEntry {
            key: "?",
            action: "This help",
            description: None,
        },
        HelpEntry {
            key: "Esc",
            action: "Close / cancel",
            description: None,
        },
        HelpEntry {
            key: "q",
            action: "Quit",
            description: None,
        },
    ]
}

fn draw_help_popup(f: &mut Frame, app: &App, area: Rect) {
    let entries = help_entries();
    let selectable: Vec<usize> = entries
        .iter()
        .enumerate()
        .filter(|(_, e)| !e.action.is_empty())
        .map(|(i, _)| i)
        .collect();
    let selectable_count = selectable.len();
    let sel_pos = app.help_selected.min(selectable_count.saturating_sub(1));
    let sel_real = selectable[sel_pos];
    let desc_text = entries[sel_real].description.unwrap_or("");
    let desc_h = 4u16;

    let popup_w = (area.width as f32 * 0.70) as u16;
    let popup_h = (area.height as f32 * 0.80) as u16;
    let popup_x = (area.width.saturating_sub(popup_w)) / 2;
    let popup_y = (area.height.saturating_sub(popup_h)) / 2;
    let popup_area = Rect::new(popup_x, popup_y, popup_w, popup_h);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(desc_h)])
        .split(popup_area);

    f.render_widget(Clear, popup_area);

    let inner_list_h = chunks[0].height.saturating_sub(2) as usize;
    let scroll = if sel_real >= inner_list_h {
        sel_real - inner_list_h + 1
    } else {
        0
    };

    let rows: Vec<Row> = entries
        .iter()
        .enumerate()
        .skip(scroll)
        .take(inner_list_h)
        .map(|(i, e)| {
            let is_section = e.action.is_empty();
            let is_selected = i == sel_real;
            if is_section {
                Row::new(vec![
                    Cell::from(e.key).style(
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Cell::from(""),
                ])
            } else {
                Row::new(vec![
                    Cell::from(format!("  {}", e.key)).style(
                        Style::default()
                            .fg(COLOR_ACCENT)
                            .add_modifier(if is_selected {
                                Modifier::BOLD
                            } else {
                                Modifier::empty()
                            }),
                    ),
                    Cell::from(e.action).style(Style::default().fg(Color::White).add_modifier(
                        if is_selected {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        },
                    )),
                ])
                .style(if is_selected {
                    Style::default().bg(COLOR_SELECTED_BG)
                } else {
                    Style::default()
                })
            }
        })
        .collect();

    let counter = format!(" {} of {} ", sel_pos + 1, selectable_count);
    let table = Table::new(rows, [Constraint::Length(18), Constraint::Min(10)]).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Keybindings ")
            .title_bottom(
                Line::from(Span::styled(counter, Style::default().fg(Color::DarkGray)))
                    .right_aligned(),
            )
            .border_style(Style::default().fg(COLOR_ACCENT)),
    );
    f.render_widget(table, chunks[0]);

    let desc_para = Paragraph::new(desc_text)
        .block(
            Block::default()
                .borders(if desc_text.is_empty() {
                    Borders::NONE
                } else {
                    Borders::ALL
                })
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White));
    f.render_widget(desc_para, chunks[1]);
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn truncate(s: &str, max: usize) -> String {
    let mut chars = s.chars();
    let mut result: String = chars.by_ref().take(max.saturating_sub(1)).collect();
    if chars.next().is_some() {
        result.push('…');
    }
    result
}
