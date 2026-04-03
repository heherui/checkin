use std::env;
use std::path::{Path, PathBuf};

use checkin::{AppView, Configuration, Position, Subject, Table, APPLICATION_ID};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

fn main() 
{
    let configuration = parse_configuration(env::args().skip(1).collect());
    let app = Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Checkin")
            .default_width(950)
            .default_height(620)
            .build();

        let table = if configuration.table_configuration_file.exists() {
            Table::load_config(&configuration.table_configuration_file).unwrap_or_else(|error| {
                eprintln!(
                    "failed to load config {}: {error}, using default table",
                    configuration.table_configuration_file.display()
                );
                default_table()
            })
        } else {
            default_table()
        };
        let app_view = AppView::new(&table, configuration.clone());
        window.set_child(Some(app_view.widget()));
        window.present();
    });

    app.run();
}

fn parse_configuration(args: Vec<String>) -> Configuration 
{
    let mut table_configuration_file = env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf))
        .or_else(|| env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
        .join("table.conf.json");
    let mut index = 0usize;
    while index < args.len() {
        let arg = &args[index];
        if let Some(path) = arg.strip_prefix("--config=") {
            table_configuration_file = PathBuf::from(path);
            index += 1;
            continue;
        }
        if arg == "--config" {
            if let Some(path) = args.get(index + 1) {
                table_configuration_file = PathBuf::from(path);
                index += 2;
                continue;
            }
            eprintln!("--config requires a file path, falling back to default");
        }
        index += 1;
    }
    Configuration::new(table_configuration_file)
}

pub fn default_table() -> Table 
{
    const ROW_COUNT: u32 = 8;
    const COLUMN_COUNT: u32 = 10;
    const STUDENT_COUNT: u32 = 51;
    const SEAT_COLUMNS: [u32; 8] = [0, 1, 3, 4, 5, 6, 8, 9];

    let mut subjects = Vec::new();

    // Top row with center lectern and no student seats.
    for x in 0..COLUMN_COUNT 
    {
        let subject = if (x == 4)|(x == 5) {
            Subject::Block(String::from("讲台"))
        } else {
            Subject::Transparent
        };
        subjects.push((Position { x, y: 0 }, subject));
    }

    // Two vertical aisles that split left/middle/right as 2/4/2 seat columns.
    for y in 1..ROW_COUNT 
    {
        subjects.push((Position { x: 2, y }, Subject::Transparent));
        subjects.push((Position { x: 7, y }, Subject::Transparent));
    }

    let mut student_index = 0u32;
    for y in 1..ROW_COUNT {
        for &x in &SEAT_COLUMNS {
            if student_index < STUDENT_COUNT {
                subjects.push((
                    Position { x, y },
                    Subject::Some(format!("Student {:02}", student_index)),
                ));
                student_index += 1;
            } else {
                subjects.push((Position { x, y }, Subject::Transparent));
            }
        }
    }

    Table::new(ROW_COUNT, COLUMN_COUNT, subjects)
}
