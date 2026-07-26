use crate::ui::{app::Message, widget::signturepad::SignturePad};
use iced::{Element, Length::Fill, widget::{container, stack, text}};

pub fn signture_pad<'a>() -> Element<'a, Message>
{
    let label = container(text("signture_pad")).center(Fill);
    let signture_pad = SignturePad {};

    container(stack![signture_pad, label])
        .center(Fill)
        .into()
}