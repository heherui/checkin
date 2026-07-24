pub mod app_state;
pub mod message;
pub mod update;

pub use app_state::AppState;
pub use message::Message;

use iced::{Font, Size};

use crate::{
    storage::STORAGE, ui::{app::app_state::{AppDialog, ImportDialogState, ImportFormat::Xlsx}, view::{component::statistics::StatisticsViewModel, screen::main_screen::main_screen}, widget::checkboard::TableViewModel},
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
                dialog: AppDialog::None,
                table_view_model,
                statistics_view_model: Some(StatisticsViewModel {
                    remain_time: format!("--:--:--"),
                    percentage: 32.0,
                    checked: 5,
                    unchecked: 20,
                    leave: 1,
                }),
                import_dialog_state: ImportDialogState { format: Xlsx },
            }
        },
        update::update,
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
