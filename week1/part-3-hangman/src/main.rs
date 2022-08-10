// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;

use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn print_words_so_far(words_so_far: &Vec<char>) {
    print!("The word so far is ");
    for i in words_so_far.iter() {
        print!("{}", *i);
    }
    println!()
}

fn print_letters(letters: &Vec<char>) {
    print!("You have guessed the following letters: ");
    for i in letters.iter() {
        print!("{}", *i);
    }
    println!()
}

fn print_info(word_so_far: &Vec<char>, letters: &Vec<char>, guesses_left: u32) {
    println!();
    print_words_so_far(&word_so_far);
    print_letters(&letters);
    println!("You have {guesses_left} guesses left");
    print!("Please guess a letter: ");
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    println!("{}", secret_word);
    print!("Welcome to CS110L Hangman!");
    let mut secret_word_chars_mut = secret_word_chars;
    let mut words_so_far = Vec::new();
    let mut letters = Vec::new();
    let mut guesses_left = NUM_INCORRECT_GUESSES;
    let mut correct_cnt = 0;
    let word_len = secret_word.len();

    for _i in 0..word_len {
        words_so_far.push('-');
    }
    while guesses_left > 0 && correct_cnt < word_len{
        print_info(&words_so_far, &letters, guesses_left);
        let mut guess= String::new();

        io::stdout().flush().expect("Error flushing stdout");
        io::stdin().read_line(&mut guess).expect("Error reading a line");
        let guess_char: Vec<char> = guess.chars().collect();
        // println!("get input {}", guess_char[0]);
        let mut flag = false;

        for i in 0..word_len {
            let c = secret_word_chars_mut[i];
            if c == guess_char[0] {
                letters.push(c);
                secret_word_chars_mut[i] = '@';
                words_so_far[i] = c;
                correct_cnt += 1;
                flag = true;

                break;
            }
        }
        if !flag {
            println!("Sorry, the letter is not in the word");
            guesses_left -= 1;
        }
    }

    if guesses_left == 0 {
        println!("\nSorry, you ran out of guesses!");
    } else {
        println!("\nCongratulations you guess the corrected word: {}!", secret_word);
    }
}
