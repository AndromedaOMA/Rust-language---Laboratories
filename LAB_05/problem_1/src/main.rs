// use std::io::{Error, ErrorKind};
// use std::{fs, io};
use std::fs;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
struct Data<'a> {
    name: &'a str,
    phone: &'a str,
    age: &'a str,
}

fn main() -> Result<()> {
    let s = fs::read_to_string("./src/text.txt")?;

    // let custom_error = Error::new(ErrorKind::Other, "Incomplete line!");

    let mut maximum_age = "0";
    let mut minimum_age = "200";

    let mut maximum_age_data: [&str; 3] = ["", "", ""];
    let mut minimum_age_data: [&str; 3] = ["", "", ""];

    for line in s.lines() {
        let mut split = line.split(',');

        let Some(extracted_name) = split.next() else {
            return Err(anyhow!("Incomplete line!"));
        };
        let Some(extracted_phone) = split.next() else {
            return Err(anyhow!("Incomplete line!"));
        };
        let Some(extracted_age) = split.next() else {
            return Err(anyhow!("Incomplete line!"));
        };

        if extracted_age > maximum_age {
            maximum_age = extracted_age;
            maximum_age_data[0] = extracted_name;
            maximum_age_data[1] = extracted_phone;
            maximum_age_data[2] = extracted_age;
        }
        
        if extracted_age < minimum_age {
            minimum_age = extracted_age;
            minimum_age_data[0] = extracted_name;
            minimum_age_data[1] = extracted_phone;
            minimum_age_data[2] = extracted_age;
        }
    }

    println!("The oldest student: {:?}", maximum_age_data);
    println!("The youngest student: {:?}", minimum_age_data);
    Ok(())
}
