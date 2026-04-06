use checkin::core::Person;
use checkin::ui::component::{
    Table, TableElement, TableElementMap, TablePersonStatus, TablePersonsStatusMap
};
use checkin::APPLICATION_ID;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use rand::Rng;
use uuid::Uuid;

fn main()
{
    let app = Application::builder()
        .application_id(APPLICATION_ID)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application)
{
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Checkin")
        .default_width(950)
        .default_height(620)
        .build();

    let table = Table::new();
    let table_element_map = default_table_element_map();
    let table_persons_status_map = default_table_persons_status_map(&table_element_map);

    if let Err(e) = table.set_element_map(table_element_map)
    {
        eprintln!("failed to set table element map: \n\t{e}");
    }
    if let Err(e) = table.set_persons_status_map(table_persons_status_map)
    {
        eprintln!("failed to set table status map: \n\t{e}");
    }

    window.set_child(Some(&table));
    window.present();
}

fn default_table_element_map() -> TableElementMap
{
    const ROW_COUNT: usize = 8;
    const COLUMN_COUNT: usize = 10;
    const AISLE_COLUMNS: [usize; 2] = [3, 7];

    let mut student_index: usize = 1;
    let mut rows: Vec<Vec<TableElement>> = Vec::with_capacity(ROW_COUNT);

    for y in 0..ROW_COUNT
    {
        let mut subjects: Vec<TableElement> = Vec::with_capacity(COLUMN_COUNT);
        for x in 0..COLUMN_COUNT
        {
            if AISLE_COLUMNS.contains(&x)
            {
                subjects.push(TableElement::Transparent);
            }
            else if y == 0
            {
                if x == 4 || x == 5 || x == 6
                {
                    subjects.push(TableElement::Block {
                        name: Some(String::from("讲台")),
                    });
                }
                else
                {
                    subjects.push(TableElement::Transparent);
                }
            }
            else
            {
                let person = Person {
                    name: format!("person {:02}", student_index),
                };
                student_index += 1;
                subjects.push(TableElement::Person {
                    uuid: Uuid::new_v4(),
                    person,
                });
            };
        }
        rows.push(subjects);
    }

    return TableElementMap::new(rows);
}

fn default_table_persons_status_map(elements: &TableElementMap) -> TablePersonsStatusMap
{
    let mut result = TablePersonsStatusMap::new();
    for row in elements.rows.iter()
    {
        for element in row
        {
            if let TableElement::Person { uuid, person: _ } = element
            {
                let rand_0:bool = rand::thread_rng().gen_bool(0.5);
                let rand_1:bool = rand::thread_rng().gen_bool(0.5);
                let status = match (rand_0, rand_1) 
                {
                    (true, true) => TablePersonStatus::Checked,
                    (false, false) => TablePersonStatus::Hanged,
                    _ => TablePersonStatus::Unchecked,
                };
                result.status.insert(uuid.clone(), status);
            };
        };
    };
    return result;
}
