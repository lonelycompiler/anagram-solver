/*
Given an anagram, return all words that are rearrangements of the characters of that word 
*/

use std::fs;
//use std::env;
use std::io;
use std::collections::HashMap;
use std::cmp::PartialEq;
use std::ops;
use std::env;

#[derive(Debug)]
struct Dictionary<'a>(pub HashMap<&'a str, u8>);

impl<'a> ops::Deref for Dictionary<'a> {
    type Target = HashMap<&'a str, u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }

}

impl<'a> PartialEq for Dictionary<'a> {

    // self is the anagram
    // other is the word in words.txt
    fn eq(&self, other: &Self) -> bool {
        for key in other.keys() {
            // compares keys, and their values, irrespective of order
            if !self.contains_key(*key) {
                return false;
            }
        }
        for key in self.keys() {
            if other.contains_key(key) {
                if *other.get(*key).unwrap() > *self.get(*key).unwrap() {
                    return false;
                }
            }
        }
        true
    }
}

fn word_freq(word: &String) -> Dictionary {
    let mut word_dict: HashMap<&str, u8> = HashMap::new();
    for letter in word.split("") {
        if letter == "" {
            continue;
        }
        if !word_dict.contains_key(letter) {
            word_dict.insert(letter, 1);
        }
        else {
            let num_letters = *word_dict.get(&letter).unwrap() + 1;
            word_dict.insert(letter, num_letters);
        }
    }
    Dictionary(word_dict)
}

fn main() {
    // create initial variables (allocating space)
    let mut answers: Vec<&str> = vec![];
    let mut input_anagram = String::new();
    let num_letters: i32;

    // read file -> file
    let file = fs::read_to_string("src/words.txt")
        .expect(String::from("Tried reading file, failed!!").as_str());
    
    // get anagram letters
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            io::stdin().read_line(&mut input_anagram).unwrap();
        },
        2 => {
            input_anagram = String::from(args[1].as_str());
        },
        _ => {
            println!("Incorrect input format!");
        },
    }

    input_anagram = input_anagram.to_lowercase();
    //input_anagram.pop(); //remove the newline at the end
    num_letters = input_anagram.len() as i32; // number of words
    
    // create dictionary for the input anagram
    let letter_freq = word_freq(&input_anagram);

    // loop through words to find possible words that
    // are of length less than/equal to num_letters
    // and all letters in that answer word are in the anagram
    for word in file.split("\r\n") {
        if word.len() as i32 <= num_letters && word.len() as i32 > 2 {
            if letter_freq == word_freq(&String::from(word.to_lowercase())) {
                answers.push(word);
            }
        }
    }

    // get points
    let mut points: HashMap<&str, u32> = HashMap::new();

    for word in answers {
        match word.len() {
            6 => points.insert(word, 2000),
            5 => points.insert(word, 1200),
            4 => points.insert(word, 400),
            3 => points.insert(word, 100),
            _ => points.insert(word, 0),
        };
    }
    let mut points_vec: Vec<(&&str, &u32)> = points.iter().collect();
    points_vec.sort_by(
        |a, b| b.1.cmp(a.1)
    );
    for (word, points) in points_vec {
        println!("{}\t\t{}", *word, *points);
    }
}