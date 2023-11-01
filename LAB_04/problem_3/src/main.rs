use std::{fs, io};

fn do_stuff() -> Result<(), io::Error> {
    let mut s = fs::read_to_string("src/Text.txt")?;

    s = s.replace("pt", "pentru");
    s = s.replace("ptr", "pentru");
    s = s.replace("dl", "domnul");
    s = s.replace("dna", "doamna");
    print!("{s}");

    Ok(())
}

fn main() {
    let _ = do_stuff();
}
