use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation};

use crate::core::{AppMode, AttendanceStatus, Subject};
use crate::ui::cell_model::{from_subject, Cell};

const CLASS_CELL: &str = "table-cell";
const CLASS_SURFACE: &str = "cell-surface";
const CLASS_BLOCKED: &str = "blocked";
const CLASS_TRANSPARENT: &str = "transparent";
const CLASS_EDIT_PENDING: &str = "edit-pending";

/// View model for a single table cell (container + rendered surface).
pub struct TableCell {
    container: GtkBox,
    surface: Label,
}

impl TableCell {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 0);
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_halign(Align::Fill);
        container.set_valign(Align::Fill);
        container.add_css_class(CLASS_CELL);

        let surface = Label::new(None);
        surface.set_hexpand(true);
        surface.set_vexpand(true);
        surface.set_halign(Align::Fill);
        surface.set_valign(Align::Fill);
        surface.set_xalign(0.5);
        surface.set_yalign(0.5);
        surface.set_justify(gtk4::Justification::Center);
        surface.add_css_class(CLASS_SURFACE);

        container.append(&surface);

        Self { container, surface }
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    pub fn surface(&self) -> &Label {
        &self.surface
    }

    pub fn render_to(
        container: &GtkBox,
        surface: &Label,
        mode: AppMode,
        subject: Option<&Subject>,
        status: Option<AttendanceStatus>,
    ) {
        match mode {
            AppMode::CheckIn => Self::render_check_mode(container, surface, subject, status),
            AppMode::Edit => Self::render_edit_mode(container, surface, subject),
        }
    }

    fn clear_styles(container: &GtkBox, surface: &Label) {
        container.remove_css_class(CLASS_TRANSPARENT);
        surface.remove_css_class(CLASS_BLOCKED);
        surface.remove_css_class(CLASS_TRANSPARENT);
        surface.remove_css_class(CLASS_EDIT_PENDING);
        for status in AttendanceStatus::ALL {
            surface.remove_css_class(status.css_class());
        }
    }

    fn render_check_mode(
        container: &GtkBox,
        surface: &Label,
        subject: Option<&Subject>,
        status: Option<AttendanceStatus>,
    ) {
        Self::clear_styles(container, surface);

        let cell = from_subject(subject);
        let check_color = cell.render_color_check_mode();

        if check_color == "#475569" {
            surface.add_css_class(CLASS_BLOCKED);
        }
        if check_color == "transparent" {
            container.add_css_class(CLASS_TRANSPARENT);
            surface.add_css_class(CLASS_TRANSPARENT);
        }
        Self::set_check_mode_text(surface, subject);
        if !subject.is_some_and(Subject::is_inert) {
            surface.add_css_class(status.unwrap_or_default().css_class());
        }
    }

    fn render_edit_mode(container: &GtkBox, surface: &Label, subject: Option<&Subject>) {
        Self::clear_styles(container, surface);

        let cell = from_subject(subject);
        let edit_color = cell.render_color_edit_mode();
        if edit_color == "#475569" {
            surface.add_css_class(CLASS_BLOCKED);
        } else {
            surface.add_css_class(CLASS_EDIT_PENDING);
        }
        Self::set_edit_mode_text(surface, &*cell);
    }

    fn set_check_mode_text(surface: &Label, subject: Option<&Subject>) {
        match subject {
            Some(Subject::Some(name)) => {
                // Escape user-provided text before embedding in markup.
                let escaped = glib::markup_escape_text(name);
                surface.set_markup(&format!("<b>{escaped}</b>"));
            }
            Some(Subject::Block(name)) => {
                let trimmed = name.trim();
                if trimmed.is_empty() {
                    surface.set_markup("<span foreground='#cbd5e1'>-</span>");
                    return;
                }
                let escaped = glib::markup_escape_text(trimmed);
                surface.set_markup(&format!("<span foreground='#cbd5e1'>{escaped}</span>"));
            }
            Some(Subject::Transparent) => {
                surface.set_markup("");
            }
            None => {
                surface.set_markup("<span foreground='#6b7280'>-</span>");
            }
        }
    }

    fn set_edit_mode_text(surface: &Label, cell: &dyn Cell) {
        let type_name = cell.type_name();
        let maybe_text = cell
            .has_name()
            .map(|name| name.trim())
            .filter(|name| !name.is_empty());
        let foreground = if type_name == "Active" {
            "#334155"
        } else {
            "#e2e8f0"
        };
        let content = match (type_name, maybe_text) {
            ("Transparent", Some(text)) => {
                let escaped = glib::markup_escape_text(text);
                format!("[Transparent] {escaped}")
            }
            ("Transparent", None) => String::from("[Transparent]"),
            (_, Some(text)) => glib::markup_escape_text(text).to_string(),
            (_, None) => String::new(),
        };
        let markup = format!("<span foreground='{foreground}'>{content}</span>");
        surface.set_markup(&markup);
    }
}
