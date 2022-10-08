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

trait HexPrinter {
    fn print(&self, count: usize);
}

impl HexPrinter for &[u8] {
    fn print(&self, count: usize) {
        print!("{:08x} ", count);
        for (count, byte) in self.iter().enumerate() {
            if count == 8 {
                print!(" ");
            }
            
            print!("{:02x} ", byte);
        }
        println!();
    }
}

pub fn print(path: impl AsRef<Path>) -> Result<()> {
    if path.as_ref().is_dir() {
        Err(HexdumpError::PathIsDir)
    } else {
        let file = File::open(path).map_err(|_err| HexdumpError::FileOpen)?;
        let mut reader = BufReader::with_capacity(16, file);
        let mut count = 0;
        loop {
            let slice = reader.fill_buf().map_err(|_err| HexdumpError::ReadBuffer)?;
            let len = slice.len();
            slice.print(count);
            if len == 0 {
                break;
            } else {
                reader.consume(len);
                count += len;
            }
        }
        Ok(())
    }
}
