mod cell_matrix;
mod geometry_matrix;
mod table;

pub use cell_matrix::{
    MergedTableCell,
    NormalTableCell,
    TableCell,
    TableCellMatrix,
};

pub use geometry_matrix::TableGeometryMatrix;
pub use table::TableViewModel;