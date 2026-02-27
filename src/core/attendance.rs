use super::{Position, Subject, Table};
use crate::utilities::SystemTimeExt;
use std::{collections::HashMap, time::SystemTime};

/// Domain-level check-in result for a seat/person.
///
/// This model is intentionally UI-agnostic and can be reused by
/// persistence, network sync, and business logic layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AttendanceStatus {
    Checked,
    #[default]
    Unchecked,
    Marked,
}

impl AttendanceStatus {
    pub const ALL: [Self; 3] = [Self::Checked, Self::Unchecked, Self::Marked];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Checked => "Checked",
            Self::Unchecked => "Unchecked",
            Self::Marked => "Marked",
        }
    }

    pub const fn css_class(self) -> &'static str {
        match self {
            Self::Checked => "status-checked",
            Self::Unchecked => "status-unchecked",
            Self::Marked => "status-marked",
        }
    }

    pub const fn background_color(self) -> &'static str {
        match self {
            Self::Checked => "#22c55e",
            Self::Unchecked => "#ff0000",
            Self::Marked => "#facc15",
        }
    }

    pub const fn background_alpha(self) -> f32 {
        match self {
            Self::Checked => 0.45,
            Self::Unchecked => 0.45,
            Self::Marked => 0.45,
        }
    }

    pub const fn background_rgb(self) -> (u8, u8, u8) {
        match self {
            Self::Checked => (34, 197, 94),
            Self::Unchecked => (239, 68, 68),
            Self::Marked => (250, 204, 21),
        }
    }

    pub const fn foreground_color(self) -> &'static str {
        match self {
            Self::Checked => "#052e16",
            Self::Unchecked => "#451010",
            Self::Marked => "#422006",
        }
    }
}

/// Aggregated attendance metrics for the current table snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttendanceStatistics {
    pub checked: u32,
    pub unchecked: u32,
    pub marked: u32,
    pub active_total: u32,
    pub blocked_total: u32,
}

impl AttendanceStatistics {
    /// Number of active seats that are no longer pending.
    pub const fn completed_count(self) -> u32 {
        self.checked + self.marked
    }

    /// All visible cells (active + blocked), excluding transparent placeholders.
    pub const fn total_cells(self) -> u32 {
        self.active_total + self.blocked_total
    }

    /// Completion ratio (`checked + marked`) across active seats.
    pub const fn completed_ratio_percent(self) -> u32 {
        if self.active_total == 0 {
            0
        } else {
            (self.completed_count() * 100) / self.active_total
        }
    }
}

/// Mutable attendance statuses keyed by position.
///
/// This model keeps check-in state in the domain layer so UI components can
/// reuse the same data flow for future persistence and sync features.
#[derive(Debug, Clone, Default)]
pub struct AttendanceBook {
    statuses: HashMap<Position, AttendanceStatus>,
}

impl AttendanceBook {
    pub fn new(table: &Table) -> Self {
        let mut statuses = HashMap::new();
        for position in table.iter_positions() {
            if !table.is_inert(position) {
                statuses.insert(position, AttendanceStatus::Unchecked);
            }
        }

        Self { statuses }
    }

    pub fn status_at(&self, position: Position) -> Option<AttendanceStatus> {
        self.statuses.get(&position).copied()
    }

    /// Ensures attendance entries match current table kinds after table edits.
    pub fn reconcile_with_table(&mut self, table: &Table) {
        self.statuses
            .retain(|position, _| table.contains(*position) && !table.is_inert(*position));

        for position in table.iter_positions() {
            if !table.is_inert(position) {
                self.statuses.entry(position).or_default();
            }
        }
    }

    /// Updates status for an active seat. Returns `true` only when a real change happened.
    pub fn update_status(
        &mut self,
        table: &Table,
        position: Position,
        next_status: AttendanceStatus,
    ) -> bool {
        if !table.contains(position) || table.is_inert(position) {
            return false;
        }

        let current = self.statuses.entry(position).or_default();
        if *current == next_status {
            return false;
        }

        *current = next_status;
        true
    }

    pub fn statistics(&self, table: &Table) -> AttendanceStatistics {
        let mut checked = 0;
        let mut unchecked = 0;
        let mut marked = 0;

        for position in table.iter_positions() {
            if table.is_inert(position) {
                continue;
            }

            match self.status_at(position).unwrap_or_default() {
                AttendanceStatus::Checked => checked += 1,
                AttendanceStatus::Unchecked => unchecked += 1,
                AttendanceStatus::Marked => marked += 1,
            }
        }

        AttendanceStatistics {
            checked,
            unchecked,
            marked,
            active_total: table.active_cells(),
            blocked_total: table.blocked_cells(),
        }
    }

    /// Builds a Chinese export string for sharing check-in progress.
    pub fn build_export_text_zh(&self, table: &Table, time: &SystemTime) -> String {
        let statistics = self.statistics(table);
        let time = format!("{}({})", time.formatted_string(), time.period_string());
        let unchecked_names = self.names_by_status(table, AttendanceStatus::Unchecked);
        let marked_names = self.names_by_status(table, AttendanceStatus::Marked);

        format!(
            "{}\n[未签到 {}人 已签到{}%]\n{}\n[请假 {}人]\n{}",
            time,
            statistics.unchecked,
            statistics.completed_ratio_percent(),
            Self::format_names(&unchecked_names),
            statistics.marked,
            Self::format_names(&marked_names),
        )
    }

    fn names_by_status(&self, table: &Table, status: AttendanceStatus) -> Vec<String> {
        let mut names = Vec::new();
        for position in table.iter_positions() {
            if self.status_at(position) != Some(status) {
                continue;
            }

            if let Some(Subject::Some(name)) = table.subject_at(position) {
                names.push(name.clone());
            }
        }
        names
    }

    fn format_names(names: &[String]) -> String {
        if names.is_empty() {
            String::from("")
        } else {
            names.join(", ")
        }
    }
}
