use serde::{Deserialize, Serialize};

/// JSON persistence model for saving attendance data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveData {
    pub table: TableSave,
    pub attendances: Vec<AttendanceSave>,
    pub marked: Vec<usize>,
}

/// Table dimensions for persistence.
///
/// `colomn_count` intentionally follows the external JSON contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableSave {
    pub colomn_count: u32,
    pub row_count: u32,
}

/// One attendance record with person name and table position.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttendanceSave {
    pub name: String,
    pub position: PositionSave,
}

/// Zero-based position in persistence payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositionSave {
    pub x: u32,
    pub y: u32,
}
