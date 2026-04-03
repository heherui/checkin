use gtk4::prelude::*;
use gtk4::{DrawingArea, GestureClick};

/// Split corner button for adding rows/columns.
///
/// Left side (`+R`) triggers row add; right side (`+C`) triggers column add.
pub struct CornerAddButton {
    widget: DrawingArea,
}

impl CornerAddButton {
    pub fn new() -> Self {
        let widget = DrawingArea::new();
        widget.set_size_request(44, 44);
        widget.set_hexpand(false);
        widget.set_vexpand(false);
        widget.set_draw_func(|_, cr, width, height| {
            let w = f64::from(width.max(1));
            let h = f64::from(height.max(1));
            // Border
            cr.rectangle(0.5, 0.5, w - 1.0, h - 1.0);
            cr.set_source_rgb(0.45, 0.55, 0.65);
            cr.stroke().ok();

            // Main diagonal separator (top-left -> bottom-right)
            cr.move_to(0.0, 0.0);
            cr.line_to(w, h);
            cr.set_source_rgb(0.20, 0.27, 0.34);
            cr.stroke().ok();

            // Labels
            cr.select_font_face(
                "Sans",
                gtk4::cairo::FontSlant::Normal,
                gtk4::cairo::FontWeight::Bold,
            );
            cr.set_font_size((w.min(h) * 0.26).max(9.0));
            cr.set_source_rgb(0.10, 0.16, 0.22);
            // +C near top/right, adjacent to column controls.
            cr.move_to(w * 0.60, h * 0.34);
            cr.show_text("+C").ok();
            // +R near left/bottom, adjacent to row controls.
            cr.move_to(w * 0.14, h * 0.86);
            cr.show_text("+R").ok();
        });

        Self { widget }
    }

    pub fn widget(&self) -> &DrawingArea {
        &self.widget
    }

    pub fn connect_split<F, G>(&self, on_add_row: F, on_add_column: G)
    where
        F: Fn() + 'static,
        G: Fn() + 'static,
    {
        let click = GestureClick::new();
        let widget = self.widget.clone();
        click.connect_pressed(move |_, _, x, y| {
            let width = f64::from(widget.allocated_width()).max(1.0);
            let height = f64::from(widget.allocated_height()).max(1.0);
            // Diagonal split: TL->BR. Above line => +C, below line => +R.
            if y <= (height / width) * x {
                on_add_column();
            } else {
                on_add_row();
            }
        });
        self.widget.add_controller(click);
    }
}
