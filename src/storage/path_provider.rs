use std::path::PathBuf;

pub fn config_file_path() -> anyhow::Result<PathBuf>
{
    let mut path =
        dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!(
                "Failed to locate config directory"
            ))?;

    path.push(env!("CARGO_PKG_NAME"));

    std::fs::create_dir_all(&path)?;

    path.push("config.toml");

    Ok(path)
}