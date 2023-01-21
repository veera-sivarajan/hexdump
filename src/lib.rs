//! This library writes a hexdump of the given input in
//! canonical, verbose format to the specified output.
//! In other words, it produces the same effect as
//! invoking `hexdump -Cv`.

use std::io;

pub struct Hexdump<O>(O);

const BYTE_COUNT: usize = 16;

impl<O> Hexdump<O>
where
    O: io::Write,
{
    pub fn new(output: O) -> Self {
        Hexdump(output)
    }

    fn format_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        let mut ascii_buf = String::with_capacity(BYTE_COUNT);
        for index in 0..BYTE_COUNT {
            if index == 8 {
                write!(self.0, " ")?;
            }

            if index >= bytes.len() {
                write!(self.0, "   ")?;
            } else {
                // store perusal format in a buffer
                let byte = bytes[index];
                if (32..=126).contains(&byte) {
                    ascii_buf.push(byte as char);
                } else {
                    ascii_buf.push('.');
                }

                write!(self.0, "{:02x} ", byte)?;
            }
        }

        // print perusal format
        writeln!(self.0, " |{}|", ascii_buf)
    }

    pub fn print(&mut self, input: &mut impl io::Read) -> io::Result<()> {
        const BUFFER_SIZE: usize = 4096;
        let mut index = 0;
        loop {
            let mut buffer = [0; BUFFER_SIZE];
            let len = input.read(&mut buffer)?;
            if len == 0 {
                writeln!(self.0, "{:08x}", index)?;
                break;
            }
            for slice in buffer[..len].chunks(BYTE_COUNT) {
                write!(self.0, "{:08x}  ", index)?;
                self.format_bytes(slice)?;
                index += slice.len();
            }
        }
        Ok(())
    }
}
