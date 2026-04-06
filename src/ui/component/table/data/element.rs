use uuid::Uuid;

use crate::{core::Person, ui::component::{TableCellColors, TableCellData, table::data::status::TablePersonStatus}};

#[derive(Debug, Clone)]
pub struct TableElementMap
{
    pub rows: Vec<Vec<TableElement>>,
}

#[derive(Debug, Clone)]
pub enum TableElement
{
    Person
    {
        uuid: Uuid,
        person: Person,
    },
    Transparent,
    Block
    {
        name: Option<String>,
    },
}

// - MARK: impl

impl TableCellData
{
    pub fn new_transparent() -> Self
    {
        return Self {
            label: None,
            colors: TableCellColors {
                background_color: String::from("rgba(0,0,0,0)"),
                label_color: String::from("rgba(0,0,0,0)"),
                border_color: String::from("rgba(0,0,0,0)"),
            },
        };
    }

    pub fn new_block(label: Option<String>) -> Self
    {
        return Self {
            label: label,
            colors: TableCellColors {
                background_color: String::from("#475569"),
                label_color: String::from("#cbd5e1"),
                border_color: String::from("#334155"),
            },
        };
    }

    pub fn from_person_with_status(person: &Person, status: &TablePersonStatus) -> Self
    {
        return Self {
            label: Some(person.name.clone()),
            colors: TableCellColors {
                background_color: status.background_color(),
                label_color: status.label_color(),
                border_color: status.border_color(),
            },
        };
    }
}

impl TableElementMap
{
    pub fn new(rows: Vec<Vec<TableElement>>) -> Self
    {
        return Self { rows };
    }
}