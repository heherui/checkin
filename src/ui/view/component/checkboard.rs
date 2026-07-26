use chrono::Local;
use iced::{Element, Pixels, advanced::text};
use crate::ui::{app::{APP_FONT, Message}, widget::checkboard::{Table, TableStyle, TableViewModel}};

pub fn checkboard<'a>(table_view_model: Option<&'a TableViewModel>) -> Element<'a, Message>
{
    let table_style = TableStyle { 
            font: APP_FONT, 
            text_size: Pixels(16.0),
            shapping: text::Shaping::Advanced,
            spacing: 1.0,
        };
    let table = Table::new(table_view_model, table_style)
    .on_press(|cell_id|{Message::ConfirmCheckin { id: cell_id, time: Local::now().naive_local() }});

    table.into()
}