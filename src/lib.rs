use std::{fs::File};
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub enum HexdumpError {
    PathIsDir,
}

type Result<T> = std::result::Result<T, HexdumpError>;

pub fn print(path: impl AsRef<Path>) -> Result<String> {
    if path.as_ref().is_dir() {
        Err(HexdumpError::PathIsDir)
    } else {
        Ok(format!("HELLO"))
    }
}
    
    
