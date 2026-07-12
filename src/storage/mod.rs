mod storage_api;

mod data_models;
pub mod path_provider;

pub use data_models::{
    app_config::AppConfig, app_settings::AppBehaviorSettings, personnel::Person,
    table_persistent::{TableData, TableCoordinate, TableLayout},
    personnel::PersonId
};
pub use storage_api::STORAGE;
