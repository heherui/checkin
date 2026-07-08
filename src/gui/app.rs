use iced::Size;

use crate::gui::view::screen::main_screen::main_screen;

#[allow(unused)]
pub static APP_ID:&'static str = "io.github.andeibuite.checkin";

pub struct App
{
    
}

impl App 
{
    pub fn new()-> Self
    {
        Self {  }
    }
}

impl App 
{
    pub fn run(&self)-> iced::Result
    {
        let inner_app = iced::application(
            AppState::default,
            crate::gui::update::update,
            main_screen
        )   
        .title("Checkin")
        //.theme(|_| {MinimalTheme})
        .window_size(Size::new(900.0, 600.0));

        inner_app.run()
    }
}

#[derive(Debug)]
pub struct AppState
{
    pub show_checkin_dialog:bool
}

impl AppState 
{
    pub fn default()-> Self
    {
        Self { 
            show_checkin_dialog: false
        }
    }
}