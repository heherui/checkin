mod storage_api;

mod datamodel;
pub mod path_provider;

pub use datamodel::{TableData, PersonId};
pub use storage_api::STORAGE;