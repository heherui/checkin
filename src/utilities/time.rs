use chrono::{DateTime, Local, Timelike};
use std::time::SystemTime;

pub trait SystemTimeExt {
    fn period_string(&self) -> &'static str;
    fn formatted_string(&self) -> String;
}

impl SystemTimeExt for SystemTime {
    fn period_string(&self) -> &'static str {
        let datetime: DateTime<Local> = (*self).into();
        let total_minutes = datetime.hour() * 60 + datetime.minute();

        const NOON_START: u32 = 11 * 60; // 11:00
        const NOON_END: u32 = 15 * 60 + 30; // 15:30

        if total_minutes >= NOON_START && total_minutes <= NOON_END {
            "中午"
        } else if total_minutes < NOON_START {
            "上午"
        } else {
            "下午"
        }
    }

    fn formatted_string(&self) -> String {
        let datetime: DateTime<Local> = (*self).into();
        datetime.format("%m.%d.%Y %H:%M:%S").to_string()
    }
}
