use anyhow::anyhow;
use anyhow::Result;
use serde_derive::Deserialize;
use std::fs;

// source: https://stackoverflow.com/questions/35458562/how-can-i-implement-rusts-copy-trait
#[derive(Debug, Deserialize, Clone, Copy)]
struct Data<'a> {
    name: &'a str,
    phone: &'a str,
    age: i32,
}

// source: https://stackoverflow.com/questions/24855830/how-convert-a-string-in-str-in-impl-block-in-rust
use std::fmt;
impl fmt::Display for Data<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            " \"name\": \"{}\", \"phone\": \"{}\", \"age\": {}",
            self.name, self.phone, self.age
        )
    }
}

// fn main() {
//     let content = fs::read_to_string("./src/text.txt").unwrap();
//     let p: Data = serde_json::from_str(&content).unwrap();
//     println!("{:?}", p);
// }

fn main() -> Result<()> {
    let s = fs::read_to_string("./src/text.txt")?;

    let _maximum_age = "0";
    let _minimum_age = "200";

    let mut _maximum_age_data: Data = Data {
        name: "",
        phone: "",
        age: 0,
    };
    let mut _minimum_age_data: Data = Data {
        name: "",
        phone: "",
        age: 0,
    };

    for line in s.lines() {
        let p: Data = serde_json::from_str(&line).unwrap();
        // println!("{:?}", p);
        let mut split = line.rsplit(':');
        let Some(mut extracted_age) = split.next() else {
            return Err(anyhow!("Incomplete line!"));
        };
        extracted_age = &extracted_age[1..extracted_age.len() - 2];

        // println!("{}", extracted_age);

        if extracted_age < _minimum_age {
            _minimum_age_data = p;
        } else if extracted_age > _maximum_age {
            _maximum_age_data = p;
        }
    }
    // let binding1 = _minimum_age_data.to_string();
    // let p1: Data = serde_json::from_str(&binding1).unwrap();
    // println!("{}", p1);
    let binding1 = _minimum_age_data.to_string();
    let p1: Data = serde_json::from_str(&binding1).unwrap();
    println!("{}", p1);
    let binding2 = _maximum_age_data.to_string();
    let p2: Data = serde_json::from_str(&binding2).unwrap();
    println!("{}", p2);
    
    Ok(())
}
