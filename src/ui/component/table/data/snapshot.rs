use crate::{core::{Person, Position}, ui::component::{Table, table::data::status::TablePersonStatus}};

#[derive(Debug, Clone)]
pub struct TablePersonsDataSnapshot
{
    cells:Vec<TablePersonsCellDataSnapshot>
}

#[derive(Debug, Clone)]
pub struct TablePersonsCellDataSnapshot
{
    person:Person,
    position:Position,
    status:TablePersonStatus,
}

impl Table 
{
    pub fn perons_data_snapshot(&self)-> TablePersonsDataSnapshot
    {
        
    }
}