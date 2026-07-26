use crate::ui::widget::table::table_state::{table_cellsref::TableCellsRef, table_layout::TableLayout};

/// the layout data for the table.
mod table_layout;
/// the cells'data referencer for the table.
mod table_cellsref;

#[derive(Debug)]
pub struct TableState
{
    pub table_layout: Option<TableLayout>,
    pub table_cellsref: Option<TableCellsRef>,
}