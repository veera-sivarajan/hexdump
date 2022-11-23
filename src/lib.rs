use std::io;

pub struct Hexdump<I, O>
{
    input: I,
    output: O,
    index: usize,
}

impl<I, O> Hexdump<I, O>
where
    I: io::Read,
    O: io::Write,
{
    pub fn new(input: I, output: O) -> Self {
        Self { input, output, index: 0 }
    }

    fn format_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        let mut ascii_buf = String::with_capacity(16);
        for index in 0..16 {
            if index == 8 {
                write!(self.output, " ")?;
            }

            if index >= bytes.len() {
                write!(self.output, "   ")?;
            } else {
                // store perusal format in a buffer
                let byte = bytes[index];
                if (32..=126).contains(&byte) {
                    ascii_buf.push(byte as char);
                } else {
                    ascii_buf.push('.');
                }

                write!(self.output, "{:02x} ", byte)?;
            }
        }

        // print perusal format
        writeln!(self.output, " |{}|", ascii_buf)
    }

    pub fn print(&mut self) -> io::Result<()> {
        loop {
            let mut buffer = [0; 1024];
            let len = self.input.read(&mut buffer)?;
            if len == 0 {
                writeln!(self.output, "{:08x}", self.index)?;
                break;
            }
            for slice in buffer[..len].chunks(16) {
                write!(self.output, "{:08x}  ", self.index)?;
                self.format_bytes(slice)?;
                self.index += slice.len();
            }
        }
        Ok(())
    }
}
