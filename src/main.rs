// crate::main

use util::{math, words};

fn main() {
    let words: Vec<String> = words::get_words("words/words.txt");
    let combinations: Vec<String> = words::get_all_combinations(5);
}