use iced::Task;

use crate::{
    storage::STORAGE, ui::{
        app_state::AppState, message::Message, view::component::checkboard::TableViewModel,
    },
};


pub fn update(
    app_state: &mut AppState,
    message: Message,
) -> Task<Message>
{
    match message {
        Message::ShowCheckinDialog => {
            app_state.show_checkin_dialog = true;
            Task::none()
        }
        Message::CloseCheckinDialog => {
            app_state.show_checkin_dialog = false;
            Task::none()
        }
        Message::LoadTableData => {
            Task::perform(
                async {
                    STORAGE
                        .load_table_data()
                        .map_err(|e| e.to_string())
                },
                |result|
                    Message::LoadTableDataResult {
                        result,
                    }
            )
        }
        Message::LoadTableDataResult { result } => {
            match result {
                Ok(table_data) => {
                    app_state.table_view_model = Some(TableViewModel::load_from(table_data));
                }
                Err(error) => {
                    eprintln!(
                        "Load table data failed: {error}"
                    );
                }
            }
            Task::none()
        }
        Message::SaveTableData { data } => {
            Task::perform(
                async move {
                    STORAGE
                        .save_table_data(&data)
                        .map_err(|e| e.to_string())
                },
                |result|
                    Message::SaveTableDataResult {
                        result,
                    }
            )
        }
        Message::SaveTableDataResult { result } => {
            match result {
                Ok(_) => {
                    println!(
                        "Table data saved"
                    );
                }
                Err(error) => {
                    eprintln!(
                        "Save table data failed: {error}"
                    );
                }
            }
            Task::none()
        }
        Message::ConfirmCheckin { id, time } => {
            println!(
                "Confirm Checkin id:{id}, time:{time}"
            );
            // TODO
            Task::none()
        }
        Message::ImportTableDataFromExcel => {
            // TODO
            Task::none()
        },
    }
}