use std::fmt::Write;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub enum HexdumpError {
    PathIsDir,
    FileOpen,
    ReadBuffer,
    WriteBuffer,
}

type Result<T> = std::result::Result<T, HexdumpError>;

trait HexPrinter {
    fn print(&self, count: usize) -> Result<()>;
}

impl HexPrinter for &[u8] {
    fn print(&self, count: usize) -> Result<()> {
        print!("{:08x}  ", count); // print the index of first byte in line
        let mut ascii_buf = String::with_capacity(16);
        for (count, byte) in self.iter().enumerate() {
            if count == 8 {
                print!(" ");
            }

            // store perusal format in a buffer
            if *byte >= 32 && *byte <= 126 {
                write!(ascii_buf, "{}", *byte as char).map_err(|_err| HexdumpError::WriteBuffer)?;
            } else {
                write!(ascii_buf, ".").map_err(|_err| HexdumpError::WriteBuffer)?;
            }

            print!("{:02x} ", byte);
        }

        // add padding when number of bytes in a line is less than 16
        let mut len = self.len();
        while len < 16 {
            if len == 8 {
                print!(" ");
            }
            print!("   ");
            len += 1;
        }

        // print perusal format
        if !ascii_buf.is_empty() {
            println!(" |{}|", ascii_buf);
        } else {
            println!();
        }
        Ok(())
    }
}

pub fn print(path: impl AsRef<Path>) -> Result<()> {
    if path.as_ref().is_dir() {
        Err(HexdumpError::PathIsDir)
    } else {
        File::open(path)
            .map_err(|_err| HexdumpError::FileOpen)
            .and_then(|file| {
                let mut reader = BufReader::with_capacity(16, file);
                let mut count = 0;
                loop {
                    let byte_slice = reader.fill_buf().map_err(|_err| HexdumpError::ReadBuffer)?;
                    let len = byte_slice.len();
                    if len == 0 {
                        println!("{:08x}", count);
                        break;
                    } else {
                        byte_slice.print(count)?;
                        reader.consume(len);
                        count += len;
                    }
                }
                Ok(())
            })
    }
}
