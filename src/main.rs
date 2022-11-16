/*
Given letters, find words that would match those letters

algorithm:
file <- open file

vector <- read every newline as an entry
letters <- read stdin

iterate over vector as i:
    if any characters in i are not in letters:
        remove word from vector

words <- return words where all letters are in `letters`
stdout <- print words from greatest to fewest

() <- close file
*/

use std::fs;
//use std::env;

fn main() {
    let file = fs::read_to_string("src/text.txt")
        .expect(String::from("Tried reading file, failed!!").as_str());
    let mut words: Vec<&str> = vec![];

    for word in file.split("\n") {
        words.push(word);
    }

    println!("Hello, world! {}", file);
}