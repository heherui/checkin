/// The logical cell matrix of a table.
///
/// The matrix contains one entry for every logical row and column position.
/// Cells occupied by a merged cell are represented by merge markers instead
/// of duplicating the original cell's content.
///
/// The matrix is intentionally independent from rendering and viewport
/// representation.
#[derive(Debug, Clone, PartialEq)]
pub struct TableCellMatrix
{
    /// The rows of the table.
    pub rows: Vec<Vec<TableCell>>,
}

impl TableCellMatrix
{
    /// Creates a cell matrix.
    ///
    /// # Panics
    ///
    /// Panics if the rows do not all have the same number of columns.
    pub fn new(rows: Vec<Vec<TableCell>>) -> Self
    {
        let columns_count = rows.first().map_or(0, Vec::len);

        assert!(
            rows.iter().all(|row| row.len() == columns_count),
            "all rows in a table cell matrix must have the same column count"
        );

        Self { rows }
    }

    /// Creates an empty cell matrix.
    pub fn empty() -> Self
    {
        Self { rows: Vec::new() }
    }

    /// Returns the number of rows.
    pub fn rows_count(&self) -> usize
    {
        self.rows.len()
    }

    /// Returns the number of columns.
    pub fn columns_count(&self) -> usize
    {
        self.rows.first().map_or(0, Vec::len)
    }

    /// Returns whether the matrix contains no cells.
    pub fn is_empty(&self) -> bool
    {
        self.rows_count() == 0 || self.columns_count() == 0
    }

    /// Returns a cell at the given row and column.
    pub fn get(&self, row: usize, column: usize) -> Option<&TableCell>
    {
        self.rows.get(row).and_then(|row| row.get(column))
    }

    /// Returns a mutable reference to a cell at the given row and column.
    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut TableCell>
    {
        self.rows.get_mut(row).and_then(|row| row.get_mut(column))
    }

    /// Returns a row of cells.
    pub fn row(&self, row: usize) -> Option<&[TableCell]>
    {
        self.rows.get(row).map(Vec::as_slice)
    }
}

/// A logical table cell.
///
/// A normal cell owns its content. A merged cell does not own duplicated
/// content and instead identifies the normal cell from which its content is
/// inherited.
#[derive(Debug, Clone, PartialEq)]
pub enum TableCell
{
    Normal(NormalTableCell),
    MergedToUpwards(MergedTableCell),
    MergedToLeft(MergedTableCell),
}

/// A reference to the normal cell that owns a merged cell's content.
///
/// The position is expressed in the logical cell matrix rather than in
/// viewport or render coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MergedTableCell
{
    /// The row of the normal cell owning the merged content.
    pub origin_row: usize,

    /// The column of the normal cell owning the merged content.
    pub origin_column: usize,
}

/// A normal table cell.
///
/// A normal cell is the owner of its own content and can also serve as the
/// origin of a merged region.
#[derive(Debug, Clone, PartialEq)]
pub struct NormalTableCell
{
    /// The textual content displayed by the cell.
    pub label: String,
}

impl NormalTableCell
{
    /// Creates a normal table cell.
    pub fn new(label: impl Into<String>) -> Self
    {
        Self {
            label: label.into(),
        }
    }
}
