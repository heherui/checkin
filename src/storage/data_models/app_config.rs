use serde::{Deserialize, Serialize};

use crate::storage::data_models::{app_settings::AppBehaviorSettings, table_persistent::TableData};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig
{
    #[serde(default)]
    pub table_data:Option<TableData>,
    
    #[serde(default)]
    pub app_behavior_settings: AppBehaviorSettings,
}
