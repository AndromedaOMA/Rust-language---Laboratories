use std::io::Write;
// use std::env::args;
use std::fs::OpenOptions;
use std::io::BufWriter;
fn write_data_to_file(path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new().append(true).write(true).open(&path)?;

    let mut file = BufWriter::new(file);

    file.write_all(&data.as_bytes())?;

    // let remaining = file.write(&data)?;

    // if remaining > 0 {
    //     return Err("Could not write all data to file".into());
    // }

    // file.flush()?;

    Ok(())
}

fn main() {
    match write_data_to_file("test.txt", "Hello World!") {
        Ok(_) => println!("Data written to file"),
        Err(e) => println!("Error writing to file: {}", e),
    }
}

//source: https://www.youtube.com/watch?v=CE_7vZKMs2Y