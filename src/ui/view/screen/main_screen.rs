use iced::{
    widget::{column, container, stack},
    Element,
    Length::Fill,
};

use crate::ui::{
    app::{AppState, Message, app_state::AppDialog}, view::component::{
        checkboard::checkboard, checkin_dialog::checkin_dialog, developer_inspector::developer_inspector, dialog_overlay::dialog_overlay, import_dialog::import_dialog, statistics::statistics,
    },
};

pub fn main_screen(app_state: &AppState) -> Element<'_, Message>
{
    let statistics = statistics(app_state.statistics_view_model.as_ref());
    let developer_inspector = developer_inspector();
    let checkboard = checkboard(app_state.table_view_model.as_ref());

    let content = container(
        column![
            container(statistics).height(100),
            container(developer_inspector).height(60),
            container(checkboard),
        ]
        .spacing(3),
    )
    .padding(10)
    .width(Fill)
    .height(Fill);

    let overlay = match app_state.dialog {
        AppDialog::None => None,
        AppDialog::Checkin => Some(dialog_overlay(checkin_dialog())),
        AppDialog::Import => Some(dialog_overlay(import_dialog(&app_state.import_dialog_state))),
    };

    match overlay {
        Some(overlay) => stack![content, overlay].into(),
        None => content.into(),
    }
}
