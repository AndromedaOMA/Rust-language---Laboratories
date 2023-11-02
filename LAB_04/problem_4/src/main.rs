use std::{fs, io};

fn do_stuff() -> Result<(), io::Error> {
    let s = fs::read_to_string("C:/Windows/System32/drivers/etc/services")?;
    let mut index;
    let mut final_string = String::new();
    for lines in s.lines() {
        if lines.find('#').is_some() {
            index = lines.find('#').unwrap();
        } else {
            println!("{lines}");
            index = 0;
        }

        if lines.is_empty() {
            continue;
        } else if index != 0 {
            for c in lines.splitn(3, ' ') {
                final_string.push_str(c);
                final_string.push_str(" => ");
            }
        }
        println!("{final_string}");
    }
    Ok(())
}

fn main() {
    _ = do_stuff();
}
