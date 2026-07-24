#[derive(Debug)]
pub struct TableViewModel
{
    pub(crate) row_count: u32,
    pub(crate) column_count: u32,
    pub(crate) rows: Vec<Vec<TableCellViewModel>>,
}

#[derive(Debug)]
pub struct TableCellViewModel
{
    pub(crate) label: String,
}


use std::collections::HashMap;

use crate::storage::{PersonId, TableData};

impl TableViewModel
{
    pub fn load_from(table_data: TableData) -> Self
    {
        let row_count = table_data.table_layout.row_count;
        let column_count = table_data.table_layout.column_count;

        let mut rows = Vec::with_capacity(row_count as usize);
        for _ in 0..row_count {
            let mut row = Vec::with_capacity(column_count as usize);
            for _ in 0..column_count {
                row.push(TableCellViewModel {
                    label: String::new(),
                });
            }
            rows.push(row);
        }

        let person_map: HashMap<PersonId, &str> = table_data
            .personnel
            .iter()
            .map(|p| (p.id, p.name.as_str()))
            .collect();

        // Fill cells that have a seat assignment
        for assignment in table_data.table_layout.seats_assignment {
            let x = assignment.coordinate.x.saturating_sub(1) as usize;
            let y = assignment.coordinate.y.saturating_sub(1) as usize;

            if y < rows.len() && x < rows[y].len() {
                if let Some(name) = person_map.get(&assignment.person_id) {
                    rows[y][x].label = name.to_string();
                }
            }
        }

        TableViewModel {
            row_count,
            column_count,
            rows,
        }
    }
}
