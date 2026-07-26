use std::path::PathBuf;
use std::sync::LazyLock;

pub static STORAGE: LazyLock<Storage> = LazyLock::new(Storage::new);

pub struct Storage
{
    config_file_path: PathBuf,
}

// public storage access api
impl Storage
{
    pub fn load_table_data(&self) -> anyhow::Result<TableData>
    {
        // self.load_app_config()?
        //     .table_data
        //     .ok_or_else(|| anyhow::anyhow!("No table data found"))
        let column_count: u32 = 10;
        let row_count: u32 = 8;
        let mut personnel: Vec<Person> = vec![];
        let mut seats_assignment: Vec<SeatAssignment> = vec![];

        for x in 1..column_count+1 {
            for y in 1..row_count+1 {
                let id: u64 = (((x - 1) * row_count) + y) as u64;
                personnel.push(Person {
                    id,
                    name: random_name::random_chinese_name()
                });
                seats_assignment.push(SeatAssignment {
                    coordinate: TableCoordinate { x, y },
                    person_id: id,
                });
            }
        }

        return Ok(TableData {
            personnel: personnel,
            table_layout: TableLayout {
                column_count,
                row_count,
                seats_assignment: seats_assignment,
            },
        });
    }

    pub fn load_app_settings(&self) -> anyhow::Result<AppBehaviorSettings>
    {
        Ok(self.load_app_config()?.app_behavior_settings)
    }
}

// public storage saving api
impl Storage
{
    pub fn save_table_data(&self, new_table_data: &TableData) -> anyhow::Result<()>
    {
        let mut config = self.load_app_config()?;

        config.table_data = Some(new_table_data.clone());

        self.save_app_config(&config)
    }

    pub fn save_app_settings(&self, new_app_settings: &AppBehaviorSettings) -> anyhow::Result<()>
    {
        let mut config = self.load_app_config()?;

        config.app_behavior_settings = new_app_settings.clone();

        self.save_app_config(&config)
    }
}

// parivate construction
impl Storage
{
    fn new() -> Self
    {
        let config_file_path = match path_provider::config_file_path() {
            Ok(path) => path,
            Err(_) => panic!("Fail to locate config file path"),
        };
        Self { config_file_path }
    }
}

use std::fs;

use crate::storage::{AppBehaviorSettings, AppConfig, Person, TableCoordinate, TableData, TableLayout, path_provider};
use crate::storage::data_models::table_persistent::SeatAssignment;
use crate::utilities::random_name;

// parivate api
impl Storage
{
    fn load_app_config(&self) -> anyhow::Result<AppConfig>
    {
        let content = fs::read_to_string(&self.config_file_path)?;

        let config: AppConfig = toml::from_str(&content)?;

        Ok(config)
    }

    fn save_app_config(&self, config: &AppConfig) -> anyhow::Result<()>
    {
        let content = toml::to_string_pretty(config)?;

        fs::write(&self.config_file_path, content)?;

        Ok(())
    }
}
