use iced::{Font, Size};

use crate::{
    storage::STORAGE,
    ui::{
        app_state::AppState,
        view::{component::checkboard::TableViewModel, screen::main_screen::main_screen},
    },
};

#[allow(unused)]
pub static APP_ID: &str = "io.github.andeibuite.checkin";

#[cfg(target_os = "macos")]
pub const APP_FONT: Font = Font::with_name("PingFang SC");

#[cfg(target_os = "windows")]
pub const APP_FONT: Font = Font::with_name("Microsoft YaHei");

#[cfg(target_os = "linux")]
pub const APP_FONT: Font = Font::with_name("Noto Sans CJK SC");

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
                statistics_view_model: None,
            }
        },
        crate::ui::update::update,
        main_screen,
    )
    .default_font(APP_FONT)
    .title("Checkin")
    .window(iced::window::Settings {
        size: Size::new(16.0 * 70.0, 10.0 * 70.0),
        position: iced::window::Position::Centered,
        min_size: Some(Size::new(16.0 * 50.0, 10.0 * 50.0)),
        ..Default::default()
    })
    .run()
}
