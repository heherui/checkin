use iced::{
    widget::{button, column, row, text},
    Element,
};

use crate::ui::{app::{Message, app_state::AppDialog::Checkin}, view::component::signture_pad::signture_pad};

pub fn checkin_dialog<'a>() -> Element<'a, Message>
{
    column![
        text("Name"),
        signture_pad(),
        row![
            button("confirm"),
            button("cancel").on_press(Message::CloseDialog(Checkin))
        ].spacing(8),
    ]
    .spacing(5)
    .padding(10)
    .into()
}
