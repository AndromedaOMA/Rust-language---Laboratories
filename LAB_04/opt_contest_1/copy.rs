use std::{fs, io};

fn do_stuff() -> Result<(), io::Error> {
    let s = fs::read_to_string("src/main.rs")?;
    fs::write("copy.rs", &s)?;

    Ok(())
}
 fn main() {
    let _=do_stuff();
 }