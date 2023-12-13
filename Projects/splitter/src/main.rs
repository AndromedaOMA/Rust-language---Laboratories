use glob::glob;
use regex::Regex;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufWriter, Error, ErrorKind, Read, Write};
// use std::iter::repeat_with;

fn main() -> Result<(), io::Error> {
    //============================================ERRORS==================================================
    let custom_error = Error::new(ErrorKind::Other, "Wrong command!");
    let list_of_split_paths_error = Error::new(
        ErrorKind::Other,
        "There are no files to unsplit! You have to split a txt file first!",
    );
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

    if (args.len() == 5 || args.len() == 4)
        && (args[0] != "./splitter" || args[1] != "split" || args[2] != "a.txt" || args[3] != "-s")
        || args.len() == 3
            && (args[0] != "./splitter" || args[1] != "unsplit" || args[2] != "a.txt")
        || args.len() != 5 && args.len() != 4 && args.len() != 3
    {
        println!("You have to chose from these two commands:");
        println!("   1. ./splitter split a.txt -s 1K|1M|1G -> to generate multiple files from a txt file of dimension 1K, 1M or 1G");
        println!(
            "   2. ./splitter unsplit a.txt           -> to recreate the original txt file from the multiple files"
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

        let mut size: usize = 1; //default size is 1 byte

        //NOT WORKING...
        // match args[4] {
        //     Some("1K") => size = 1024,
        //     Some("1M") => size = 1024 * 1024,
        //     Some("1G") => size = 1024 * 1024 * 1024,
        //     _ => size = 1, // Default size is 1 byte
        // }

        if args[4] == "1K" {//todo pot fi parametrii diferiti
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

        for (i, _) in bytes.iter().enumerate() {
            //cargo clippy
            // let name_file = format!("a.txt.part{}.split", i);
            match i {
                0..=9 => {
                    let name_file = format!("a.txt.part000{}.split", i);
                    let mut f = File::create(name_file)?;
                    f.write_all(&bytes[i].to_be_bytes())?;
                }
                10..=99 => {
                    let name_file = format!("a.txt.part00{}.split", i);
                    let mut f = File::create(name_file)?;
                    f.write_all(&bytes[i].to_be_bytes())?;
                }
                100..=999 => {
                    let name_file = format!("a.txt.part0{}.split", i);
                    let mut f = File::create(name_file)?;
                    f.write_all(&bytes[i].to_be_bytes())?;
                }
                _ => {
                    let name_file = format!("a.txt.part{}.split", i);
                    let mut f = File::create(name_file)?;
                    f.write_all(&bytes[i].to_be_bytes())?;
                }
            }
        }

        println!("Done! -> {} files created", bytes.len());
    } else if args[1] == "unsplit" {
        //============================================UNSPLIT=================================================
        // println!("unsplit");

        //============================================GLOB -> populate vec!===================================
        let mut list_of_split_paths = vec![];
        for entry in glob("../splitter/*.split").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => list_of_split_paths.push(path.display().to_string()),
                Err(e) => println!("{:?}", e),
            }
        }
        //test
        // println!("list_of_paths: {:?}", list_of_split_paths);
        //====================================================================================================

        if list_of_split_paths.is_empty() {
            return Err(list_of_split_paths_error);
        } else {
            list_of_split_paths.sort();
            // let mut f = File::create(args[2])?;
            let file = OpenOptions::new()
                .append(true)
                .write(true)
                .open(args[2].to_string())?;
            // let mut file = BufWriter::<dyn Write>::new(f);
            let mut file = BufWriter::new(file);
            // let mut f = file?;

            for path in list_of_split_paths.iter() {
                //===========================================NOT WORKING...
                // let mut fd = File::open(path).expect("Can't open file");
                // let size = fd.metadata()?.len() as usize;
                // let mut bytes = vec![0u8; size];
                // fd.read_exact(&mut bytes)?;
                // f.write_all(&bytes)?;
                //===========================================NOT WORKING...
                let data = fs::read_to_string(path)?;
                file.write_all(&data.as_bytes())?;

                fs::remove_file(path)?;
            }
            // fs::remove_file(args[2])?;
        } //  SOME IDEAS  -> create a string that contain all the data then move it to the a.txt file
    }

    Ok(())
}

//=====================================BIBLIOGRAPHY==================================================
//source: https://users.rust-lang.org/t/read-variable-number-of-bytes-from-a-file/89179
//source: https://www.youtube.com/watch?v=nQq raiMymcU
//source: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
//source: https://stackoverflow.com/questions/26076005/how-can-i-list-files-of-a-directory-in-rust
//source: https://crates.io/crates/glob
//source: https://doc.rust-lang.org/std/fs/fn.remove_file.html#:~:text=Function%20std%3A%3Afs%3A%3Aremove_file&text=Removes%20a%20file%20from%20the,descriptors%20may%20prevent%20immediate%20removal).
//source: https://doc.rust-lang.org/std/fs/struct.Metadata.html
//source: https://redandgreen.co.uk/buffers-in-rust/rust-programming/
//source: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.append
//source: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create
//source: https://www.youtube.com/watch?v=CE_7vZKMs2Y
//source:
//source:
//====================================================================================================


//TODO :1. SA VERIFIC DACA NU SUN FISIERE CORUPTE -> PUN IN HEADER-UL FISIERULUI UN HASH
//TODO :2. SA VERIFIC DACA AM TOATE FIZIERELE (DACA U S-A STERSC VREUNUL)