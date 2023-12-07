use regex::Regex;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a string:");
    let _=io::stdin()
        .read_line(&mut input);

    let mut map = HashMap::<&str, i32>::with_capacity(100);

    //source: https://docs.rs/regex/latest/regex/struct.Regex.html#method.find_iter
    let re = Regex::new(r"\b\w+\b").unwrap();
    let words: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();

    for word in words {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    let mut vec: Vec<(&str, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    //source: https://gdt050579.github.io/rust_course_fii/labs/lab08/lab08.html
    vec.sort_by(|a, b| b.1.cmp(&a.1));

    for (word, count) in &vec {
        println!("{} => {}", word, count);
    }
}
