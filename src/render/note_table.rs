//! Terminal rendering for note lists.

use colored::Colorize;

use crate::models::{Note, Project};
use crate::render::formatting::truncate;

pub struct NoteTableLayout {
    pub body_w: usize,
    pub proj_w: usize,
    pub lang_w: usize,
    pub tags_w: usize,
    pub show_project: bool,
    pub show_lang: bool,
    pub show_tags: bool,
    pub show_resources: bool,
    pub total_w: usize,
}

impl NoteTableLayout {
    pub fn new(
        notes: &[&Note],
        projects: &[Project],
        resources: &[crate::models::Resource],
    ) -> Self {
        let body_w = notes
            .iter()
            .map(|n| {
                n.title
                    .as_deref()
                    .unwrap_or_else(|| {
                        let b = n.body.as_str();
                        if b.len() > 60 { &b[..60] } else { b }
                    })
                    .len()
            })
            .max()
            .unwrap_or(10)
            .clamp(10, 48);

        let proj_w = notes
            .iter()
            .filter_map(|n| {
                n.project_id
                    .and_then(|pid| projects.iter().find(|p| p.uuid == pid))
                    .map(|p| p.name.len())
            })
            .max()
            .unwrap_or(0)
            .clamp(7, 24);

        let lang_w = notes
            .iter()
            .filter_map(|n| n.language.as_deref().map(|l| l.len()))
            .max()
            .unwrap_or(0)
            .clamp(4, 16);

        let tags_w = notes
            .iter()
            .map(|n| {
                if n.tags.is_empty() {
                    0
                } else {
                    n.tags.join(", ").len()
                }
            })
            .max()
            .unwrap_or(0)
            .clamp(4, 24);

        let show_project = notes.iter().any(|n| n.project_id.is_some());
        let show_lang = notes.iter().any(|n| n.language.is_some());
        let show_tags = notes.iter().any(|n| !n.tags.is_empty());
        let show_resources = notes.iter().any(|n| {
            n.resource_ids
                .iter()
                .any(|rid| resources.iter().any(|r| !r.is_deleted() && r.uuid == *rid))
        });

        let mut total_w = 4 + 2 + body_w;
        if show_project {
            total_w += 2 + proj_w;
        }
        if show_lang {
            total_w += 2 + lang_w;
        }
        if show_tags {
            total_w += 2 + tags_w;
        }
        if show_resources {
            total_w += 2 + 3;
        }

        Self {
            body_w,
            proj_w,
            lang_w,
            tags_w,
            show_project,
            show_lang,
            show_tags,
            show_resources,
            total_w,
        }
    }

    pub fn display_header(&self) {
        print!("{:>4}  ", "ID".dimmed());
        print!("{:<body_w$}", "Note".dimmed(), body_w = self.body_w);
        if self.show_project {
            print!("  {:<proj_w$}", "Project".dimmed(), proj_w = self.proj_w);
        }
        if self.show_lang {
            print!("  {:<lang_w$}", "Lang".dimmed(), lang_w = self.lang_w);
        }
        if self.show_tags {
            print!("  {:<tags_w$}", "Tags".dimmed(), tags_w = self.tags_w);
        }
        if self.show_resources {
            print!("  {:^3}", "Res".dimmed());
        }
        println!();
        println!("{}", "─".repeat(self.total_w).dimmed());
    }

    pub fn display_row(
        &self,
        id: usize,
        note: &Note,
        projects: &[Project],
        resources: &[crate::models::Resource],
    ) {
        let preview = note.title.as_deref().unwrap_or_else(|| {
            let b = note.body.as_str();
            if b.len() > 60 { &b[..60] } else { b }
        });
        let preview_str = truncate(preview, self.body_w);

        let proj_str = note
            .project_id
            .and_then(|pid| projects.iter().find(|p| p.uuid == pid))
            .map(|p| truncate(&p.name, self.proj_w))
            .unwrap_or_default();

        let lang_str = note
            .language
            .as_deref()
            .map(|l| truncate(l, self.lang_w))
            .unwrap_or_default();

        let tags_str = if note.tags.is_empty() {
            String::new()
        } else {
            truncate(&note.tags.join(", "), self.tags_w)
        };

        print!("{:>4}  ", format!("#{}", id).dimmed());
        print!("{:<body_w$}", preview_str, body_w = self.body_w);
        if self.show_project {
            print!("  {:<proj_w$}", proj_str.magenta(), proj_w = self.proj_w);
        }
        if self.show_lang {
            print!("  {:<lang_w$}", lang_str.yellow(), lang_w = self.lang_w);
        }
        if self.show_tags {
            print!("  {:<tags_w$}", tags_str.dimmed(), tags_w = self.tags_w);
        }
        if self.show_resources {
            let count = note
                .resource_ids
                .iter()
                .filter(|rid| resources.iter().any(|r| !r.is_deleted() && r.uuid == **rid))
                .count();
            let res_str = if count > 0 {
                format!("{:^3}", count).dimmed().to_string()
            } else {
                format!("{:^3}", "—").dimmed().to_string()
            };
            print!("  {}", res_str);
        }
        println!();
    }

    pub fn display_separator(&self) {
        println!("{}", "─".repeat(self.total_w).dimmed());
    }
}

pub fn display_notes(notes: &[&Note], projects: &[Project], resources: &[crate::models::Resource]) {
    println!("\nNotes:\n");
    let layout = NoteTableLayout::new(notes, projects, resources);
    layout.display_header();
    for (i, note) in notes.iter().enumerate() {
        layout.display_row(i + 1, note, projects, resources);
    }
    layout.display_separator();
    println!();
}
