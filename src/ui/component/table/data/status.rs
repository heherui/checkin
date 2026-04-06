use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TablePersonsStatusMap
{
    pub status: HashMap<Uuid, TablePersonStatus>,
}

#[derive(Debug, Clone)]
pub enum TablePersonStatus
{
    Checked,
    Unchecked,
    Hanged,
}

// - MARK: impl

impl TablePersonStatus
{
    pub fn default() -> Self
    {
        Self::Unchecked
    }
}

impl TablePersonsStatusMap
{
    pub fn new() -> Self
    {
        return Self {
            status: HashMap::new(),
        };
    }
}


impl TablePersonStatus
{
    pub fn background_color(&self) -> String
    {
        return match self
        {
            TablePersonStatus::Checked => String::from("rgba(26, 201, 70, 0.6)"),
            TablePersonStatus::Unchecked => String::from("rgba(205, 13, 16, 0.6)"),
            TablePersonStatus::Hanged => String::from("rgba(201, 160, 26, 0.8)"),
        };
    }

    pub fn label_color(&self) -> String
    {
        return String::from("rgb(0, 0, 0)");
    }

    pub fn border_color(&self) -> String
    {
        return match self
        {
            TablePersonStatus::Checked => String::from("rgb(6, 143, 40)"),
            TablePersonStatus::Unchecked => String::from("rgb(143, 9, 11)"),
            TablePersonStatus::Hanged => String::from("rgb(143, 112, 11)"),
        };
    }
}
