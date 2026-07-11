use iced::Task;

use crate::{
    core::storage,
    ui::{app::AppState, message::Message},
};

pub fn update(app_state: &mut AppState, message: Message) -> Task<Message>
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
        Message::ConfirmCheckin { id, time } => {
            println!("Confirm Checkin: id:{id}, time:{time}");
            Task::perform(storage::save_checkin(), |_| Message::CloseCheckinDialog)
        }
    }
}
