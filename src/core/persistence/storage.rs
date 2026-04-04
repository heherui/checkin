use std::{io::Write, path::PathBuf};

use atomic_write_file::AtomicWriteFile;

use crate::core::persistence::AppPaths;

#[derive(Debug)]
pub struct Storage
{
    
}

impl Storage
{
    pub fn new()-> Self
    {
        Self { }
    }
}

#[derive(Debug)]
pub enum StorageError
{
    IO(std::io::Error),
}

impl Storage 
{
    pub fn save(&self, data:Vec<u8>, path:&AppPaths)-> Result<(),StorageError>
    {
        let path = path.solved();

        if let Err(e) = self.ensure_exists(&path) 
        {
            return Err(StorageError::IO(e))
        };

        let mut f = match AtomicWriteFile::options().open(path)
        {
            Ok(f) => f,
            Err(e) => { 
                return Err(StorageError::IO(e));
            },
        };

        if let Err(e) = f.write_all(&data)
        {
            return Err(StorageError::IO(e));
        };

        if let Err(e) = f.commit()
        {
            return Err(StorageError::IO(e));
        };

        return Ok(());
    }

    fn ensure_exists(&self, path:&PathBuf)-> Result<(),std::io::Error>
    {   
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;

        Ok(())
    }
}