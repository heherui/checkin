use iced::{Element, Length::Fill, widget::container};

use crate::gui::message::Message;

pub fn checkboard<'a>()-> Element<'a, Message>
{
    container("checkboard")
        .center(Fill)
        .into()
}