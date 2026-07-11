use iced::Size;

use crate::{storage::STORAGE, ui::{app_state::AppState, view::{component::checkboard::TableViewModel, screen::main_screen::main_screen}}};

#[allow(unused)]
pub static APP_ID: &str = "io.github.andeibuite.checkin";

pub fn run() -> iced::Result
{
    iced::application(
        || {
            let table_view_model = STORAGE
                .load_table_data()
                .ok()
                .map(TableViewModel::load_from);

            AppState {
                show_checkin_dialog: false,
                table_view_model,
            }
        },
        crate::ui::update::update,
        main_screen,
    )
    .title("Checkin")
    .window_size(Size::new(900.0, 600.0))
    .run()
}