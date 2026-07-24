use crate::ui::{view::component::statistics::StatisticsViewModel, widget::checkboard::TableViewModel};

#[derive(Debug)]
pub struct AppState
{
    pub dialog: AppDialog,
    pub table_view_model: Option<TableViewModel>,
    pub import_dialog_state: ImportDialogState,
    pub statistics_view_model: Option<StatisticsViewModel>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AppDialog 
{
    None,
    Checkin,
    Import,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportFormat 
{
    Xlsx,
    Txt,
}

#[derive(Debug)]
pub struct ImportDialogState 
{
    pub format: ImportFormat,
}