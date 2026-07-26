use iced::{
    alignment::Horizontal::Left,
    font,
    widget::{column, container, row, text},
    Background, Border, Color, Element,
    Length::Fill,
};

use crate::ui::app::Message;

pub fn statistics<'a>(vm: Option<&'a StatisticsViewModel>) -> Element<'a, Message>
{
    let vm = vm.unwrap_or(&ZERO_STATISTICS_VIEW_MODEL);
    let medium_weight_font = font::Font {
        weight: font::Weight::Medium,
        ..font::Font::DEFAULT
    };

    container(
        column![
            text(&vm.remain_time).size(20),
            row![
                row![text("已签到:"), text(vm.checked).font(medium_weight_font)].spacing(1),
                text("·"),
                row![text("未签到:"), text(vm.unchecked).font(medium_weight_font)].spacing(1),
                text("·"),
                row![text("请假:"), text(vm.leave).font(medium_weight_font)].spacing(1),
            ]
            .spacing(8),
        ]
        .spacing(8)
        .align_x(Left),
    )
    .width(Fill)
    .padding(20)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb8(245, 248, 252))),

        border: Border {
            radius: 16.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },

        ..Default::default()
    })
    .into()
}

#[derive(Debug, Clone)]
pub struct StatisticsViewModel
{
    pub remain_time: String,
    pub percentage: f32,
    pub checked: u32,
    pub unchecked: u32,
    pub leave: u32,
}

use std::sync::LazyLock;

pub static ZERO_STATISTICS_VIEW_MODEL: LazyLock<StatisticsViewModel> =
    LazyLock::new(|| StatisticsViewModel {
        remain_time: "--:--:--".to_owned(),
        percentage: 0.0,
        checked: 0,
        unchecked: 0,
        leave: 0,
    });
