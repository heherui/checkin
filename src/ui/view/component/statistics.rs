use iced::{
    widget::{column, row, text},
    Element,
};

use crate::ui::message::Message;

pub fn statistics<'a>(vm: Option<&'a StatisticsViewModel>) -> Element<'a, Message>
{
    let vm = vm.unwrap_or(&ZERO_STATISTICS_VIEW_MODEL);

    column![
        text(&vm.remain_time).size(42),
        text(format!("{:.1}% 已完成签到", vm.percentage)).size(20),
        row![
            text(format!("已签到 {}", vm.checked)),
            text(format!("未签到 {}", vm.unchecked)),
            text(format!("请假 {}", vm.leave)),
        ]
        .spacing(20),
    ]
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
