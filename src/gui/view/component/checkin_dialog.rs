use iced::{Element, widget::{button, column, text}};

use crate::gui::message::Message;

pub fn checkin_dialog<'a>()-> Element<'a, Message>
{
    column![
        text("name"),
        button("confirm"),
        button("cancel"),
    ]
    .into()
}