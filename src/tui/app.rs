//! Application state for the TUI.

use crate::models::{Priority, Project, Recurrence, StatusFilter, Task};
use crate::storage::Storage;
use anyhow::Result;

// ── Mode ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    ConfirmDelete,
    ConfirmClearAll,
    Search,
    EditForm,
    AddForm,
    Help,
}

// ── FocusedPanel ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum FocusedPanel {
    Left,
    Right,
}

impl FocusedPanel {
    pub fn toggle(self) -> Self {
        match self {
            FocusedPanel::Left => FocusedPanel::Right,
            FocusedPanel::Right => FocusedPanel::Left,
        }
    }
}

// ── LeftPanel ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum LeftPanel {
    Tasks,
    Projects,
    Tags,
}

impl LeftPanel {
    pub fn next(self) -> Self {
        match self {
            LeftPanel::Tasks => LeftPanel::Projects,
            LeftPanel::Projects => LeftPanel::Tags,
            LeftPanel::Tags => LeftPanel::Tasks,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            LeftPanel::Tasks => LeftPanel::Tags,
            LeftPanel::Projects => LeftPanel::Tasks,
            LeftPanel::Tags => LeftPanel::Projects,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            LeftPanel::Tasks => "Tasks",
            LeftPanel::Projects => "Projects",
            LeftPanel::Tags => "Tags",
        }
    }
}

// ── RightPanel ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum RightPanel {
    Details,
    Stats,
    Deps,
}

impl RightPanel {
    pub fn next(self) -> Self {
        match self {
            RightPanel::Details => RightPanel::Stats,
            RightPanel::Stats => RightPanel::Deps,
            RightPanel::Deps => RightPanel::Details,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            RightPanel::Details => RightPanel::Deps,
            RightPanel::Stats => RightPanel::Details,
            RightPanel::Deps => RightPanel::Stats,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            RightPanel::Details => "Details",
            RightPanel::Stats => "Stats",
            RightPanel::Deps => "Deps",
        }
    }
}

// ── EditField ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum EditField {
    Text,
    Priority,
    Due,
    Recurrence,
    Project,
    Tags,
    Deps,
}

impl EditField {
    pub fn next(self) -> Self {
        match self {
            EditField::Text => EditField::Priority,
            EditField::Priority => EditField::Due,
            EditField::Due => EditField::Recurrence,
            EditField::Recurrence => EditField::Project,
            EditField::Project => EditField::Tags,
            EditField::Tags => EditField::Deps,
            EditField::Deps => EditField::Text,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            EditField::Text => EditField::Deps,
            EditField::Priority => EditField::Text,
            EditField::Due => EditField::Priority,
            EditField::Recurrence => EditField::Due,
            EditField::Project => EditField::Recurrence,
            EditField::Tags => EditField::Project,
            EditField::Deps => EditField::Tags,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            EditField::Text => "Text",
            EditField::Priority => "Priority",
            EditField::Due => "Due",
            EditField::Recurrence => "Recurrence",
            EditField::Project => "Project",
            EditField::Tags => "Tags",
            EditField::Deps => "Deps (IDs)",
        }
    }
}

// ── EditFormState ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct EditFormState {
    pub focused: EditField,
    pub text: String,
    pub priority: Priority,
    pub due: String,
    pub recurrence: Option<Recurrence>,
    pub project: String,
    pub tags: String,
    pub deps: String,
}

impl EditFormState {
    pub fn blank() -> Self {
        Self {
            focused: EditField::Text,
            text: String::new(),
            priority: Priority::Medium,
            due: String::new(),
            recurrence: None,
            project: String::new(),
            tags: String::new(),
            deps: String::new(),
        }
    }

    pub fn from_task(task: &Task, all_tasks: &[Task], projects: &[Project]) -> Self {
        let visible: Vec<&Task> = all_tasks.iter().filter(|t| !t.is_deleted()).collect();
        let deps = task
            .depends_on
            .iter()
            .filter_map(|uuid| {
                let pos = visible.iter().position(|t| t.uuid == *uuid)?;
                Some((pos + 1).to_string())
            })
            .collect::<Vec<_>>()
            .join(", ");

        // Resolve project_id → project name for the form field
        let project_name = task
            .project_id
            .and_then(|pid| projects.iter().find(|p| p.uuid == pid && !p.is_deleted()))
            .map(|p| p.name.clone())
            .unwrap_or_default();

        Self {
            focused: EditField::Text,
            text: task.text.clone(),
            priority: task.priority,
            due: task
                .due_date
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_default(),
            recurrence: task.recurrence,
            project: project_name,
            tags: task.tags.join(", "),
            deps,
        }
    }

