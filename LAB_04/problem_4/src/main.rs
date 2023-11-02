use std::{fs, io};

fn do_stuff() -> Result<(), io::Error> {
    let s = fs::read_to_string("C:/Windows/System32/drivers/etc/services")?;
    let mut index;
    for l in s.lines() {
        if l.find('#').is_some() {
            index = l.find('#').unwrap();
        } else {
            index = 0;
        }

        if l.is_empty() {
            continue;
        } else if index != 0 {
            let mut columns = l.split_whitespace();

            if let Some(first_column) = columns.next() {
                if let Some(second_column) = columns.next() {
                    println!("{} => {}", first_column, second_column);
                }
            }
        }
    }
    Ok(())
}

fn main() {
    _ = do_stuff();
}
