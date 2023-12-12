use regex::Regex;
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Read, Write};
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

    if (args.len() == 5 || args.len() == 4)
        && (args[0] != "./splitter" || args[1] != "split" || args[2] != "a.zip" || args[3] != "-s")
        || args.len() == 3
            && (args[0] != "./splitter" || args[1] != "unsplit" || args[2] != "a.zip")
        || args.len() != 5 && args.len() != 4 && args.len() != 3
    {
        println!("You have to chose from these two commands:");
        println!("   1. ./splitter split a.zip -s 1K|1M|1G -> to generate multiple files from a zip file of dimension 1K, 1M or 1G");
        println!(
            "   2. ./splitter unsplit a.zip           -> to recreate the original zip file from the multiple files"
        );
        return Err(custom_error);
    }
    //====================================================================================================

    //========================================COMMAND SELECTION===========================================
    if args[1] == "split" {
        //============================================SPLIT===================================================
        // println!("split");

        let path = format!("./src/{}", args[2]);
        let mut fd = File::open(path).expect("Can't open file");

        let mut size: usize = 1;

        //NOT WORKING...
        // match args[4] {
        //     Some("1K") => size = 1024,
        //     Some("1M") => size = 1024 * 1024,
        //     Some("1G") => size = 1024 * 1024 * 1024,
        //     _ => size = 1, // Default size is 1 byte
        // }

        if args[4] == "1K" {
            size = 1024;
        } else if args[4] == "1M" {
            size = 1024 * 1024;
        } else if args[4] == "1G" {
            size = 1024 * 1024 * 1024;
        }

        //test
        // println!("size: {}", size);

        let mut bytes = vec![0u8; size];
        fd.read_exact(&mut bytes)?;

        //test
        // println!("content: {:?}", &bytes);

        for i in 0..bytes.len() {
            let name_file = format!("a.zip.part{}.split", i);
            let mut f = File::create(name_file)?;
            f.write_all(&bytes[i].to_be_bytes())?;
        }
        println!("Done! -> {} files created", bytes.len());
    } else if args[1] == "unsplit" {
        //============================================UNSPLIT=================================================
        // println!("unsplit");
        
    }

    Ok(())
}

//=====================================BIBLIOGRAPHY==================================================
//source: https://users.rust-lang.org/t/read-variable-number-of-bytes-from-a-file/89179
//source: https://www.youtube.com/watch?v=nQqraiMymcU
//source: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
//====================================================================================================
