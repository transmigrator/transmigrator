// file_manager/mod.rs

use std::fs::File;
use std::io::{Read, Write};

pub struct FileManager {
    selected_file: Option<String>,
}

impl FileManager {
    pub fn new() -> FileManager {
        FileManager {
            selected_file: None,
        }
    }

    pub fn select_file(&mut self, file_path: &str) {
        self.selected_file = Some(file_path.to_string());
    }

    pub fn download_file(&self, data: &str, filename: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(filename)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn upload_file(&self) -> Result<String, std::io::Error> {
        if let Some(file_path) = &self.selected_file {
            let mut file = File::open(file_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No file selected"))
        }
    }

    pub fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn write_file(path: &str, data: &[u8]) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        file.write_all(data)?;
        Ok(())
    }
}
