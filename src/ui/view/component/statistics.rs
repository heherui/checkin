use iced::{Element, Length::Fill, widget::{container, text}};

use crate::ui::message::Message;

pub fn statistics<'a>()-> Element<'a, Message>
{
    container(text("statistics"))
        .width(Fill)
        .into()
}