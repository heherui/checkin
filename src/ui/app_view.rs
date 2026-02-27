use std::rc::Rc;
use std::time::SystemTime;

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Label, Orientation};

use crate::core::{AppMode, Configuration, Table};
use crate::ui::mode_switch::ModeSwitch;
use crate::ui::statistics_panel::StatisticsPanel;
use crate::ui::table_view::TableView;
use crate::ui::ui_styles::ensure_ui_styles;
use crate::utilities::write_text_to_clipboard;

/// Top-level app content that composes all UI components.
pub struct AppView {
    root: GtkBox,
    _configuration: Configuration,
    _mode_switch: ModeSwitch,
    _table_view: Rc<TableView>,
    _statistics_panel: StatisticsPanel,
    pub stats_label: Label,
}

impl AppView {
    /// Creates the top-level app view.
    pub fn new(table: &Table, configuration: Configuration) -> Self {
        ensure_ui_styles();

        let root = GtkBox::new(Orientation::Vertical, 8);
        root.add_css_class("app-root");
        root.set_margin_top(20);
        root.set_margin_bottom(20);
        root.set_margin_start(20);
        root.set_margin_end(20);

        let mode_switcher = GtkBox::new(Orientation::Horizontal, 12);
        mode_switcher.add_css_class("toolbar");
        let mode_switch = ModeSwitch::new(AppMode::CheckIn);
        let copy_statistics_button = Button::with_label("copy statistics");
        copy_statistics_button.set_visible(true);
        mode_switcher.append(mode_switch.widget());
        mode_switcher.append(&copy_statistics_button);

        let board_shell = GtkBox::new(Orientation::Vertical, 0);
        board_shell.add_css_class("board-shell");
        board_shell.set_vexpand(true);

        let table_view = Rc::new(TableView::new(table));
        let statistics_panel = StatisticsPanel::new(table_view.get_statistics());
        let stats_label = statistics_panel.summary_label();

        {
            let panel_for_updates = statistics_panel.clone();
            table_view.connect_status_changed(move |statistics| {
                panel_for_updates.update(statistics);
            });
        }
        {
            let config_file = configuration.config_file.clone();
            table_view.connect_table_exported(move |table| {
                if let Err(error) = table.write_config(&config_file) {
                    eprintln!(
                        "failed to write table config to {}: {error}",
                        config_file.display()
                    );
                }
            });
        }
        {
            let table_view = Rc::clone(&table_view);
            let copy_statistics_button = copy_statistics_button.clone();
            mode_switch.connect_toggled(move |mode| {
                table_view.set_mode(mode);
                copy_statistics_button.set_visible(mode == AppMode::CheckIn);
            });
        }
        {
            let table_view = Rc::clone(&table_view);
            copy_statistics_button.connect_clicked(move |_| {
                let time: SystemTime = SystemTime::now();
                let text = table_view.build_statistics_export_text_zh(&time);
                if let Err(error) = write_text_to_clipboard(&text) {
                    eprintln!("copy statistics failed: {error}");
                }
            });
        }

        board_shell.append(table_view.widget());
        root.append(statistics_panel.widget());
        root.append(&mode_switcher);
        root.append(&board_shell);

        Self {
            root,
            _configuration: configuration,
            _mode_switch: mode_switch,
            _table_view: table_view,
            _statistics_panel: statistics_panel,
            stats_label,
        }
    }

    /// Returns the top-level root widget.
    pub fn widget(&self) -> &GtkBox {
        &self.root
    }
}
