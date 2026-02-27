use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::Button;

use crate::core::AppMode;

/// Single-button mode switch component.
///
/// Click toggles between check-in and edit mode.
pub struct ModeSwitch {
    button: Button,
    mode: Rc<RefCell<AppMode>>,
}

impl ModeSwitch {
    pub fn new(initial_mode: AppMode) -> Self {
        let button = Button::new();

        let mode = Rc::new(RefCell::new(initial_mode));
        Self::apply_visual_state(&button, initial_mode);

        Self { button, mode }
    }

    pub fn widget(&self) -> &Button {
        &self.button
    }

    pub fn connect_toggled<F>(&self, on_mode_changed: F)
    where
        F: Fn(AppMode) + 'static,
    {
        let mode = Rc::clone(&self.mode);
        self.button.connect_clicked(move |btn| {
            let next_mode = match *mode.borrow() {
                AppMode::CheckIn => AppMode::Edit,
                AppMode::Edit => AppMode::CheckIn,
            };

            *mode.borrow_mut() = next_mode;
            Self::apply_visual_state(btn, next_mode);
            on_mode_changed(next_mode);
        });
    }

    fn apply_visual_state(button: &Button, mode: AppMode) {
        match mode {
            AppMode::CheckIn => {
                button.set_label("edit");
            }
            AppMode::Edit => {
                button.set_label("done");
            }
        }
    }
}
