use iced::{Element, Pixels, advanced::text};
use crate::ui::{app::{APP_FONT, Message}, widget::checkboard::{Table, TableStyle, TableViewModel}};

pub fn checkboard<'a>(table_view_model: Option<&'a TableViewModel>) -> Element<'a, Message>
{
    let table_style = TableStyle { 
            font: APP_FONT, 
            text_size: Pixels(16.0),
            shapping: text::Shaping::Advanced,
        };
    let table = Table::new(table_view_model, table_style);

    table.into()
}