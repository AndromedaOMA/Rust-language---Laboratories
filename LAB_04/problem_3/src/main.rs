use std::io::{Error, ErrorKind};
use std::{fs, io};

fn do_stuff() -> Result<(), io::Error> {
    let custom_error = Error::new(ErrorKind::Other, "oh no!");
    let mut s = fs::read_to_string("src/Text.txt")?;

    if s.find("pt").is_none()
        && s.find("ptr").is_none()
        && s.find("dl").is_none()
        && s.find("dna").is_none()
    {
        return Err(custom_error);
    }

    s = s.replace("pt", "pentru");
    s = s.replace("ptr", "pentru");
    s = s.replace("dl", "domnul");
    s = s.replace("dna", "doamna");
    print!("{s}");

    Ok(())
}

fn main() {
    print!("{:?}",do_stuff());
}
