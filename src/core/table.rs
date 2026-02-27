use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

use rand::prelude::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// A rectangular table layout.
/// `subjects` only stores explicitly assigned positions.
/// Any missing position is treated as an empty active seat.
#[derive(Debug, Clone)]
pub struct Table {
    row_count: u32,
    column_count: u32,
    subjects: HashMap<Position, Subject>,
}

/// Zero-based table coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

/// Data rendered inside a cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Subject {
    Transparent,
    Block(String),
    Some(String),
}

impl Subject {
    pub fn is_transparent(&self) -> bool {
        matches!(self, Self::Transparent)
    }

    pub fn is_blocked(&self) -> bool {
        matches!(self, Self::Block(_))
    }

    pub fn is_inert(&self) -> bool {
        self.is_transparent() || self.is_blocked()
    }

    pub fn name(&self) -> Option<&String> {
        match self {
            Self::Transparent => None,
            Self::Block(name) | Self::Some(name) => Some(name),
        }
    }
}

/// Canonical domain kind for each table cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellKind {
    Active,
    Blocked,
    Transparent,
}

impl Table {
    /// Creates a table and normalizes subjects into a position-indexed map.
    ///
    /// Out-of-bounds subjects are discarded and duplicate coordinates keep the last value.
    pub fn new(row_count: u32, column_count: u32, subjects: Vec<(Position, Subject)>) -> Self {
        let mut subject_map = HashMap::with_capacity(subjects.len());
        for (position, subject) in subjects {
            if position.x < column_count && position.y < row_count {
                subject_map.insert(position, subject);
            }
        }

        Self {
            row_count,
            column_count,
            subjects: subject_map,
        }
    }

    pub const fn row_count(&self) -> u32 {
        self.row_count
    }

    pub const fn column_count(&self) -> u32 {
        self.column_count
    }

    pub const fn total_cells(&self) -> u32 {
        self.row_count * self.column_count
    }

    pub fn contains(&self, position: Position) -> bool {
        position.x < self.column_count && position.y < self.row_count
    }

    pub fn subject_at(&self, position: Position) -> Option<&Subject> {
        self.subjects.get(&position)
    }

    pub fn subject_at_owned(&self, position: Position) -> Option<Subject> {
        self.subject_at(position).cloned()
    }

    pub fn set_subject(&mut self, position: Position, subject: Option<Subject>) -> bool {
        if !self.contains(position) {
            return false;
        }

        let normalized = Self::normalize_subject(subject);
        if self.subjects.get(&position) == normalized.as_ref() {
            return false;
        }

        match normalized {
            Some(subject) => {
                self.subjects.insert(position, subject);
            }
            None => {
                self.subjects.remove(&position);
            }
        }
        true
    }

    pub fn cell_kind(&self, position: Position) -> Option<CellKind> {
        if !self.contains(position) {
            return None;
        }

        let kind = match self.subject_at(position) {
            Some(subject) if subject.is_transparent() => CellKind::Transparent,
            Some(subject) if subject.is_blocked() => CellKind::Blocked,
            _ => CellKind::Active,
        };
        Some(kind)
    }

    pub fn is_inert(&self, position: Position) -> bool {
        matches!(
            self.cell_kind(position),
            Some(CellKind::Blocked) | Some(CellKind::Transparent)
        )
    }

    pub fn blocked_cells(&self) -> u32 {
        self.subjects
            .values()
            .filter(|subject| subject.is_blocked())
            .count() as u32
    }

    pub fn transparent_cells(&self) -> u32 {
        self.subjects
            .values()
            .filter(|subject| subject.is_transparent())
            .count() as u32
    }

