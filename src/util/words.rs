// util::words

use std::fs::File;
use std::io::Read;

pub fn get_words(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).expect("Could not find word file");
    let mut words: String = String::new();
    file.read_to_string(&mut words);
    let output: Vec<String> = words.lines().map(String::from).collect();
    output
}

// Gets a list of all possible output combinations
// YAY I figured out recursion in Rust :D
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