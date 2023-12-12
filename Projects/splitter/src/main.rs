use regex::Regex;
use std::io::{Error, ErrorKind};
use std::{fs, io};
// use std::iter::repeat_with;

fn main() -> Result<(), io::Error> {
    let custom_error = Error::new(ErrorKind::Other, "Wrong command!");

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

    if args.len() == 5
        && (args[0] != "./splitter" || args[1] != "split" || args[2] != "a.zip" || args[3] != "-s")
        || args.len() == 3
            && (args[0] != "./splitter" || args[1] != "unsplit" || args[2] != "a.zip")
        || args.len() != 5 && args.len() != 3
    {
        println!("You have to chose from these two commands:");
        println!("   1. ./splitter split a.zip -s <size> -> to generate multiple files from a zip file of dimension <size>");
        println!(
            "   2. ./splitter unsplit a.zip         -> to recreate the original zip file from the multiple files"
        );
        return Err(custom_error);
    }
    //====================================================================================================

    //========================================COMMAND SELECTION===========================================
    if args[1] == "split" {
        // println!("split");
        let fd=&args[2];
        let mut file = fs::File::open(fd)?;
        let size:usize;
        match args[4] {
            Some(s) if s=='1K' => size=1024;
            Some(s) if s=='1M' => size=1024*1024;
            Some(s) if s=='1G' => size=1024*1024*1024;
            _ => => size=1; //default size is 1 byte
        }
        
        
    } else if args[1] == "unsplit" {
        println!("unsplit");
    }

    Ok(())
}