    pub fn active_cells(&self) -> u32 {
        self.total_cells()
            .saturating_sub(self.blocked_cells() + self.transparent_cells())
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.row_count)
            .flat_map(move |y| (0..self.column_count).map(move |x| Position { x, y }))
    }

    pub fn add_row(&mut self) {
        self.row_count = self.row_count.saturating_add(1);
    }

    pub fn add_column(&mut self) {
        self.column_count = self.column_count.saturating_add(1);
    }

    pub fn remove_row(&mut self, row_index: u32) -> bool {
        if row_index >= self.row_count || self.row_count <= 1 {
            return false;
        }

        let mut next = HashMap::with_capacity(self.subjects.len());
        for (position, subject) in std::mem::take(&mut self.subjects) {
            if position.y == row_index {
                continue;
            }

            let y = if position.y > row_index {
                position.y - 1
            } else {
                position.y
            };
            next.insert(Position { x: position.x, y }, subject);
        }

        self.subjects = next;
        self.row_count -= 1;
        true
    }

    pub fn remove_column(&mut self, column_index: u32) -> bool {
        if column_index >= self.column_count || self.column_count <= 1 {
            return false;
        }

        let mut next = HashMap::with_capacity(self.subjects.len());
        for (position, subject) in std::mem::take(&mut self.subjects) {
            if position.x == column_index {
                continue;
            }

            let x = if position.x > column_index {
                position.x - 1
            } else {
                position.x
            };
            next.insert(Position { x, y: position.y }, subject);
        }

        self.subjects = next;
        self.column_count -= 1;
        true
    }

    pub fn write_config(&self, config_file: &Path) -> io::Result<()> {
        let payload = AppConfigFile {
            default_table: TableConfig::from_table(self),
        };
        let text = serde_json::to_string_pretty(&payload).map_err(io::Error::other)?;
        fs::write(config_file, text)
    }

    pub fn load_config(config_file: &Path) -> io::Result<Self> {
        let text = fs::read_to_string(config_file)?;
        if let Ok(payload) = serde_json::from_str::<AppConfigFile>(&text) {
            return Ok(payload.default_table.into_table());
        }

        let payload: TableConfig = serde_json::from_str(&text).map_err(io::Error::other)?;
        Ok(payload.into_table())
    }

    pub fn default_table() -> Self {
        const ROW_COUNT: u32 = 5;
        const COLUMN_COUNT: u32 = 6;
        const NAMES: [&str; 24] = [
            "Alice", "Ben", "Cindy", "Dylan", "Ethan", "Fiona", "Gavin", "Helen", "Ivy", "Jason",
            "Kira", "Leo", "Mila", "Nora", "Owen", "Penny", "Quinn", "Ruby", "Sam", "Tina", "Uma",
            "Vince", "Wendy", "Zack",
        ];

        let mut rng = rand::thread_rng();
        let mut subjects = Vec::new();

        for y in 0..ROW_COUNT {
            for x in 0..COLUMN_COUNT {
                let roll = rng.gen_range(0..10);
                let subject = if roll == 0 {
                    Subject::Transparent
                } else if roll <= 2 {
                    Subject::Block(format!("Block {}", rng.gen_range(1..=9)))
                } else {
                    let name = NAMES
                        .choose(&mut rng)
                        .map_or_else(|| String::from("Guest"), |name| (*name).to_owned());
                    Subject::Some(name)
                };
                subjects.push((Position { x, y }, subject));
            }
        }

        Self::new(ROW_COUNT, COLUMN_COUNT, subjects)
    }

    fn normalize_subject(subject: Option<Subject>) -> Option<Subject> {
        match subject {
            Some(Subject::Some(name)) => {
                let name = name.trim().to_owned();
                if name.is_empty() {
                    None
                } else {
                    Some(Subject::Some(name))
                }
            }
            Some(Subject::Block(name)) => Some(Subject::Block(name.trim().to_owned())),
            Some(Subject::Transparent) => Some(Subject::Transparent),
            None => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppConfigFile {
    default_table: TableConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TableConfig {
    row_count: u32,
    column_count: u32,
    subjects: Vec<TableConfigSubject>,
}

impl TableConfig {
    fn from_table(table: &Table) -> Self {
        let mut subjects = Vec::new();
        for position in table.iter_positions() {
            if let Some(subject) = table.subject_at(position) {
                subjects.push(TableConfigSubject::from_subject(position, subject));
            }
        }

        Self {
            row_count: table.row_count(),
            column_count: table.column_count(),
            subjects,
        }
    }

    fn into_table(self) -> Table {
        let subjects = self
            .subjects
            .into_iter()
            .map(TableConfigSubject::into_subject)
            .collect();
        Table::new(self.row_count, self.column_count, subjects)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TableConfigSubject {
    x: u32,
    y: u32,
    kind: TableConfigCellKind,
    name: Option<String>,
}

impl TableConfigSubject {
    fn from_subject(position: Position, subject: &Subject) -> Self {
        let (kind, name) = match subject {
            Subject::Transparent => (TableConfigCellKind::Transparent, None),
            Subject::Block(name) => (TableConfigCellKind::Blocked, Some(name.clone())),
            Subject::Some(name) => (TableConfigCellKind::Active, Some(name.clone())),
        };

        Self {
            x: position.x,
            y: position.y,
            kind,
            name,
        }
    }

    fn into_subject(self) -> (Position, Subject) {
        let subject = match self.kind {
            TableConfigCellKind::Transparent => Subject::Transparent,
            TableConfigCellKind::Blocked => Subject::Block(self.name.unwrap_or_default()),
            TableConfigCellKind::Active => Subject::Some(self.name.unwrap_or_default()),
        };

        (
            Position {
                x: self.x,
                y: self.y,
            },
            subject,
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TableConfigCellKind {
    Active,
    Blocked,
    Transparent,
}
