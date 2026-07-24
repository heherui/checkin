use iced::{
    Alignment, Background, Border, Color, Element, Length::Fill, widget::{container, row},
};

use crate::ui::{app::Message, view::component::plain_button::plain_button};

pub fn developer_inspector<'a>() -> Element<'a, Message>
{
    let buttons = row![
        plain_button("dev:show_dialog").on_press(Message::ShowCheckinDialog),
        plain_button("Save Config"),
        plain_button("Load Config"),
        plain_button("Import .xlsx").on_press(Message::ImportTableDataFromExcel),
        plain_button("Copy Statistics"),
        plain_button("Send to QQ"),
    ]
    .align_y(Alignment::Center)
    .spacing(10)
    .padding(8);

    container(buttons)
        .padding(5)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgb8(245, 247, 250))),

            border: Border {
                width: 1.0,
                radius: 12.0.into(),
                color: Color::from_rgba8(0, 0, 0, 0.08),
            },

            ..Default::default()
        })
        .width(Fill)
        .into()
}
