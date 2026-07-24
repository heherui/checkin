use std::println;

use iced::{Task, clipboard};

use crate::{
    storage::STORAGE,
    ui::{
        app::{
            app_state::{
                AppDialog::None,
                AppState,
            },
            message::Message,
        },
        widget::checkboard::TableViewModel,
    },
};

pub fn update(app_state: &mut AppState, message: Message) -> Task<Message>
{
    match message {
        Message::ShowDialog(dialog) => {
            app_state.dialog = dialog;
            Task::none()
        }
        Message::CloseDialog(dialog) => {
            if app_state.dialog == dialog {
                app_state.dialog = None
            }
            Task::none()
        }
        Message::LoadTableData => Task::perform(
            async { STORAGE.load_table_data().map_err(|e| e.to_string()) },
            |result| Message::LoadTableDataResult { result },
        ),
        Message::LoadTableDataResult { result } => {
            match result {
                Ok(table_data) => {
                    app_state.table_view_model = Some(TableViewModel::load_from(table_data));
                }
                Err(error) => {
                    eprintln!("Load table data failed: {error}");
                }
            }
            Task::none()
        }
        Message::SaveTableData { data } => Task::perform(
            async move { STORAGE.save_table_data(&data).map_err(|e| e.to_string()) },
            |result| Message::SaveTableDataResult { result },
        ),
        Message::SaveTableDataResult { result } => {
            match result {
                Ok(_) => {
                    println!("Table data saved");
                }
                Err(error) => {
                    eprintln!("Save table data failed: {error}");
                }
            }
            Task::none()
        }
        Message::ConfirmCheckin { id, time } => {
            println!("Confirm Checkin id:{id}, time:{time}");
            // TODO
            Task::none()
        }
        Message::ImportTableDataFromExcel => {
            // TODO
            Task::none()
        },

        Message::SelectImportFormat(format) => {
            app_state.import_dialog_state.format = format;
            Task::none()
        },

        Message::CopyStatistics => {
            let Some(statistics) = &app_state.statistics_view_model else {
                return Task::none();
            };

            let text = format!(
                "签到统计\n\
                 签到率：{:.1}%\n\
                 已签到：{}\n\
                 未签到：{}\n\
                 请假：{}\n\
                 剩余时间：{}",
                statistics.percentage,
                statistics.checked,
                statistics.unchecked,
                statistics.leave,
                statistics.remain_time,
            );

            clipboard::write(text)
        }
    }
}
