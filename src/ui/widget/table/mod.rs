/// data to initialize a table.
mod table_view_model;
/// table iced widget api.
mod table_widget;
/// comtomizable widget style api.
mod table_style;

mod table_state;

// TODO: 局部更新 Table
// TODO: Table drawing 压测

pub use table_widget::Table;
pub use table_view_model::TableViewModel;
pub use table_style::TableStyle;