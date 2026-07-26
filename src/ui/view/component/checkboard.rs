use crate::ui::{
    app::{APP_FONT, Message, app_state::AppDialog::Import}, view::component::plain_button::plain_button, widget::table::{Table, TableStyle, TableViewModel},
};
use chrono::Local;
use iced::{
    Element, Length::Fill, Pixels, widget::{container, text::Shaping::Advanced},
};

pub fn checkboard<'a>(table_view_model: Option<&'a TableViewModel>) -> Element<'a, Message>
{
    if let Some(table_view_model) = table_view_model {
        Table::new()
            .view_model(table_view_model)
            .style(TableStyle {
                label_font: APP_FONT,
                label_size: Pixels(16.0),
                shapping: Advanced,
            })
            .on_press(|cell_id| Message::ConfirmCheckin {
                id: cell_id,
                time: Local::now().naive_local(),
            })
            .into()
    } else {
        let import_button = plain_button("Import Table Data").on_press(Message::ShowDialog(Import));
        container(import_button).center(Fill).into()
    }
}
