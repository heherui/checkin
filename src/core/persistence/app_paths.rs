use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum AppPaths
{
    UserData { filename:String },
    CacheFile { filename:String },
}

impl AppPaths 
{
    #[cfg(target_os = "windows")]
    pub fn solved(&self)-> PathBuf
    {
        let install_folder = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));

        return match self 
        {
            AppPaths::UserData { filename } => { 
                install_folder.join("Data").join(filename) 
            },
            AppPaths::CacheFile { filename } => {
                install_folder.join("Cache").join(filename)
            },
        }
    }

    #[cfg(target_os = "macos")]
    pub fn solved(&self)-> PathBuf
    {
        let home_dir = std::env::var_os("HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::current_dir().ok())
            .unwrap_or_else(|| PathBuf::from("."));

        // On macOS, using ~/Library ensures standard locations; in sandboxed apps
        // HOME already points to the container's Data directory.
        let app_name = crate::APPLICATION_ID;

        match self
        {
            AppPaths::UserData { filename } => {
                home_dir
                    .join("Library")
                    .join("Application Support")
                    .join(app_name)
                    .join(filename)
            },
            AppPaths::CacheFile { filename } => {
                home_dir
                    .join("Library")
                    .join("Caches")
                    .join(app_name)
                    .join(filename)
            },
        }
    }
}
