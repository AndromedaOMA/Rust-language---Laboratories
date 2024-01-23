use std::fs::OpenOptions;
use std::io;
// use std::io::read_to_string;
use std::io::{Read, Write};

use std::fs::File;
// use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct Data {
    title: String,
    description: String,
    completed: bool,
}
impl Data {
    fn new(title: String, description: String) -> Data {
        Data {
            title,
            description,
            completed: false,
        }
    }
}

struct Todo {
    title_list: String,
    data: Vec<Data>,
}
impl Todo {
    fn new(title_list: String) -> Todo {
        Todo {
            title_list,
            data: Vec::new(),
        }
    }
    fn check_if_file_exists(&self) -> Option<bool> {
        let path = Path::new(&self.title_list);
        if path.exists() {
            Some(true)
        } else {
            None
        }
    }
    fn create_file(&mut self) {
        let mut file = File::create(&self.title_list).expect("Unable to create file");
        println!(
            "Please enter some stuff to do: <title1> <description1>, <title2> <description2>, ..."
        );
        let mut stuff_to_do: String = String::new();
        io::stdin()
            .read_line(&mut stuff_to_do)
            .expect("Failed to read line");
        let stuff_to_do: Vec<&str> = stuff_to_do.split(", ").collect();
        for stuff in stuff_to_do {
            let stuff: Vec<&str> = stuff.split(" ").collect();
            let title: String = stuff[0].to_string();
            let description: String = stuff[1].to_string();
            let completed: bool = false;
            self.data
                .push(Data::new(title.clone(), description.clone()));

            writeln!(
                file,
                "Title: {}\nDescription: {}\nCompleted: {}\n",
                title, description, completed
            )
            .expect("Unable to write data to file");
        }
    }
    fn populate_data(&mut self) {}
    fn add_method(&mut self) {
        let path = Path::new(&self.title_list);
        let mut file: File = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&path)
            .expect("Unable to open or create file");

        println!(
            "Please enter some stuff to do: <title1> <description1>, <title2> <description2>, ..."
        );
        let mut stuff_to_do: String = String::new();
        io::stdin()
            .read_line(&mut stuff_to_do)
            .expect("Failed to read line");

        let stuff_to_do: Vec<&str> = stuff_to_do.split(", ").collect();
        for stuff in stuff_to_do {
            let stuff: Vec<&str> = stuff.split(" ").collect();
            let title: String = stuff[0].to_string();
            let description: String = stuff[1].to_string();
            let completed: bool = false;
            self.data
                .push(Data::new(title.clone(), description.clone()));

            writeln!(
                file,
                "Title: {}\nDescription: {}\nCompleted: {}\n",
                title, description, completed
            )
            .expect("Unable to write data to file");
        }
    }

    fn remove_method(&mut self) {
        self.show_method();
        println!(
            "Please enter the titles of the stuff you want to remove: <title1>, <title2>, ..."
        );
        let mut title_list: String = String::new();
        io::stdin()
            .read_line(&mut title_list)
            .expect("Failed to read line");
        title_list = title_list.trim().to_string();
        let title_list: Vec<&str> = title_list.split(", ").collect();
        for title in title_list {
            for i in self.data.iter() {
                if i.title == title {
                    //TODO: Remove from data (struct)
                    // self.data.remove(i);

                    let path = Path::new(&self.title_list);
                    let mut file: File = File::open(&path).expect("Unable to open file");

                    let mut new_content: String = String::new();
                    file.read_to_string(&mut new_content).unwrap();
                    let new_content: String = new_content.replace(
                        &format!(
                            "Title: {}\nDescription: {}\nCompleted: {}\n",
                            i.title, i.description, i.completed
                        ),
                        "",
                    );

                    let mut file = File::create(&self.title_list).expect("Unable to create file");
                    file.write_all(new_content.as_bytes())
                        .expect("Unable to write data to file");
                }
            }
        }
    }

    fn mark_method(&mut self) {
        self.show_method();
        println!("Please enter the titles of the stuff you want to mark as DONE: <title1>, <title2>, ...");
        let mut title_list: String = String::new();
        io::stdin()
            .read_line(&mut title_list)
            .expect("Failed to read line");

        title_list = title_list.trim().to_string();
        let title_list: Vec<&str> = title_list.split(", ").collect();
        for title in title_list {
            for i in &mut self.data {
                if i.title == title {
                    i.completed = true;
                }
            }
        }
    }
    fn show_method(&self) {
        let path = Path::new(&self.title_list);
        let display = path.display();

        let mut file: File = File::open(&path).expect("Unable to open file");

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}", display, content),
        }
    }
    fn exit_method(&self) {
        println!("Bye bye!");
        std::process::exit(0);
    }
    fn help_method(&self) {
        println!("Here are the commands you can use: <add> / <remove> / <mark> / <show> / <exit> / <help>");
    }
    fn do_something(&mut self) {
        println!("What do you want to do? <add> / <remove> / <mark> / <show> / <exit> / <help>");
        let mut action: String = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
        match action.trim() {
            "add" => self.add_method(),
            "remove" => self.remove_method(),
            "mark" => self.mark_method(),
            "show" => self.show_method(),
            "exit" => self.exit_method(),
            "help" => self.help_method(),
            &_ => println!("Command not found"),
        }
    }
}

fn main() {
    let mut list_username: String = String::new();

    println!("Enter the title of the list you want to examinate/create: ");
    io::stdin()
        .read_line(&mut list_username)
        .expect("Failed to read line");

    let mut list: Todo = Todo::new(list_username.trim().to_string());
    if list.check_if_file_exists() == None {
        println!("File does not exist... But we create one for you! <3");
        list.create_file();
        loop {
            list.do_something();
        }
    } else {
        println!("File exists! <3");
        list.populate_data();
        loop {
            list.do_something();
        }
    }
}
