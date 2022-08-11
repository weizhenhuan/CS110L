use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut lines = Vec::new();

    for line in BufReader::new(file).lines() {
        let line_str = line?;
        lines.push(line_str);
    }

    Ok(lines)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let lines = read_file_lines(filename).expect("Error reading the file!");

    let count_lines = lines.len();
    let mut count_ch = 0;
    let mut count_words = 0;

    for line in lines.iter() {
        for word in line.split(" ") {
            count_words += 1;
            count_ch += word.len();
        }
    }

    println!("Count line: {count_lines}");
    println!("Count words: {count_words}");
    println!("Count characters: {count_ch}");

}
