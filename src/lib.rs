use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub enum HexdumpError {
    PathIsDir,
    FileOpen,
    ReadBuffer,
}

type Result<T> = std::result::Result<T, HexdumpError>;

pub fn print(path: impl AsRef<Path>) -> Result<String> {
    if path.as_ref().is_dir() {
        Err(HexdumpError::PathIsDir)
    } else {
        File::open(path)
            .map_err(|_err| HexdumpError::FileOpen)
            .and_then(|file| {
                let mut reader = BufReader::with_capacity(16, file);
                loop {
                    let slice = reader.fill_buf()
                        .map_err(|_err| HexdumpError::ReadBuffer)?;
                    let len = slice.len();
                    if len == 0 {
                        break;
                    } else {
                        println!("{:?}", slice);
                        reader.consume(len);
                    }
                }
                Ok(format!("HELLO"))
            })
    }
}
