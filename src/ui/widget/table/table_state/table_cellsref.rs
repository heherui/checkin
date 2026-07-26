use crate::ui::widget::table::{TableViewModel, table_view_model::TableCellViewModel};

#[derive(Debug)]
pub struct TableCellsRef
{
    rows:Vec<Vec<TableCellViewModel>>
}

impl TableCellsRef 
{
    pub fn from_view_model(view_model:&TableViewModel)-> Self
    {
        Self { rows: view_model.rows.clone() }
    }
}

impl TableCellsRef 
{
    pub fn access(&self, x:usize, y:usize)-> &TableCellViewModel
    {
        return &self.rows[y][x];
    }
}