use regex::Regex;
use std::io::{Error, ErrorKind};
use std::{fs, io};
// use std::iter::repeat_with;

use clap::Parser;

#[derive(Parser)]
#[command(
    version,
    about = "Decide if the password is generated randomly or from a dictionary"
)]
struct Args {
    /// Title of the file directory of the dictionary
    #[arg(long, short, default_value = "None")]
    dict: String,
}

use lazy_static::lazy_static;

lazy_static! {
    static ref LIST_OF_ALL_CHARS: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!?#@".chars().collect();
    static ref LIST_OF_SPECIAL_CHARS: Vec<char> = "!?#@".chars().collect();
    static ref LIST_OF_UPPERCASE_CHARS: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
}

fn random_password() -> String {
    //============================================RANDOM PASSWORD==========================================
    let random_length = fastrand::usize(12..19);

    let password: String = (0..random_length)
        .map(|current_index| {
            if current_index == 0 {
                let random_index = fastrand::usize(..LIST_OF_UPPERCASE_CHARS.len());
                LIST_OF_UPPERCASE_CHARS[random_index].to_string()
            } else if current_index == 1 {
                let random_index = fastrand::usize(..LIST_OF_SPECIAL_CHARS.len());
                LIST_OF_SPECIAL_CHARS[random_index].to_string()
            } else {
                let random_index = fastrand::usize(..LIST_OF_ALL_CHARS.len());
                LIST_OF_ALL_CHARS[random_index].to_string()
            }
        })
        .collect();
    return password;
}
fn dictionary_password(dic: String) -> String {
    //============================================DICTIONARY PASSWORD======================================
    let path = format!("./src/{}", dic);
    let binding = fs::read_to_string(path).expect("NOTHING");
    let s:&str= binding.as_str();

    let re = Regex::new(r"\S+").unwrap();
    let words: Vec<&str> = re.find_iter(s).map(|m| m.as_str()).collect();

    let random_length = fastrand::usize(12..19);
    let mut password_length = 0;
    let password: String = (0..random_length)
        .map(|_| {
            let random_index = fastrand::usize(..words.len());
            if password_length + words[random_index].len() <= random_length {
                password_length += words[random_index].len();
                words[random_index].to_string()
            } else {
                "".to_string()
            }
        })
        .collect();
    return password;
}

fn main() -> Result<(), io::Error> {
    let _custom_error = Error::new(ErrorKind::Other, "Wrong command!");

    let args = Args::parse();
    if args.dict != "None" {
        println!("Hello!!");
        let password = dictionary_password(args.dict);
        println!(
            "-> Here is the random password generated with dictionary: {}",
            password
        );
        println!("Enjoy!!");
    } else {
        println!("Hello!!");
        let password = random_password();
        println!("-> Here is the random password: {}", password);
        println!("Enjoy!!");
    }

    Ok(())
}

//=====================================BIBLIOGRAPHY==================================================
//source: https://www.youtube.com/watch?v=AxPDP9Dj08U
//and: https://docs.rs/fastrand/1.3.0/fastrand/
//source: https://stackoverflow.com/questions/15619320/how-can-i-access-command-line-parameters-in-rust
//source: https://docs.rs/regex/latest/regex/struct.Regex.html#method.find_iter
//source: https://www.youtube.com/watch?v=B_UZu-jBYgw
//source: https://docs.rs/lazy_static/latest/lazy_static/
//====================================================================================================
