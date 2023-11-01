use std::{fs, io};

fn longest() -> Result<(), io::Error> {
    let s = fs::read_to_string("src/Text.txt")?;
    let mut no_ch = 0;
    let mut no_line = 0;
    let mut no_ch_max=0;
    let mut no_bytes_max=0;
    let mut no_line_ch_max = 0;
    let mut no_line_bytes_max = 0;

    for k in s.lines() {
        no_line += 1;
        if k.len()>no_bytes_max {
            no_bytes_max=k.len();
            no_line_bytes_max=no_line;
        }
        for _i in s.chars() {
            no_ch += 1;
        }
        if k.len()>no_ch_max {
            no_ch_max=no_ch;
            no_line_ch_max=no_line;
        }
    }

    println!("The longest line contain the max number of bytes is {no_line_bytes_max} and the max number of characters is {no_line_ch_max}");

    Ok(())
}

fn main() {
    let _ = longest();
}