    pub fn focused_buf_mut(&mut self) -> Option<&mut String> {
        match self.focused {
            EditField::Text => Some(&mut self.text),
            EditField::Due => Some(&mut self.due),
            EditField::Project => Some(&mut self.project),
            EditField::Tags => Some(&mut self.tags),
            EditField::Deps => Some(&mut self.deps),
            EditField::Priority | EditField::Recurrence => None,
        }
    }

    pub fn priority_prev(&mut self) {
        self.priority = match self.priority {
            Priority::High => Priority::Low,
            Priority::Medium => Priority::High,
            Priority::Low => Priority::Medium,
        };
    }
    pub fn priority_next(&mut self) {
        self.priority = match self.priority {
            Priority::High => Priority::Medium,
            Priority::Medium => Priority::Low,
            Priority::Low => Priority::High,
        };
    }
    pub fn recurrence_next(&mut self) {
        self.recurrence = match self.recurrence {
            None => Some(Recurrence::Daily),
            Some(Recurrence::Daily) => Some(Recurrence::Weekly),
            Some(Recurrence::Weekly) => Some(Recurrence::Monthly),
            Some(Recurrence::Monthly) => None,
        };
    }
    pub fn recurrence_prev(&mut self) {
        self.recurrence = match self.recurrence {
            None => Some(Recurrence::Monthly),
            Some(Recurrence::Daily) => None,
            Some(Recurrence::Weekly) => Some(Recurrence::Daily),
            Some(Recurrence::Monthly) => Some(Recurrence::Weekly),
        };
    }
    pub fn recurrence_label(&self) -> &'static str {
        match self.recurrence {
            None => "None",
            Some(Recurrence::Daily) => "Daily",
            Some(Recurrence::Weekly) => "Weekly",
            Some(Recurrence::Monthly) => "Monthly",
        }
    }
}

// ── ListFilter ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ListFilter {
    Pending,
    Done,
    All,
}

impl ListFilter {
    pub fn next(self) -> Self {
        match self {
            ListFilter::Pending => ListFilter::Done,
            ListFilter::Done => ListFilter::All,
            ListFilter::All => ListFilter::Pending,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            ListFilter::Pending => "Pending",
            ListFilter::Done => "Done",
            ListFilter::All => "All",
        }
    }
    pub fn as_status_filter(self) -> StatusFilter {
        match self {
            ListFilter::Pending => StatusFilter::Pending,
            ListFilter::Done => StatusFilter::Done,
            ListFilter::All => StatusFilter::All,
        }
    }
}

// ── PriorityFilter ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PriorityFilter {
    All,
    High,
    Medium,
    Low,
}

impl PriorityFilter {
    pub fn next(self) -> Self {
        match self {
            PriorityFilter::All => PriorityFilter::High,
            PriorityFilter::High => PriorityFilter::Medium,
            PriorityFilter::Medium => PriorityFilter::Low,
            PriorityFilter::Low => PriorityFilter::All,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            PriorityFilter::All => "All",
            PriorityFilter::High => "High",
            PriorityFilter::Medium => "Med",
            PriorityFilter::Low => "Low",
        }
    }
}

// ── TreeItem ──────────────────────────────────────────────────────────────────

/// A flat navigable item in the project tree view.
#[derive(Debug, Clone)]
pub enum TreeItem {
    /// A project header (expandable).
    Project {
        name: Option<String>,
        task_count: usize,
        expanded: bool,
    },
    /// A task row nested under its project.
    Task { task_idx: usize },
}

impl TreeItem {
    pub fn is_project(&self) -> bool {
        matches!(self, TreeItem::Project { .. })
    }
    pub fn is_task(&self) -> bool {
        matches!(self, TreeItem::Task { .. })
    }
    pub fn task_idx(&self) -> Option<usize> {
        match self {
            TreeItem::Task { task_idx } => Some(*task_idx),
            _ => None,
        }
    }
    pub fn expanded(&self) -> bool {
        match self {
            TreeItem::Project { expanded, .. } => *expanded,
            _ => false,
        }
    }
}

