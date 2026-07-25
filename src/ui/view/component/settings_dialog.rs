use crate::ui::app::{app_state::AppDialog::Settings, Message};
use iced::{
    Element, Length::Fill, widget::{button, column, container, text},
};

pub fn settings_dialog<'a>() -> Element<'a, Message>
{
    let text = container(text("settings")).center(Fill);

    let close_button =
        container(button("close").on_press(Message::CloseDialog(Settings))).width(Fill);

    column![text, close_button].into()
}
