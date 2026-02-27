use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Configuration {
    pub config_file: PathBuf,
}

impl Configuration {
    pub fn new(config_file: PathBuf) -> Self {
        Self { config_file }
    }

    pub fn default_config_file() -> PathBuf {
        env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(Path::to_path_buf))
            .or_else(|| env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."))
            .join("table.conf.json")
    }
}
