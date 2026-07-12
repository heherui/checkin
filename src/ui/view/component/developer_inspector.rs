use iced::{Element, Length::Fill, widget::{button, container, row, svg}};

use crate::ui::message::Message;

pub fn developer_inspector<'a>()-> Element<'a,Message>
{
    let show_dialog_icon = svg("./resources/dialog.svg");
    let save_to_config_icon = svg("./resources/save.svg");
    let load_from_config_icon = svg("./resources/load.svg");
    let import_from_excel_icon = svg("./resources/import.svg");
    let copy_icon = svg("./resources/copy.svg");
    let send_to_qq_icon = svg("./resources/qq.svg");

    let buttons = row![
        button(show_dialog_icon).on_press(Message::ShowCheckinDialog),
        button(save_to_config_icon),
        button(load_from_config_icon),
        button(import_from_excel_icon).on_press(Message::ImportTableDataFromExcel),
        button(copy_icon),
        button(send_to_qq_icon),
    ]
    .spacing(3)
    .padding(1)
    .width(Fill)
    .height(Fill);

    container(buttons)
        .width(Fill)
        .height(Fill)
        .into()
}