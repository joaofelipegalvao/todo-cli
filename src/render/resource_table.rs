//! Terminal rendering for resource lists.

use colored::Colorize;

use crate::models::Resource;
use crate::render::formatting::truncate;

pub struct ResourceTableLayout {
    pub title_w: usize,
    pub type_w: usize,
    pub url_w: usize,
    pub tags_w: usize,
    pub desc_w: usize,
    pub show_type: bool,
    pub show_url: bool,
    pub show_tags: bool,
    pub show_desc: bool,
    pub show_notes: bool,
    pub total_w: usize,
}

impl ResourceTableLayout {
    pub fn new(resources: &[&Resource], notes: &[crate::models::Note]) -> Self {
        let show_type = resources.iter().any(|r| r.resource_type.is_some());
        let type_w = if show_type { 7 } else { 0 }; // "article" = 7

        let title_w = resources
            .iter()
            .map(|r| r.title.len())
            .max()
            .unwrap_or(5)
            .clamp(5, 32);

        let url_w = resources
            .iter()
            .filter_map(|r| r.url.as_deref().map(|u| u.len()))
            .max()
            .unwrap_or(0)
            .clamp(3, 40);

        let tags_w = resources
            .iter()
            .map(|r| {
                if r.tags.is_empty() {
                    0
                } else {
                    r.tags.join(", ").len()
                }
            })
            .max()
            .unwrap_or(0)
            .clamp(4, 24);

        let desc_w = resources
            .iter()
            .filter_map(|r| r.description.as_deref().map(|d| d.len()))
            .max()
            .unwrap_or(0)
            .min(40);

        let show_url = resources.iter().any(|r| r.url.is_some());
        let show_tags = resources.iter().any(|r| !r.tags.is_empty());
        let show_desc = resources.iter().any(|r| r.description.is_some());
        let show_notes = resources.iter().any(|r| {
            notes
                .iter()
                .any(|n| !n.is_deleted() && n.resource_ids.contains(&r.uuid))
        });

        let mut total_w = 4 + 2 + title_w;
        if show_type {
            total_w += 2 + type_w;
        }
        if show_url {
            total_w += 2 + url_w;
        }
        if show_tags {
            total_w += 2 + tags_w;
        }
        if show_desc {
            total_w += 2 + desc_w;
        }
        if show_notes {
            total_w += 2 + 5;
        }

        Self {
            title_w,
            type_w,
            url_w,
            tags_w,
            desc_w,
            show_type,
            show_url,
            show_tags,
            show_desc,
            show_notes,
            total_w,
        }
    }

    pub fn display_header(&self) {
        print!("{:>4}  ", "ID".dimmed());
        print!("{:<title_w$}", "Title".dimmed(), title_w = self.title_w);
        if self.show_type {
            print!("  {:<type_w$}", "Type".dimmed(), type_w = self.type_w);
        }
        if self.show_url {
            print!("  {:<url_w$}", "URL".dimmed(), url_w = self.url_w);
        }
        if self.show_tags {
            print!("  {:<tags_w$}", "Tags".dimmed(), tags_w = self.tags_w);
        }
        if self.show_desc {
            print!(
                "  {:<desc_w$}",
                "Description".dimmed(),
                desc_w = self.desc_w
            );
        }
        if self.show_notes {
            print!("  {:^5}", "Notes".dimmed());
        }
        println!();
        println!("{}", "─".repeat(self.total_w).dimmed());
    }

    pub fn display_row(&self, id: usize, resource: &Resource, notes: &[crate::models::Note]) {
        let title_str = truncate(&resource.title, self.title_w);
        let type_str = resource
            .resource_type
            .map(|t| t.label().to_string())
            .unwrap_or_default();
        let url_str = resource
            .url
            .as_deref()
            .map(|u| truncate(u, self.url_w))
            .unwrap_or_default();
        let tags_str = if resource.tags.is_empty() {
            String::new()
        } else {
            truncate(&resource.tags.join(", "), self.tags_w)
        };
        let desc_str = resource
            .description
            .as_deref()
            .map(|d| truncate(d, self.desc_w))
            .unwrap_or_default();

        print!("{:>4}  ", format!("#{}", id).dimmed());
        print!("{:<title_w$}", title_str, title_w = self.title_w);
        if self.show_type {
            print!("  {:<type_w$}", type_str.yellow(), type_w = self.type_w);
        }
        if self.show_url {
            print!("  {:<url_w$}", url_str.dimmed(), url_w = self.url_w);
        }
        if self.show_tags {
            print!("  {:<tags_w$}", tags_str.cyan(), tags_w = self.tags_w);
        }
        if self.show_desc {
            print!("  {:<desc_w$}", desc_str.dimmed(), desc_w = self.desc_w);
        }
        if self.show_notes {
            let count = notes
                .iter()
                .filter(|n| !n.is_deleted() && n.resource_ids.contains(&resource.uuid))
                .count();
            let notes_str = if count > 0 {
                format!("{:^5}", count).dimmed().to_string()
            } else {
                format!("{:^5}", "—").dimmed().to_string()
            };
            print!("  {}", notes_str);
        }
        println!();
    }

    pub fn display_separator(&self) {
        println!("{}", "─".repeat(self.total_w).dimmed());
    }
}

pub fn display_resources(resources: &[&Resource], notes: &[crate::models::Note]) {
    println!("\nResources:\n");
    let layout = ResourceTableLayout::new(resources, notes);
    layout.display_header();
    for (i, resource) in resources.iter().enumerate() {
        layout.display_row(i + 1, resource, notes);
    }
    layout.display_separator();
    println!();
}
