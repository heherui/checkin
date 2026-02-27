use std::cell::RefCell;

use gtk4::{gdk, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};

use crate::core::AttendanceStatus;

thread_local! {
    static GLOBAL_STYLE_PROVIDER: RefCell<Option<CssProvider>> = const { RefCell::new(None) };
}

/// Installs application-level CSS once per UI thread and keeps it alive.
pub fn ensure_ui_styles() {
    GLOBAL_STYLE_PROVIDER.with(|slot| {
        if slot.borrow().is_some() {
            return;
        }

        let provider = CssProvider::new();
        provider.load_from_data(&build_ui_css());

        if let Some(display) = gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        *slot.borrow_mut() = Some(provider);
    });
}

fn build_ui_css() -> String {
    let mut css = String::from(
        "
        window {
            background-color: rgb(208, 208, 208);
        }
        .app-root {
            background-color: transparent;
            border-radius: 24px;
        }
        .statistics-panel {
            margin-bottom: 8px;
            padding: 10px 12px;
            border-radius: 12px;
            border: 1px solid rgba(148, 163, 184, 0.4);
            background-color: rgba(248, 250, 252, 0.8);
        }
        .statistics-title {
            color: #64748b;
            font-size: 11px;
            font-weight: 760;
        }
        .statistics-summary {
            color: #0f172a;
            font-size: 15px;
            font-weight: 760;
        }
        .statistics-detail {
            color: #334155;
            font-size: 12px;
        }
        .board-shell {
            padding: 0px;
        }
        .table-board {
            background-color: transparent;
        }
        .table-grid {
            margin: 0;
        }
        .table-cell {
            margin: 0;
            padding: 0;
        }
        .cell-surface {
            color: #0f172a;
            border: 2px solid transparent;
            padding: 8px 10px;
            font-size: 13px;
            font-weight: 620;
            transition: border-color 180ms ease, box-shadow 180ms ease, transform 180ms ease;
        }
        .cell-surface.selected {
            border-color: #0ea5e9;
            box-shadow: 0 0 0 2px rgba(14, 165, 233, 0.25);
            transform: translateY(-1px);
        }
        .cell-surface.edit-pending {
            background-color: rgb(226, 232, 240);
            border-color: rgb(148, 163, 184);
            color: rgb(51, 65, 85);
            box-shadow: none;
        }
        ",
    );

    for status in AttendanceStatus::ALL {
        let (r, g, b) = status.background_rgb();
        css.push_str(&format!(
            "
            .cell-surface.{} {{
                background-color: rgba({}, {}, {}, {});
                border-color: rgba({}, {}, {}, 0.55);
                color: {};
            }}
            ",
            status.css_class(),
            r,
            g,
            b,
            status.background_alpha(),
            r,
            g,
            b,
            status.foreground_color(),
        ));
    }

    css.push_str(
        "
        .cell-surface.blocked {
            background-color: rgb(71, 85, 105);
            border-color: rgb(51, 65, 75);
            color: rgb(203, 213, 225);
        }
        .table-cell.transparent,
        .table-cell .cell-surface.transparent {
            background-color: transparent;
            border-color: transparent;
            color: transparent;
            box-shadow: none;
            padding: 0;
        }
        ",
    );

    css
}
