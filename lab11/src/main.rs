use std::fs::File;
use std::io::{self, Write, Result};

struct MyWriter {
    file: File,
}

impl MyWriter {
    fn new(file: File) -> MyWriter {
        MyWriter { file }
    }
}

impl Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut duplicate = Vec::with_capacity(buf.len() * 2);
        for &byte in buf {
            duplicate.push(byte);
            duplicate.push(byte);
        }

        self.file.write_all(&duplicate)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

fn main() -> Result<()> {
    let mut writer = MyWriter::new(File::create("a.txt")?);
    writer.write_all(b"abc")?;

    Ok(())
}
