use iced::{
    widget::{button, column, container, row, text},
    Element,
    Length::Fill,
};

use crate::ui::app::{
    app_state::{AppDialog::Import, ImportDialogState, ImportFormat},
    Message,
};

pub fn import_dialog<'a>(state: &'a ImportDialogState) -> Element<'a, Message>
{
    let format_switch = row![
        container(
            button(".xlsx")
                .width(Fill)
                .on_press(Message::SelectImportFormat(ImportFormat::Xlsx))
        )
        .width(Fill),
        container(
            button(".txt")
                .width(Fill)
                .on_press(Message::SelectImportFormat(ImportFormat::Txt))
        )
        .width(Fill),
    ]
    .width(Fill)
    .spacing(1);

    let import_options = match state.format {
        ImportFormat::Xlsx => xlsx_options(),
        ImportFormat::Txt => txt_options(),
    };

    let ctrl_buttons = row![
        button("confirm"),
        button("cancel").on_press(Message::CloseDialog(Import)),
    ]
    .spacing(5);

    column![
        container(format_switch).height(30),
        container(import_options).height(Fill),
        container(ctrl_buttons).height(50),
    ]
    .width(500)
    .spacing(2)
    .into()
}

fn xlsx_options<'a>() -> Element<'a, Message>
{
    text("xlsx_options").center().into()
}

fn txt_options<'a>() -> Element<'a, Message>
{
    text("txt_options").center().into()
}
