use gtk4::prelude::*;
use gtk4::{Box as GtkBox, ButtonsType, Label, MessageDialog, ResponseType, Window};

use crate::core::AttendanceStatus;

/// Modal dialog used for selecting an attendance status.
///
/// This component only owns dialog presentation and response mapping.
/// Business-state updates should stay in callers via the callback.
pub struct StatusDialog;

impl StatusDialog {
    /// Presents the status dialog and invokes callback when user chooses a status.
    pub fn present<F>(cell: &GtkBox, surface: &Label, on_status_selected: F)
    where
        F: Fn(AttendanceStatus, Label) + 'static,
    {
        let dialog = Self::build(cell);
        let weak_surface = surface.downgrade();

        dialog.connect_response(move |dialog, response| {
            if let Some(status) = Self::map_response_to_status(response) {
                if let Some(surface) = weak_surface.upgrade() {
                    on_status_selected(status, surface);
                }
            }

            dialog.close();
        });

        dialog.present();
    }

    fn build(cell: &GtkBox) -> MessageDialog {
        let dialog_builder = MessageDialog::builder()
            .modal(true)
            .text("请选择签到结果")
            .buttons(ButtonsType::None);

        let dialog =
            if let Some(window) = cell.root().and_then(|root| root.downcast::<Window>().ok()) {
                dialog_builder.transient_for(&window).build()
            } else {
                dialog_builder.build()
            };

        for status in AttendanceStatus::ALL {
            dialog.add_button(status.label(), Self::map_status_to_response(status));
        }
        dialog.add_button("取消", ResponseType::Cancel);

        dialog
    }

    fn map_status_to_response(status: AttendanceStatus) -> ResponseType {
        match status {
            AttendanceStatus::Checked => ResponseType::Accept,
            AttendanceStatus::Unchecked => ResponseType::Reject,
            AttendanceStatus::Marked => ResponseType::Apply,
        }
    }

    fn map_response_to_status(response: ResponseType) -> Option<AttendanceStatus> {
        match response {
            ResponseType::Accept => Some(AttendanceStatus::Checked),
            ResponseType::Reject => Some(AttendanceStatus::Unchecked),
            ResponseType::Apply => Some(AttendanceStatus::Marked),
            _ => None,
        }
    }
}
