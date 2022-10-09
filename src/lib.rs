use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::io;

#[derive(Debug)]
pub enum HexdumpError {
    IOError(io::Error),
    PathIsDir,
}

impl From<io::Error> for HexdumpError {
    fn from(err: io::Error) -> Self {
        HexdumpError::IOError(err)
    }
}

type Result<T> = std::result::Result<T, HexdumpError>;

trait HexPrinter {
    fn print(&self, count: usize);
}

impl HexPrinter for &[u8] {
    fn print(&self, count: usize) {
        print!("{:08x}  ", count); // print the index of first byte in line
        // let mut ascii_buf = String::with_capacity(16);
        let mut ascii_buf = ['0'; 16];
        let mut ascii_buf_index = 0;
        for (count, byte) in self.iter().enumerate() {
            if count == 8 {
                print!(" ");
            }

            // store perusal format in a buffer
            if *byte >= 32 && *byte <= 126 {
                // ascii_buf.push(*byte as char);
                ascii_buf[ascii_buf_index] = *byte as char;
            } else {
                // ascii_buf.push('.');
                ascii_buf[ascii_buf_index] = '.'; 
            }
            ascii_buf_index += 1;

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
            print!(" |");
            for a in ascii_buf {
                print!("{a}");
            }
            println!("|");
        } else {
            println!();
        }
    }
}

pub fn print(path: impl AsRef<Path>) -> Result<()> {
    if path.as_ref().is_dir() {
        Err(HexdumpError::PathIsDir)
    } else {
        let file = File::open(path)?;
        let mut reader = BufReader::with_capacity(16, file);
        let mut count = 0;
        loop {
            let byte_slice = reader.fill_buf()?;
            let len = byte_slice.len();
            if len == 0 {
                println!("{:08x}", count);
                break;
            } else {
                byte_slice.print(count);
                reader.consume(len);
                count += len;
            }
        }
        Ok(())
    }
}