// ── App ───────────────────────────────────────────────────────────────────────

pub struct App {
    pub tasks: Vec<Task>,
    pub projects: Vec<Project>,
    pub filtered_indices: Vec<usize>,
    pub selected: usize,
    pub mode: Mode,
    pub status_msg: Option<String>,
    pub details_scroll: usize,
    pub list_filter: ListFilter,
    pub priority_filter: PriorityFilter,
    pub input: String,
    pub edit_form: Option<EditFormState>,
    pub help_selected: usize,
    pub right_panel: RightPanel,
    pub left_panel: LeftPanel,
    pub left_selected: usize,
    pub focused_panel: FocusedPanel,
    /// Flat navigable list for the project tree.
    pub project_tree: Vec<TreeItem>,
    /// Selected row index within project_tree.
    pub tree_selected: usize,
}

impl App {
    pub fn new(storage: &impl Storage) -> Result<Self> {
        let tasks = Self::load_visible(storage)?;
        let projects = storage.load_projects()?;
        let filtered_indices = tasks
            .iter()
            .enumerate()
            .filter(|(_, t)| !t.completed)
            .map(|(i, _)| i)
            .collect();
        let mut app = Self {
            tasks,
            projects,
            filtered_indices,
            selected: 0,
            mode: Mode::Normal,
            status_msg: None,
            details_scroll: 0,
            list_filter: ListFilter::Pending,
            priority_filter: PriorityFilter::All,
            input: String::new(),
            edit_form: None,
            help_selected: 0,
            right_panel: RightPanel::Details,
            left_panel: LeftPanel::Tasks,
            left_selected: 0,
            focused_panel: FocusedPanel::Left,
            project_tree: vec![],
            tree_selected: 0,
        };
        app.build_project_tree();
        Ok(app)
    }

    pub fn reload(&mut self, storage: &impl Storage) -> Result<()> {
        self.tasks = Self::load_visible(storage)?;
        self.projects = storage.load_projects()?;
        self.refilter();
        if self.selected >= self.filtered_indices.len() {
            self.selected = self.filtered_indices.len().saturating_sub(1);
        }
        self.build_project_tree();
        Ok(())
    }

