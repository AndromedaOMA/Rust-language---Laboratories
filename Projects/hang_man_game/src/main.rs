use rand::Rng;
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use std::{fs, io};

#[derive(Debug)]
struct Guess {
    word: String,
    category: String,
    no_of_guesses: u32,
    maximum_no_of_guesses: u32,
    word_guessed: bool,
    display_word: String,
    content_category: String,
}
impl Guess {
    fn create(
        category: &String,
        word: String,
        display_word: String,
        maxi: u32,
        content_category: String,
    ) -> Option<Guess> {
        if word.len() != 0 {
            return Some(Guess {
                word,
                category: category.to_string(),
                no_of_guesses: 0,
                maximum_no_of_guesses: maxi,
                word_guessed: false,
                display_word,
                content_category,
            });
        } else {
            return None;
        }
    }
    fn get_word(&self) -> &String {
        &self.word
    }
    fn get_category(&self) -> String {
        self.category.clone()
    }
    fn get_no_of_guesses(&self) -> u32 {
        self.no_of_guesses
    }
    fn get_maximum_no_of_guesses(&self) -> u32 {
        self.maximum_no_of_guesses
    }
    fn get_word_guessed(&self) -> bool {
        self.word_guessed
    }
    fn get_display_word(&self) -> &String {
        &self.display_word
    }
    fn get_content_category(&self) -> &String {
        &self.content_category
    }
    fn display_update_dw(&mut self, guessed_letters: HashSet<char>) {
        self.display_word = self
            .word
            .chars()
            .map(|c| if guessed_letters.contains(&c) { c } else { '_' })
            .collect();
        println!("This is your progress: {}", self.display_word);
    }
    fn cont_category(content: &String) -> String {
        let content_category = content
            .lines()
            .map(|line| {
                let pattern = Regex::new(r"[ ,]+").unwrap();
                let words: Vec<&str> = pattern.split(line).collect();
                words[0].to_string()
            })
            .collect::<Vec<String>>()
            .join(", ");
        content_category
    }
}

fn main() {
    let path = Path::new("words.txt");
    if path.exists() {
        println!("File exists!");
    } else {
        println!("File not found!");
    }

    let content = fs::read_to_string(Path::new("words.txt")).expect("Unable to read file");
    // println!("The content of the file is: {}", content);
    let content_category = content
        .lines()
        .map(|line| {
            let pattern = Regex::new(r"[ :,]+").unwrap();
            let words: Vec<&str> = pattern.split(line).collect();
            words[0].to_string()
        })
        .collect::<Vec<String>>()
        .join(", ");
    println!(
        "Enter the category one of these categories: {:?}",
        content_category
    );

    let mut category = String::new();
    io::stdin()
        .read_line(&mut category)
        .expect("Failed to read line");
    category = category.trim().to_string();

    let mut word = String::new();
    let mut letters: HashSet<char> = HashSet::new();

    if content.contains(&category) {
        for line in content.lines() {
            if line.contains(&category) {
                let pattern = Regex::new(r"[ ,]+").unwrap();
                let words: Vec<&str> = pattern.split(line).collect();
                // println!("The words are: {:?}", words);
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(1..=words.len());
                word = words[index].to_string();
                // println!("The word is: {}", word);

                letters = word.chars().map(|c| c).collect();
                break;
            }
        }
    } else {
        println!("The {} category is not in the file!", category);
        return;
    }

    let mut display_word = String::new();
    for _ in 0..word.len() {
        display_word.push('_');
    }

    let mut word = Guess::create(
        &category,
        word,
        display_word,
        letters.len() as u32,
        Guess::cont_category(&content),
    );
    // println!("The word is: {:?}", word);

    println!(
        "Let's play hangman! Guess a word from --> {} <-- category!",
        word.as_mut()
            .expect("Can't get the category")
            .get_category()
    );
    println!(
        "You have only {} guesses!  Good luck!",
        word.as_mut().unwrap().maximum_no_of_guesses
    );

    // while &word.no_of_guesses < 6 && !&word.word_guessed {
    let mut guessed_letters: HashSet<char> = HashSet::new();
    while let Some(ref mut word) = word {
        if word.no_of_guesses > word.maximum_no_of_guesses {
            println!("You lost! The word was: {}", word.word);
            break;
        } else {
            let mut guess = String::new();
            println!("Enter your guess: ");
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            guess = guess.trim().to_string();

            if guess.len() == 1 {
                if word.word.contains(&guess) {
                    println!("You guessed a letter!");
                    word.no_of_guesses += 1;
                    guessed_letters.insert(guess.chars().next().unwrap());
                } else {
                    println!("You didn't guess a letter!");
                    word.no_of_guesses += 1;
                }

                word.display_update_dw(guessed_letters.clone());

                if word.word == word.display_word {
                    println!("Congrats! You guessed the word!");
                    return;
                }
            } else {
                if word.word == guess {
                    println!("You guessed the word!");
                    return;
                } else {
                    println!("You didn't guess the word!");
                    word.no_of_guesses += 1;
                }
            }
        }
    }
}
