use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation};

use crate::core::AttendanceStatistics;

const CLASS_PANEL: &str = "statistics-panel";
const CLASS_TITLE: &str = "statistics-title";
const CLASS_SUMMARY: &str = "statistics-summary";
const CLASS_DETAIL: &str = "statistics-detail";

/// Compact statistics panel for attendance progress.
#[derive(Clone)]
pub struct StatisticsPanel {
    root: GtkBox,
    title_label: Label,
    summary_label: Label,
    detail_label: Label,
}

impl StatisticsPanel {
    /// Creates a statistics panel with initial values.
    pub fn new(initial: AttendanceStatistics) -> Self {
        let root = GtkBox::new(Orientation::Vertical, 2);
        root.add_css_class(CLASS_PANEL);

        let title_label = Label::new(Some("Attendance"));
        title_label.add_css_class(CLASS_TITLE);
        title_label.set_xalign(0.0);

        let summary_label = Label::new(None);
        summary_label.add_css_class(CLASS_SUMMARY);
        summary_label.set_xalign(0.0);

        let detail_label = Label::new(None);
        detail_label.add_css_class(CLASS_DETAIL);
        detail_label.set_xalign(0.0);
        detail_label.set_wrap(true);

        root.append(&title_label);
        root.append(&summary_label);
        root.append(&detail_label);

        let panel = Self {
            root,
            title_label,
            summary_label,
            detail_label,
        };
        panel.update(initial);
        panel
    }

    /// Returns the panel root widget.
    pub fn widget(&self) -> &GtkBox {
        &self.root
    }

    /// Primary summary label, kept for compatibility with external callers.
    pub fn summary_label(&self) -> Label {
        self.summary_label.clone()
    }

    /// Updates panel values from table statistics.
    pub fn update(&self, statistics: AttendanceStatistics) {
        let completion = statistics.completed_ratio_percent();
        self.summary_label.set_markup(&format!(
            "<b>{}%</b> completed ({}/{})",
            completion,
            statistics.completed_count(),
            statistics.active_total
        ));
        self.detail_label.set_markup(&format!(
            "Checked: <b>{}</b>  Unchecked: <b>{}</b>  Marked: <b>{}</b>  Blocked: <b>{}</b>  Total: <b>{}</b>",
            statistics.checked,
            statistics.unchecked,
            statistics.marked,
            statistics.blocked_total,
            statistics.total_cells()
        ));
    }

    /// Returns the title label for integration tests and advanced customization.
    pub fn title_label(&self) -> Label {
        self.title_label.clone()
    }
}
