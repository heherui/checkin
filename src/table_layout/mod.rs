use serde::{Deserialize, Serialize};

use crate::storage::PersonId;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TableLayout
{
    pub column_count: u32,
    pub row_count: u32,
    pub persons_mapper: Vec<SeatAssignment>,
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
