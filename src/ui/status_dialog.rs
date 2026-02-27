use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Label, Orientation, Window};

use crate::core::AttendanceStatus;

/// Standalone window for selecting an attendance status.
pub struct StatusDialog;

impl StatusDialog {
    pub fn present<F>(cell: &GtkBox, surface: &Label, on_status_selected: F)
    where
        F: Fn(AttendanceStatus, Label) + 'static,
    {
        let on_status_selected: Rc<dyn Fn(AttendanceStatus, Label)> = Rc::new(on_status_selected);
        let window = Self::build(cell);
        let weak_surface = surface.downgrade();

        let content = GtkBox::new(Orientation::Vertical, 10);
        content.set_margin_top(14);
        content.set_margin_bottom(14);
        content.set_margin_start(14);
        content.set_margin_end(14);

        let title = Label::new(Some("请选择签到结果"));
        title.set_xalign(0.0);
        content.append(&title);

        let actions = GtkBox::new(Orientation::Horizontal, 8);
        for status in AttendanceStatus::ALL {
            let button = Button::with_label(status.label());
            let window_clone = window.clone();
            let weak_surface = weak_surface.clone();
            let on_status_selected = Rc::clone(&on_status_selected);
            button.connect_clicked(move |_| {
                if let Some(surface) = weak_surface.upgrade() {
                    on_status_selected(status, surface);
                }
                window_clone.close();
            });
            actions.append(&button);
        }
        content.append(&actions);

        let cancel_button = Button::with_label("取消");
        {
            let window = window.clone();
            cancel_button.connect_clicked(move |_| {
                window.close();
            });
        }
        content.append(&cancel_button);

        window.set_child(Some(&content));
        window.present();
    }

    fn build(cell: &GtkBox) -> Window {
        let window = Window::builder()
            .modal(true)
            .title("签到")
            .default_width(360)
            .default_height(120)
            .build();

        if let Some(parent) = cell.root().and_then(|root| root.downcast::<Window>().ok()) {
            window.set_transient_for(Some(&parent));
        }

        window
    }
}
