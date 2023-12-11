use fastrand;
use regex::Regex;
use std::io;
use std::io::{Error, ErrorKind};
// use std::iter::repeat_with;

fn main() -> Result<(), io::Error> {
    let custom_error = Error::new(ErrorKind::Other, "oh no!");

    //============================================LISTS OF CHARS==========================================
    let list_of_alphanumeric: Vec<char> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            .chars()
            .collect();
    let list_of_special_chars: Vec<char> = "!?#@".chars().collect();
    let list_of_uppercase: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    //====================================================================================================

    //============================================INPUT===================================================
    let mut input = String::new();
    println!("Enter a command:");
    let _ = io::stdin().read_line(&mut input);

    let re = Regex::new(r"\S+").unwrap();
    let args: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();

    //test
    /*
    for arg in args.iter() {
        println!("{}", arg);
    }
    */

    if args.len() == 1 && args[0] != "./pass_gen"
        || args.len() > 1
            && (args[0] != "./pass_gen" || args[1] != "--dict" || args[2] != "dict.txt")
    {
        println!("You have to chose from these two commands:");
        println!("   1. ./pass_gen                 -> to generate a random password");
        println!(
            "   2. ./pass_gen --dict dict.txt -> to generate a random password from a dictionary"
        );
        return Err(custom_error);
    }
    //====================================================================================================

    // let password: String = repeat_with(fastrand::alphanumeric).take(random_length).collect();

    //========================================COMMAND SELECTION===========================================
    if args.len() == 1 {
        //============================================RANDOM PASSWORD==========================================
        let random_length = fastrand::usize(12..19);

        let password: String = (0..random_length)
            .map(|current_index| {
                if current_index == 0 {
                    let random_index = fastrand::usize(..list_of_uppercase.len());
                    list_of_uppercase[random_index].to_string()
                } else if current_index == 1 {
                    let random_index = fastrand::usize(..list_of_special_chars.len());
                    list_of_special_chars[random_index].to_string()
                } else {
                    let random_index = fastrand::usize(..list_of_alphanumeric.len());
                    list_of_alphanumeric[random_index].to_string()
                }
            })
            .collect();

        println!("Hello!!");
        println!("-> Here is the random password: {}", password);
        println!("Enjoy!!");
    } else {
        //============================================DICTIONARY PASSWORD======================================
        println!("Hello!!");
    }
    //====================================================================================================

    Ok(())
}

//=====================================BIBLIOGRAPHY==================================================
//source: https://www.youtube.com/watch?v=AxPDP9Dj08U
//and: https://docs.rs/fastrand/1.3.0/fastrand/
//source: https://stackoverflow.com/questions/15619320/how-can-i-access-command-line-parameters-in-rust
//source: https://docs.rs/regex/latest/regex/struct.Regex.html#method.find_iter
//====================================================================================================
