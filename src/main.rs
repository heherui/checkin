mod gui;
mod core;

mod utilities;

fn main()-> iced::Result
{
    return gui::App::new().run();
}