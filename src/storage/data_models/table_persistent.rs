use serde::{Deserialize, Serialize};

use crate::storage::data_models::personnel::{Person, PersonId};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TableData
{
    pub personnel: Vec<Person>,
    pub table_layout: TableLayout,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TableLayout
{
    pub column_count: u32,
    pub row_count: u32,
    pub seats_assignment: Vec<SeatAssignment>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SeatAssignment
{
    pub coordinate: TableCoordinate,
    pub person_id: PersonId,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TableCoordinate
{
    pub x: u32,
    pub y: u32,
}
