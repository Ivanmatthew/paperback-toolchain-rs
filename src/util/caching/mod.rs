use std::fs::File;
use std::io::{Read, Write, Result};
use std::path::Path;
// buff writer is only useful for files greater than 8kb

pub struct DiskCache {
    file: File,
}

pub trait DiskCacheTrait {
    fn new(path: &Path) -> Result<Self> where Self: Sized;
    fn store(&mut self, data: &str) -> Result<()>;
    fn load(&mut self) -> Result<String>;
}

impl DiskCacheTrait for DiskCache {
    fn new(path: &Path) -> Result<Self> {
        let file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Failed to create disk cache file");
        Ok(Self { file })
    }

    fn store(&mut self, data: &str) -> Result<()> {
        self.file.write_all(data.as_bytes())
    }

    fn load(&mut self) -> Result<String> {
        let mut data = String::new();
        self.file.read_to_string(&mut data)?;
        Ok(data)
    }
}