use std::{fs, io};

fn do_stuff() -> Result<(), io::Error> {
    let s = fs::read_to_string("/c/Windows/System32/drivers/etc/services")?;

    for l in s.lines() {
        if l.is_empty() {
            continue;
        } else if l.find('#').is_some() {
            println!("{l}");
        }
    }

    Ok(())
}

fn main() {
    _ = do_stuff();
}
