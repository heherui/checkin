use std::path::PathBuf;

pub fn config_file_path() -> anyhow::Result<PathBuf>
{
    // In debug builds, use the current working directory.
    if cfg!(debug_assertions) {
        
        eprintln!("enforcing config file path as current directory beacause this is a debug build.");
        let mut config_dir = std::env::current_dir()?;
        config_dir.push("target/debug/config");
        std::fs::create_dir_all(&config_dir)?;
        return Ok(config_dir);
    };

    // config_dir()/<package_name>/config.toml
    let mut config_dir = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Failed to locate config directory"))?;
    config_dir.push(env!("CARGO_PKG_NAME"));
    std::fs::create_dir_all(&config_dir)?;
    config_dir.push("config.toml");
    Ok(config_dir)
}
