use crate::widgets::table::{
    table_style::TableGridLine,
    view_model::TableViewModel,
    viewport::{Point, Rectangle},
};

/// The resolved geometry of a table.
///
/// Unlike [`TableGeometryMatrix`], this structure contains actual positions
/// after the table grid-line style has been applied.
///
/// In particular, [`TableGridLine::Sized`] inserts layout space between
/// adjacent cells, while [`TableGridLine::Visual`] does not.
#[derive(Debug, Clone)]
pub(super) struct TableGeometry
{
    pub column_starts: Vec<f32>,
    pub column_ends: Vec<f32>,
    pub row_starts: Vec<f32>,
    pub row_ends: Vec<f32>,
    pub grid_line_width: f32,
    pub sized_grid_line: bool,
}

/// A rectangular range of logical table cells.
///
/// `last_row` and `last_column` are inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CellRange
{
    pub first_row: usize,
    pub last_row: usize,
    pub first_column: usize,
    pub last_column: usize,
}

impl CellRange
{
    /// Creates a cell range.
    pub const fn new(
        first_row: usize,
        last_row: usize,
        first_column: usize,
        last_column: usize,
    ) -> Self
    {
        Self {
            first_row,
            last_row,
            first_column,
            last_column,
        }
    }
}

impl TableGeometry
{
    /// Builds resolved table geometry from a view model and table style.
    pub fn new(view_model: &TableViewModel, grid_line: TableGridLine) -> Self
    {
        let grid_line_width = grid_line.width().max(0.0);
        let sized_grid_line = grid_line.is_sized();

        let mut column_starts = Vec::with_capacity(view_model.columns_count());
        let mut column_ends = Vec::with_capacity(view_model.columns_count());

        let mut x = 0.0;

        for column in 0..view_model.columns_count() {
            column_starts.push(x);
            x += view_model.geometry_matrix.column_width(column);
            column_ends.push(x);
            if sized_grid_line && column + 1 < view_model.columns_count() {
                x += grid_line_width;
            }
        }

        let mut row_starts = Vec::with_capacity(view_model.rows_count());
        let mut row_ends = Vec::with_capacity(view_model.rows_count());
        let mut y = 0.0;
        for row in 0..view_model.rows_count() {
            row_starts.push(y);
            y += view_model.geometry_matrix.row_height(row);
            row_ends.push(y);
            if sized_grid_line && row + 1 < view_model.rows_count() {
                y += grid_line_width;
            }
        }

        Self {
            column_starts,
            column_ends,
            row_starts,
            row_ends,
            grid_line_width,
            sized_grid_line,
        }
    }

    /// Returns the number of columns represented by this geometry.
    pub fn columns_count(&self) -> usize
    {
        self.column_starts.len()
    }

    /// Returns the number of rows represented by this geometry.
    pub fn rows_count(&self) -> usize
    {
        self.row_starts.len()
    }

    /// Returns the width of a logical column's cell region.
    pub fn column_width(&self, column: usize) -> f32
    {
        self.column_ends[column] - self.column_starts[column]
    }

    /// Returns the height of a logical row's cell region.
    pub fn row_height(&self, row: usize) -> f32
    {
        self.row_ends[row] - self.row_starts[row]
    }

    /// Returns the left edge of a logical column.
    pub fn column_start(&self, column: usize) -> f32
    {
        self.column_starts[column]
    }

    /// Returns the right edge of a logical column.
    pub fn column_end(&self, column: usize) -> f32
    {
        self.column_ends[column]
    }

    /// Returns the top edge of a logical row.
    pub fn row_start(&self, row: usize) -> f32
    {
        self.row_starts[row]
    }

    /// Returns the bottom edge of a logical row.
    pub fn row_end(&self, row: usize) -> f32
    {
        self.row_ends[row]
    }

    /// Returns the actual position of the vertical grid line between two
    /// adjacent columns.
    ///
    /// For a visual grid line, the line is rendered over the logical
    /// boundary. For a sized grid line, the line is centered inside the
    /// layout space inserted between the two cells.
    pub fn vertical_line_position(&self, boundary_column: usize) -> f32
    {
        let boundary = self.column_end(boundary_column);

        if self.sized_grid_line {
            boundary + self.grid_line_width / 2.0
        } else {
            boundary
        }
    }

    /// Returns the actual position of the horizontal grid line between two
    /// adjacent rows.
    pub fn horizontal_line_position(&self, boundary_row: usize) -> f32
    {
        let boundary = self.row_end(boundary_row);

        if self.sized_grid_line {
            boundary + self.grid_line_width / 2.0
        } else {
            boundary
        }
    }

    /// Returns the bounds occupied by a range of logical cells.
    ///
    /// For sized grid lines, this also includes all grid-line layout space
    /// between the first and last cell.
    pub fn bounds(&self, range: CellRange) -> Rectangle
    {
        let x = self.column_start(range.first_column);
        let right = self.column_end(range.last_column);
        let y = self.row_start(range.first_row);
        let bottom = self.row_end(range.last_row);

        Rectangle::new(x, y, right - x, bottom - y)
    }

    /// Finds the range of cells intersecting a viewport.
    ///
    /// The returned range is cell-aligned. A viewport beginning or ending
    /// inside a cell includes that whole logical cell in the slice.
    pub fn cells_intersecting(&self, viewport: Rectangle) -> Option<CellRange>
    {
        if viewport.width <= 0.0
            || viewport.height <= 0.0
            || self.columns_count() == 0
            || self.rows_count() == 0
        {
            return None;
        }

        let viewport_right = viewport.x + viewport.width;
        let viewport_bottom = viewport.y + viewport.height;
        let first_column = Self::first_intersecting(
            &self.column_starts,
            &self.column_ends,
            viewport.x,
            viewport_right,
        )?;

        let last_column = Self::last_intersecting(
            &self.column_starts,
            &self.column_ends,
            viewport.x,
            viewport_right,
        )?;

        let first_row = Self::first_intersecting(
            &self.row_starts,
            &self.row_ends,
            viewport.y,
            viewport_bottom,
        )?;

        let last_row = Self::last_intersecting(
            &self.row_starts,
            &self.row_ends,
            viewport.y,
            viewport_bottom,
        )?;

        Some(CellRange::new(
            first_row,
            last_row,
            first_column,
            last_column,
        ))
    }

    /// Returns the first cell whose region intersects the given interval.
    fn first_intersecting(starts: &[f32], ends: &[f32], start: f32, end: f32) -> Option<usize>
    {
        if end <= starts[0] || start >= ends[ends.len() - 1] {
            return None;
        }

        let index = ends.partition_point(|&value| value <= start);

        if index >= starts.len() {
            return None;
        }

        if starts[index] >= end {
            None
        } else {
            Some(index)
        }
    }

    /// Returns the last cell whose region intersects the given interval.
    fn last_intersecting(starts: &[f32], ends: &[f32], start: f32, end: f32) -> Option<usize>
    {
        if end <= starts[0] || start >= ends[ends.len() - 1] {
            return None;
        }

        let index = starts.partition_point(|&value| value < end);

        if index == 0 {
            return None;
        }

        let index = index - 1;

        if ends[index] <= start {
            None
        } else {
            Some(index)
        }
    }

    /// Converts a table-space point into slice coordinates.
    pub fn to_slice_point(&self, point: Point, slice_bounds: Rectangle) -> Point
    {
        Point::new(point.x - slice_bounds.x, point.y - slice_bounds.y)
    }

    /// Returns the configured grid-line width.
    pub fn line_width(&self) -> f32
    {
        self.grid_line_width
    }
}
