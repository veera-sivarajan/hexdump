use std::io;

#[derive(Copy, Clone)]
pub struct Hexdump<I, O> {
    input: I,
    output: O,
}

impl<I, O> Hexdump<I, O>
where
    I: std::io::Read,
    O: std::io::Write,
{
    pub fn new(input: I, output: O) -> Self {
        Self { input, output }
    }

    fn format_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        let mut ascii_buf = String::with_capacity(16);
        for (count, byte) in bytes.iter().enumerate() {
            if count == 8 {
                write!(self.output, " ")?;
            }

            // store perusal format in a buffer
            if *byte >= 32 && *byte <= 126 {
                ascii_buf.push(*byte as char);
            } else {
                ascii_buf.push('.');
            }

            write!(self.output, "{:02x} ", byte)?;
        }

        // add padding when number of bytes in a line is less than 16
        for i in bytes.len()..16 {
            if i == 8 {
                write!(self.output, " ")?;
            }
            write!(self.output, "   ")?;
        }

        // print perusal format
        writeln!(self.output, " |{}|", ascii_buf)?;
        Ok(())
    }

    pub fn print(&mut self) -> io::Result<()> {
        let mut count = 0;
        loop {
            let mut buffer = [0; 16];
            let len = self.input.read(&mut buffer)?;
            if len == 0 {
                writeln!(self.output, "{:08x}", count)?;
                break;
            } else {
                // print the index of first byte in line
                write!(self.output, "{:08x}  ", count)?;
                self.format_bytes(&buffer)?;
                count += len;
            }
        }
        Ok(())
    }
}
