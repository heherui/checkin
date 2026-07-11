pub mod storage;
pub mod table_layout;
mod ui;

mod utilities;

fn main()-> iced::Result
{
    ui::run()
}