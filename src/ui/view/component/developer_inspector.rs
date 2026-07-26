use iced::{
    widget::{container, row, scrollable},
    Alignment, Background, Border, Color, Element,
    Length::Fill,
};

use crate::ui::{
    app::{
        Message, app_state::AppDialog::{Checkin, Import, Settings},
    }, view::component::plain_button::plain_button,
};

pub fn developer_inspector<'a>() -> Element<'a, Message>
{
    let dev_buttons = container(
        row![
            plain_button("dev:show_dialog").on_press(Message::ShowDialog(Checkin)),
            plain_button("dev:save_config"),
            plain_button("dev:load_config").on_press(Message::LoadTableData),
        ]
        .align_y(Alignment::Center)
        .padding(5),
    )
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb8(255, 248, 225))), // 浅黄
        border: Border {
            width: 1.0,
            radius: 8.0.into(),
            color: Color::from_rgb8(255, 193, 7), // Amber
        },
        ..Default::default()
    });

    let buttons = row![
        plain_button("Settings").on_press(Message::ShowDialog(Settings)),
        plain_button("Import Table").on_press(Message::ShowDialog(Import)),
        plain_button("Copy Statistics").on_press(Message::CopyStatistics),
        plain_button("Send to QQ"),
    ]
    .align_y(Alignment::Center)
    .spacing(5)
    .padding(8);

    container(
        scrollable(row![dev_buttons, buttons]).direction(scrollable::Direction::Horizontal(
            scrollable::Scrollbar::hidden(),
        )),
    )
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
