use crate::ui::{view::component::statistics::StatisticsViewModel, widget::checkboard::TableViewModel};

#[derive(Debug)]
pub struct AppState
{
    pub show_checkin_dialog: bool,
    pub table_view_model: Option<TableViewModel>,
    pub statistics_view_model: Option<StatisticsViewModel>
}
