use super::{TableCellMatrix, TableGeometryMatrix};

/// The complete view model of a table.
///
/// `TableViewModel` describes the logical structure, content, and geometry
/// of a table. It is independent from viewport state, caching, and rendering.
///
/// A table consists of two parallel matrices:
///
/// - [`TableGeometryMatrix`] describes the size of columns and rows.
/// - [`TableCellMatrix`] describes the logical cells and their contents.
///
/// The two matrices must always describe the same table dimensions.
#[derive(Debug, Clone, PartialEq)]
pub struct TableViewModel {
    /// The geometry of the table.
    pub geometry_matrix: TableGeometryMatrix,

    /// The logical cells of the table.
    pub cell_matrix: TableCellMatrix,
}

impl TableViewModel {
    /// Creates a table view model.
    ///
    /// # Panics
    ///
    /// Panics if the geometry matrix and cell matrix have different
    /// dimensions.
    pub fn new(
        geometry_matrix: TableGeometryMatrix,
        cell_matrix: TableCellMatrix,
    ) -> Self {
        assert_eq!(
            geometry_matrix.columns_count(),
            cell_matrix.columns_count(),
            "table geometry and cell matrix have different column counts"
        );
        assert_eq!(
            geometry_matrix.rows_count(),
            cell_matrix.rows_count(),
            "table geometry and cell matrix have different row counts"
        );

        Self {
            geometry_matrix,
            cell_matrix,
        }
    }

    /// Returns the number of columns in the table.
    pub fn columns_count(&self) -> usize {
        self.geometry_matrix.columns_count()
    }

    /// Returns the number of rows in the table.
    pub fn rows_count(&self) -> usize {
        self.geometry_matrix.rows_count()
    }

    /// Returns whether the table contains no cells.
    pub fn is_empty(&self) -> bool {
        self.rows_count() == 0 || self.columns_count() == 0
    }

    /// Returns the total logical width of the table.
    pub fn width(&self) -> f32 {
        self.geometry_matrix.width()
    }

    /// Returns the total logical height of the table.
    pub fn height(&self) -> f32 {
        self.geometry_matrix.height()
    }
}