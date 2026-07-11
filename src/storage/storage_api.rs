use std::path::PathBuf;

use crate::storage::datamodel::{AppBehaviorSettings, AppConfig};
use crate::storage::{TableData, path_provider};
use crate::table_layout::TableLayout;

use std::sync::LazyLock;

pub static STORAGE: LazyLock<Storage> =
    LazyLock::new(Storage::new);

pub struct Storage
{
    config_file_path:PathBuf,
}

// public storage access api
impl Storage 
{
    pub fn load_table_data(
        &self,
    ) -> anyhow::Result<TableData>
    {
        // self.load_app_config()?
        //     .table_data
        //     .ok_or_else(|| anyhow::anyhow!("No table data found"))
        return Ok(TableData {
            personnel: vec![],
            table_layout: TableLayout {
                column_count: 10,
                row_count: 8,
                persons_mapper: vec![],
            },
        })
    }

    pub fn load_app_settings(
        &self,
    ) -> anyhow::Result<AppBehaviorSettings>
    {
        Ok(
            self.load_app_config()?
                .app_behavior_settings
        )
    }
}

// public storage saving api
impl Storage 
{
    pub fn save_table_data(
        &self,
        new_table_data: &TableData,
    ) -> anyhow::Result<()>
    {
        let mut config =
            self.load_app_config()?;

        config.table_data =
            Some(new_table_data.clone());

        self.save_app_config(&config)
    }


    pub fn save_app_settings(
        &self,
        new_app_settings: &AppBehaviorSettings,
    ) -> anyhow::Result<()>
    {
        let mut config =
            self.load_app_config()?;

        config.app_behavior_settings =
            new_app_settings.clone();

        self.save_app_config(&config)
    }
}

// parivate construction
impl Storage 
{
    fn new()-> Self
    {
        let config_file_path = match path_provider::config_file_path() {
            Ok(path) => path,
            Err(_) => panic!("Fail to locate config file path"),
        };
        Self { config_file_path }
    }
}

use std::fs;

// parivate api
impl Storage
{
    fn load_app_config(
        &self,
    ) -> anyhow::Result<AppConfig>
    {
        let content =
            fs::read_to_string(&self.config_file_path)?;

        let config: AppConfig =
            toml::from_str(&content)?;

        Ok(config)
    }

    fn save_app_config(
        &self,
        config: &AppConfig,
    ) -> anyhow::Result<()>
    {
        let content =
            toml::to_string_pretty(config)?;

        fs::write(
            &self.config_file_path,
            content,
        )?;

        Ok(())
    }
}
