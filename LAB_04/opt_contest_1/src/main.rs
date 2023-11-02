use std::io::{Error, ErrorKind};
use std::{fs, io};

fn rot13() -> Result<(), io::Error> {
    let custom_error = Error::new(ErrorKind::Other, "oh no!");
    let s = fs::read_to_string("./input.txt")?;
    let mut final_string: String = String::from("");
    let mut result;
    for i in s.chars() {
        if !(i as u32 <= 127) {
            return Err(custom_error);
        } else if i as u8 == 32 {
            result = ' ';
        } else if i >= 'n' && i <= 'z' || i >= 'N' && i <= 'Z' {
            let ascii_value = i as i8;
            let ch_plus = (ascii_value - 13) as u8;
            result = ch_plus as char;
        } else if i >= 'a' && i < 'n' || i >= 'A' && i < 'N' {
            let ascii_value = i as u8;
            let ch_plus = ascii_value + 13;
            result = ch_plus as char;
        } else {
            result = i;
        }
        final_string.push(result);
    }
    fs::write("output.txt", &final_string)?;

    Ok(())
}

fn main() {
    print!("{:?}", rot13());
}
