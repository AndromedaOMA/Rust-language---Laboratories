use std::fs::File;
use std::io::{Result, Write};

struct MyWriter {
    file: File,
}

impl MyWriter {
    fn new(file: File) -> MyWriter {
        MyWriter { file }
    }
}

impl Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let duplicated_buf: Vec<u8> = buf.iter().flat_map(|&byte| vec![byte, byte]).collect();
        self.file.write_all(&duplicated_buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.file.flush()
    }
}

fn main() -> Result<()> {
    let mut writer = MyWriter::new(File::create("a.txt")?);
    writer.write_all(b"abc")?;

    Ok(())
}
