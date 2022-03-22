// util::words

use std::fs::File;
use std::io::Read;
use super::math;

pub struct Guess {
    pub value: String,
    pub entropy: f64,
}

pub fn get_words(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).expect("Could not find word file");
    let mut words: String = String::new();
    file.read_to_string(&mut words);
    let output: Vec<String> = words.lines().map(String::from).collect();
    output
}

// Checks if a guess and given feedback could match to a word in the wordlist
// Helps to reduce the space of possible words
pub fn check_match(guess: String, combination: String, word: String) -> bool {
    // word refers to a (possible) secret word
    // guess refers to the word that the user has guessed
    let mut bools: Vec<bool> = Vec::new();
    for ((i, letter), truth) in guess.chars().enumerate().zip(combination.chars()) {
        let right_place: bool = word.as_bytes()[i] as char == letter && truth == '.';
        let wrong_place: bool = word.contains(letter) && truth == '/';
        let not_in_word: bool = !word.contains(letter) && truth == 'X';
        let one_true: bool = right_place || wrong_place || not_in_word;
        bools.push(one_true);
    }
    if !bools.contains(&false) {
        return true;
    }
    return false;
}

// Helper function: get all words that match a combination in the wordlist
pub fn get_matches(guess: String, wordlist: Vec<String>, combination: String) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    for word in wordlist.iter() {
        let chk: bool = check_match(guess.clone(), combination.clone(), word.to_string().clone());
        if chk {
            words.push(word.to_string());
        }
    }
    words
}

// Helper function: count the number of words that match a combination in the wordlist
pub fn count_matches(guess: String, wordlist: Vec<String>, combination: String) -> u64 {
    let mut count: u64 = 0;
    for word in wordlist.iter() {
        let chk: bool = check_match(guess.clone(), combination.clone(), word.to_string().clone());
        if chk {
            count += 1;
        }
    }
    count
}

// Gets a list of all possible output combinations
pub fn get_all_combinations(len: u8) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    
    // BASE CASE
    if len == 1 {
        return vec!["X", "/", "."].into_iter().map(String::from).collect::<Vec<String>>();
    }

    // RECURSIVE CASE
    for sub in get_all_combinations(len - 1) {
        result.push(sub.clone() + "X");
        result.push(sub.clone() + "/");
        result.push(sub.clone() + ".");
    }
    result
}

pub fn guess_entropy(guess: String, wordlist: Vec<String>) -> f64 {
    let combos: Vec<String> = get_all_combinations(5);
    let search_space_size: f64 = wordlist.len() as f64;
    let mut entropy: f64 = 0.0;
    let mut possible_count: u64 = 0;
    for combo in combos {
        let possible_words: u64 = count_matches(guess.clone(), wordlist.clone(), combo.to_string().clone());
        if possible_words != 0 {
            possible_count += 1;
            entropy += (search_space_size/(possible_words as f64)).ln();
        }
    }

    entropy *= 1.0/(possible_count as f64);
    entropy
}
