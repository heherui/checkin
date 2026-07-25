#[derive(Debug)]
pub struct TableViewModel
{
    pub(crate) cell_default_width: f32,
    pub(crate) cell_default_height: f32,

    pub(crate) cell_width_offsets: Vec<f32>,
    pub(crate) cell_height_offsets: Vec<f32>,

    pub(crate) start_point: Point,

    pub(crate) rows: Vec<Vec<TableCellViewModel>>,
}
#[derive(Debug)]
pub struct TableCellViewModel
{
    pub(crate) label: String,
    pub(crate) background_color: Color,
    pub(crate) border_color: Option<Color>,
}

use crate::storage::{PersonId, TableData};
use iced::{Color, Point};
use std::collections::HashMap;

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
                    background_color: Color::from_rgb8(222, 145, 145),
                    border_color: Some(Color::from_rgb8(231, 103, 103))
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

        let mut cell_width_offsets = vec![0.0; row_count as usize - 1];
        let mut cell_height_offsets = vec![0.0; column_count as usize - 1];

        cell_width_offsets[2] = 50.0;
        cell_height_offsets[2] = 50.0;

        TableViewModel {
            rows,
            start_point: Point { x: 0.0, y: 0.0 },
            cell_default_width: 80.0,
            cell_default_height: 50.0,
            cell_width_offsets,
            cell_height_offsets,
        }
    }
}
