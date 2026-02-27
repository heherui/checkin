use std::env;
use std::path::PathBuf;

use checkin::{AppView, Configuration, Table};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

fn main() {
    let configuration = parse_configuration(env::args().skip(1).collect());
    let app = Application::builder()
        .application_id("io.github.andeibuite.checkin")
        .build();

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Checkin")
            .default_width(950)
            .default_height(620)
            .build();

        let table = if configuration.config_file.exists() {
            Table::load_config(&configuration.config_file).unwrap_or_else(|error| {
                eprintln!(
                    "failed to load config {}: {error}, using default table",
                    configuration.config_file.display()
                );
                Table::default_table()
            })
        } else {
            Table::default_table()
        };
        let app_view = AppView::new(&table, configuration.clone());
        window.set_child(Some(app_view.widget()));
        window.present();
    });

    app.run();
}

fn parse_configuration(args: Vec<String>) -> Configuration {
    let mut config_file = Configuration::default_config_file();
    let mut index = 0usize;
    while index < args.len() {
        let arg = &args[index];
        if let Some(path) = arg.strip_prefix("--config=") {
            config_file = PathBuf::from(path);
            index += 1;
            continue;
        }
        if arg == "--config" {
            if let Some(path) = args.get(index + 1) {
                config_file = PathBuf::from(path);
                index += 2;
                continue;
            }
            eprintln!("--config requires a file path, falling back to default");
        }
        index += 1;
    }
    Configuration::new(config_file)
}
