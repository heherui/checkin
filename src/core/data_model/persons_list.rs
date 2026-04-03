use serde::{Deserialize, Serialize};

use crate::core::data_model::Person;

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonsList
{
    pub colomn_count:u32,
    pub row_count:u32,

    #[serde(default)]
    pub persons:Vec<PersonListEntry>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  PersonListEntry
{
    pub position:Position,
    pub person:Person,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position
{
    pub x:u32,
    pub y:u32,
}

impl PersonsList
{
    pub fn insert(&mut self, entry:PersonListEntry)-> Result<(),String>
    {
        if self.persons.iter().any(|e|
            e.position.x == entry.position.x &&
            e.position.y == entry.position.y)
        {
            return Err(String::from("duplicate position"));
        }

        self.persons.push(entry);
        return Ok(())
    }

    pub fn upsert(&mut self, entry:PersonListEntry)
    {
        if let Some(e) = self.persons.iter_mut().find(|e|
            e.position.x == entry.position.x &&
            e.position.y == entry.position.y)
        {
            *e = entry;
        }
        else
        {
            self.persons.push(entry);
        }
    }
}