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