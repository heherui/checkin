use gtk4::gdk;
use gtk4::prelude::*;

pub fn write_text_to_clipboard(text: &str) -> Result<(), &'static str> {
    let Some(display) = gdk::Display::default() else {
        return Err("unable access to system display");
    };

    let clipboard = display.clipboard();
    clipboard.set_text(text);
    Ok(())
}
