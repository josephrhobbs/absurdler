// tests::test0.rs

use util::words;
use std::env;

#[test]
fn combinations() {
    dbg!(words::get_all_combinations(5));
}

#[test]
fn single_match() {
    let word: String = String::from("their");
    let combo: String = String::from("X/X//");
    let guess: String = String::from("crate");
    let check_match: bool = words::check_match(guess.clone(), combo.clone(), word.clone());

    dbg!(&word, &combo, &guess, &check_match);
}

#[test]
fn all_matches() {
    let wordlist = words::get_words("src/words/words.txt");
    let guess: String = String::from("crate");
    let combo: String = String::from("X/X//");
    let words: Vec<String> = words::get_matches(guess.clone(), wordlist.clone(), combo.clone());

    dbg!(&guess, &combo, &words);
}

#[test]
fn entropy() {
    let wordlist = words::get_words("src/words/words.txt");
    let guess: String = String::from("crate");
    let entropy: f64 = words::compute_entropy(guess.clone(), wordlist.clone());

    dbg!(&entropy);
}