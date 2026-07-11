use serde::{Deserialize, Serialize};

use crate::table_layout::TableLayout;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig
{
    #[serde(default)]
    pub table_data:Option<TableData>,
    
    #[serde(default)]
    pub app_behavior_settings: AppBehaviorSettings,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TableData
{
    pub personnel: Vec<Person>,
    pub table_layout: TableLayout,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Person
{
    pub id: PersonId,
    pub name: String,
}

pub type PersonId = u64;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct AppBehaviorSettings {}
