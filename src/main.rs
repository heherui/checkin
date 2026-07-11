mod ui;
mod core;

mod utilities;

fn main()-> iced::Result
{
    return ui::App::new().run();
}