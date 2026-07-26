use std::eprintln;

use crate::ui::widget::table::{table_view_model::CellSize, TableViewModel};
use iced::{Point, Rectangle};

#[derive(Debug)]
pub struct TableLayout
{
    /// cell widths in the row.
    pub cell_widths: Vec<f32>,

    /// cell height in the column.
    pub cell_heights: Vec<f32>,

    pub start_point: Point,
}

impl TableLayout 
{
    #[allow(unused)]
    pub fn column_count(&self)-> usize
    {
        return self.cell_widths.len();
    }

    #[allow(unused)]
    pub fn row_count(&self)-> usize
    {
        return self.cell_heights.len();
    }
}

impl TableLayout
{
    pub fn from_view_model_with_frame(view_model:&TableViewModel, frame:Rectangle)
        -> Option<Self>
    {
        if let CellSize::Fixed(default_size) = view_model.cell_default_size {
            if view_model.rows.is_empty() {
                return None;
            }
            let mut cell_widths = vec![];
            let mut cell_heights = vec![];
            let start_point = view_model.start_point;
            for y in 0..view_model.rows.len() {
                let height_offset = view_model.cell_height_offsets[y];
                cell_heights.push(default_size.height + height_offset);
                for x in 0..view_model.rows[0].len() {
                    let width_offset = view_model.cell_width_offsets[x];
                    cell_widths.push(default_size.width + width_offset);
                }
            }
            return Some(Self {
                cell_widths,
                cell_heights,
                start_point,
            });
        };

        if let CellSize::StretchToFit = view_model.cell_default_size {
            let cell_width_offsets:&Vec<f32> = &view_model.cell_width_offsets;
            let cell_height_offsets:&Vec<f32> = &view_model.cell_height_offsets;

            // measure cell default size
            let cell_width_offsets_sum:f32 = cell_width_offsets.iter().sum();
            let cell_height_offsets_sum:f32 = cell_height_offsets.iter().sum();

            let avaliable_frame_width:f32 = frame.width - cell_width_offsets_sum;
            let avaliable_frame_height:f32 = frame.height - cell_height_offsets_sum;

            if view_model.rows.is_empty() { return None };

            let row_count = view_model.rows.len();
            let column_count = view_model.rows[0].len();

            let cell_default_width:f32 = avaliable_frame_width / column_count as f32;
            let cell_default_height:f32 = avaliable_frame_height / row_count as f32;

            // assembly result
            let mut cell_widths = vec![];
            let mut cell_heights = vec![];
            let start_point = view_model.start_point;
            for y in 0..row_count {
                let height_offset = cell_height_offsets[y];
                cell_heights.push(cell_default_height + height_offset);
                for x in 0..column_count {
                    let width_offset = cell_width_offsets[x];
                    cell_widths.push(cell_default_width + width_offset);
                }
            }
            return Some(Self {
                cell_widths,
                cell_heights,
                start_point,
            });
        };

        eprintln!("Unsupported cell default size for table layout parse: {:?}", view_model.cell_default_size);
        return None;
    }
}
