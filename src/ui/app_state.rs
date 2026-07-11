use crate::ui::view::component::checkboard::TableViewModel;

#[derive(Debug)]
pub struct AppState
{
    pub show_checkin_dialog: bool,
    pub table_view_model: Option<TableViewModel>,
}
