use std::io::{Error, ErrorKind};
use std::{fs, io};

fn ROT13() -> Result<(), io::Error> {
    let custom_error = Error::new(ErrorKind::Other, "oh no!");
    let s = fs::read_to_string("src/input.txt")?;
    let mut final_string: String = String::from("");
    let mut result;
    for i in s.chars() {
        if !(i >= 'a' && i <= 'z' || i >= 'A' && i <= 'Z') {
            return Err(custom_error);
        } else if !(i >= 'a' && i <= 'n' || i >= 'A' && i <= 'N') {
            let ascii_value = i as u8;
            let ch_plus = ascii_value - 13;
            result = ch_plus as char;
        } else {
            let ascii_value = i as u8;
            let ch_plus = ascii_value + 13;
            result = ch_plus as char;
        }
        final_string.push(result);
    }
    fs::write("output.txt", &final_string)?;

    Ok(())
}

fn main() {
    let _ = ROT13();
}