    /// Resolve project_id → project name for display purposes.
    pub fn project_name_for<'a>(&'a self, task: &Task) -> Option<&'a str> {
        let pid = task.project_id?;
        self.projects
            .iter()
            .find(|p| p.uuid == pid && !p.is_deleted())
            .map(|p| p.name.as_str())
    }

    pub fn refilter(&mut self) {
        let raw = self.input.to_lowercase();
        let status = self.list_filter.as_status_filter();

        let mut project_filter: Option<String> = None;
        let mut tag_filters: Vec<String> = Vec::new();
        let mut text_tokens: Vec<String> = Vec::new();

        if self.mode == Mode::Search && !raw.is_empty() {
            for token in raw.split_whitespace() {
                if let Some(proj) = token.strip_prefix('@') {
                    project_filter = Some(proj.to_string());
                } else if let Some(tag) = token.strip_prefix('#') {
                    tag_filters.push(tag.to_string());
                } else {
                    text_tokens.push(token.to_string());
                }
            }
        }

        self.filtered_indices = self
            .tasks
            .iter()
            .enumerate()
            .filter(|(_, t)| t.matches_status(status))
            .filter(|(_, t)| match self.priority_filter {
                PriorityFilter::All => true,
                PriorityFilter::High => t.priority == Priority::High,
                PriorityFilter::Medium => t.priority == Priority::Medium,
                PriorityFilter::Low => t.priority == Priority::Low,
            })
            .filter(|(_, t)| {
                if self.mode != Mode::Search || raw.is_empty() {
                    return true;
                }
                if let Some(ref pf) = project_filter {
                    // Resolve project name via project_id for search filtering
                    let proj_name = t
                        .project_id
                        .and_then(|pid| {
                            self.projects
                                .iter()
                                .find(|p| p.uuid == pid && !p.is_deleted())
                        })
                        .map(|p| p.name.to_lowercase());
                    match proj_name {
                        Some(ref name) if name.contains(pf.as_str()) => {}
                        _ => return false,
                    }
                }
                for tf in &tag_filters {
                    if !t
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(tf.as_str()))
                    {
                        return false;
                    }
                }
                for token in &text_tokens {
                    if !t.text.to_lowercase().contains(token.as_str()) {
                        return false;
                    }
                }
                true
            })
            .map(|(i, _)| i)
            .collect();
    }

    // ── project tree ──────────────────────────────────────────────────────────

    /// Build (or rebuild) the flat navigable project tree.
    /// Preserves expanded/collapsed state across rebuilds.
    pub fn build_project_tree(&mut self) {
        // Preserve existing expanded states
        let prev_expanded: std::collections::HashMap<String, bool> = self
            .project_tree
            .iter()
            .filter_map(|item| match item {
                TreeItem::Project { name, expanded, .. } => {
                    Some((name.clone().unwrap_or_default(), *expanded))
                }
                _ => None,
            })
            .collect();

        // Collect project names from loaded projects (non-deleted), sorted
        let mut project_names: Vec<Option<String>> = self
            .projects
            .iter()
            .filter(|p| !p.is_deleted())
            .map(|p| Some(p.name.clone()))
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        project_names.sort();

        let mut tree: Vec<TreeItem> = Vec::new();
        for proj_name in &project_names {
            let key = proj_name.clone().unwrap_or_default();
            let expanded = prev_expanded.get(&key).copied().unwrap_or(true);

            // Count tasks that belong to this project
            let task_count = self
                .tasks
                .iter()
                .filter(|t| {
                    let name = t
                        .project_id
                        .and_then(|pid| {
                            self.projects
                                .iter()
                                .find(|p| p.uuid == pid && !p.is_deleted())
                        })
                        .map(|p| p.name.as_str());
                    name == proj_name.as_deref()
                })
                .count();

            tree.push(TreeItem::Project {
                name: proj_name.clone(),
                task_count,
                expanded,
            });

            if expanded {
                for (idx, task) in self.tasks.iter().enumerate() {
                    let name = task
                        .project_id
                        .and_then(|pid| {
                            self.projects
                                .iter()
                                .find(|p| p.uuid == pid && !p.is_deleted())
                        })
                        .map(|p| p.name.as_str());
                    if name == proj_name.as_deref() {
                        tree.push(TreeItem::Task { task_idx: idx });
                    }
                }
            }
        }

        self.project_tree = tree;
        self.tree_selected = self
            .tree_selected
            .min(self.project_tree.len().saturating_sub(1));
    }

    /// Toggle expand/collapse on the currently selected project header.
    pub fn tree_toggle_expand(&mut self) {
        if let Some(TreeItem::Project { expanded, .. }) =
            self.project_tree.get_mut(self.tree_selected)
        {
            *expanded = !*expanded;
            // Rebuild in place to add/remove child rows
            self.build_project_tree();
        }
    }

    pub fn tree_move_down(&mut self) {
        if !self.project_tree.is_empty() {
            self.tree_selected = (self.tree_selected + 1).min(self.project_tree.len() - 1);
        }
        self.details_scroll = 0;
    }

    pub fn tree_move_up(&mut self) {
        self.tree_selected = self.tree_selected.saturating_sub(1);
        self.details_scroll = 0;
    }

    /// The task currently selected in the tree (if a Task row is selected).
    pub fn tree_selected_task(&self) -> Option<&Task> {
        match self.project_tree.get(self.tree_selected)? {
            TreeItem::Task { task_idx } => self.tasks.get(*task_idx),
            _ => None,
        }
    }

    /// Visible 1-based ID of the tree-selected task.
    pub fn tree_selected_task_visible_id(&self) -> Option<usize> {
        let task_idx = match self.project_tree.get(self.tree_selected)? {
            TreeItem::Task { task_idx } => *task_idx,
            _ => return None,
        };
        let visible: Vec<&Task> = self.tasks.iter().filter(|t| !t.is_deleted()).collect();
        visible
            .iter()
            .position(|t| std::ptr::eq(*t, &self.tasks[task_idx]))
            .map(|p| p + 1)
    }

    // ── form helpers ──────────────────────────────────────────────────────────

    pub fn open_edit_form(&mut self) {
        if let Some(real) = self.selected_real_index() {
            let task = self.tasks[real].clone();
            let all_tasks = self.tasks.clone();
            let projects = self.projects.clone();
            self.edit_form = Some(EditFormState::from_task(&task, &all_tasks, &projects));
            self.mode = Mode::EditForm;
            self.status_msg = None;
        }
    }

    pub fn open_add_form(&mut self) {
        self.edit_form = Some(EditFormState::blank());
        self.mode = Mode::AddForm;
        self.status_msg = None;
    }

    // ── selection helpers ─────────────────────────────────────────────────────

    pub fn selected_task(&self) -> Option<&Task> {
        let real = *self.filtered_indices.get(self.selected)?;
        self.tasks.get(real)
    }

    pub fn selected_real_index(&self) -> Option<usize> {
        self.filtered_indices.get(self.selected).copied()
    }

    pub fn selected_visible_id(&self) -> Option<usize> {
        let real = self.selected_real_index()?;
        Some(real + 1)
    }

    // ── navigation ────────────────────────────────────────────────────────────

    pub fn move_down(&mut self) {
        if !self.filtered_indices.is_empty() {
            self.selected = (self.selected + 1).min(self.filtered_indices.len() - 1);
            self.details_scroll = 0;
        }
    }

    pub fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
        self.details_scroll = 0;
    }

    pub fn scroll_details_down(&mut self) {
        self.details_scroll = self.details_scroll.saturating_add(1);
    }
    pub fn scroll_details_up(&mut self) {
        self.details_scroll = self.details_scroll.saturating_sub(1);
    }

    pub fn cycle_status_filter(&mut self) {
        self.list_filter = self.list_filter.next();
        self.selected = 0;
        self.details_scroll = 0;
        self.refilter();
    }

    pub fn cycle_priority_filter(&mut self) {
        self.priority_filter = self.priority_filter.next();
        self.selected = 0;
        self.details_scroll = 0;
        self.refilter();
    }

    pub fn move_left_down(&mut self) {
        let len = self.left_list_len();
        if len > 0 {
            self.left_selected = (self.left_selected + 1).min(len - 1);
        }
    }

    pub fn move_left_up(&mut self) {
        self.left_selected = self.left_selected.saturating_sub(1);
    }

    // ── lists ─────────────────────────────────────────────────────────────────

    pub fn projects_list(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .projects
            .iter()
            .filter(|p| !p.is_deleted())
            .map(|p| p.name.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        names.sort();
        names
    }

    pub fn tags_list(&self) -> Vec<String> {
        let mut tags: Vec<String> = self
            .tasks
            .iter()
            .flat_map(|t| t.tags.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        tags.sort();
        tags
    }

    pub fn tasks_for_selected_project(&self) -> Vec<&Task> {
        let projects = self.projects_list();
        if let Some(proj_name) = projects.get(self.left_selected) {
            let proj_uuid = self
                .projects
                .iter()
                .find(|p| &p.name == proj_name && !p.is_deleted())
                .map(|p| p.uuid);
            self.tasks
                .iter()
                .filter(|t| proj_uuid.is_some() && t.project_id == proj_uuid)
                .collect()
        } else {
            vec![]
        }
    }

    pub fn tasks_for_selected_tag(&self) -> Vec<&Task> {
        let tags = self.tags_list();
        if let Some(tag) = tags.get(self.left_selected) {
            self.tasks.iter().filter(|t| t.tags.contains(tag)).collect()
        } else {
            vec![]
        }
    }

    pub fn left_list_len(&self) -> usize {
        match self.left_panel {
            LeftPanel::Tasks => self.filtered_indices.len(),
            LeftPanel::Projects => self.project_tree.len(),
            LeftPanel::Tags => self.tags_list().len(),
        }
    }

    pub fn pending_count(&self) -> usize {
        self.tasks.iter().filter(|t| !t.completed).count()
    }
    pub fn total_count(&self) -> usize {
        self.tasks.len()
    }

    fn load_visible(storage: &impl Storage) -> Result<Vec<Task>> {
        Ok(storage
            .load()?
            .into_iter()
            .filter(|t| !t.is_deleted())
            .collect())
    }
}
