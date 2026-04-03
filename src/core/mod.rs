pub mod persistence;
pub mod data_model;
pub mod codec;

pub use data_model::{Person, PersonsList, Position};
pub use persistence::{AppPaths, Storage};
