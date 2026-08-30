/// The geometry matrix of a table.
///
/// The geometry matrix stores the size of every column and row. It does not
/// store absolute positions. Positions can be derived from the accumulated
/// widths and heights.
///
/// The column at index `i` has width `columns_width[i]`.
/// The row at index `i` has height `rows_height[i]`.
#[derive(Debug, Clone, PartialEq)]
pub struct TableGeometryMatrix {
    /// The width of every column.
    pub columns_width: Vec<f32>,

    /// The height of every row.
    pub rows_height: Vec<f32>,
}

impl TableGeometryMatrix {
    /// Creates a geometry matrix.
    ///
    /// # Panics
    ///
    /// Panics if a column width or row height is negative or not finite.
    pub fn new(columns_width: Vec<f32>, rows_height: Vec<f32>) -> Self {
        assert!(
            columns_width
                .iter()
                .all(|width| width.is_finite() && *width >= 0.0),
            "column widths must be finite and non-negative"
        );
        assert!(
            rows_height
                .iter()
                .all(|height| height.is_finite() && *height >= 0.0),
            "row heights must be finite and non-negative"
        );

        Self {
            columns_width,
            rows_height,
        }
    }

    /// Returns the number of columns.
    pub fn columns_count(&self) -> usize {
        self.columns_width.len()
    }

    /// Returns the number of rows.
    pub fn rows_count(&self) -> usize {
        self.rows_height.len()
    }

    /// Returns the width of a column.
    pub fn column_width(&self, column: usize) -> f32 {
        self.columns_width[column]
    }

    /// Returns the height of a row.
    pub fn row_height(&self, row: usize) -> f32 {
        self.rows_height[row]
    }

    /// Returns the total width of the table.
    pub fn width(&self) -> f32 {
        self.columns_width.iter().sum()
    }

    /// Returns the total height of the table.
    pub fn height(&self) -> f32 {
        self.rows_height.iter().sum()
    }

    /// Returns the horizontal position of the beginning of a column.
    pub fn column_offset(&self, column: usize) -> f32 {
        self.columns_width[..column].iter().sum()
    }

    /// Returns the vertical position of the beginning of a row.
    pub fn row_offset(&self, row: usize) -> f32 {
        self.rows_height[..row].iter().sum()
    }
}