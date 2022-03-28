// util::words

use std::fs::File;
use std::io::Read;

use indicatif::ProgressBar;

pub struct Guess {
    pub value: String,
    pub entropy: f64,
}

pub fn get_words() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    let mut file;
    let mut words: String = String::new();
    let result;
    if args.len() < 2 {
        file = match File::open("words.txt") {
            Ok(f) => f,
            Err(_) => {
                println!("When executing ezwordle, you must specify a raw text file containing Wordle's allowed word list.");
                println!("You may download this file from https://github.com/hobbsbros/ezwordle/blob/main/src/words/words.txt\n");
                println!("\nFor example: $ ezwordle words.txt\n");
                println!("If the file words.txt exists within the directory, you do not need to specify words.txt as an argument.");
                panic!("No word list specified");
            }
        };
        result = file.read_to_string(&mut words);
    } else {
        let filename = args[1].clone();
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => {
                println!("ERROR: EZWordle could not find the file specified.\n");
                panic!("Could not find specified word list file");
            }
        };
        result = file.read_to_string(&mut words);
    }
    match result {
        Ok(_) => {},
        Err(_) => {
            println!("ERROR: EZWordle found the file specified but was unable to read it.\n");
            panic!("Could not read word list file");
        }
    }
    let output: Vec<String> = words.lines().map(String::from).collect();
    output
}

// Checks if a guess and given feedback could match to a word in the wordlist
// Helps to reduce the space of possible words
pub fn check_match(colored_word: String, combination: String, word_to_check: String) -> bool {
    // word_to_check refers to a (possible) secret word
    // colored_word refers to the word that the user has guessed
    for ((i, letter), truth) in colored_word.chars().enumerate().zip(combination.chars()) {
        let letter_to_check: char = word_to_check.as_bytes()[i] as char;
        if truth == '.' {
            if letter_to_check != letter {
                return false;
            }
        } else if truth == '/' {
            if !word_to_check.contains(letter) || letter_to_check == letter {
                return false;
            }
        } else if truth == 'x' {
            if word_to_check.contains(letter) {
                return false;
            }
        }
    }
    return true;
}

// Helper function: get all words that match a combination in the wordlist
pub fn get_matches(colored_word: String, wordlist: Vec<String>, combination: String) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    for word in wordlist.iter() {
        let chk: bool = check_match(colored_word.clone(), combination.clone(), word.to_string().clone());
        if chk {
            words.push(word.to_string());
        }
    }
    words
}

// Gets a list of all possible output combinations
pub fn get_all_combinations(len: u8) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    
    // BASE CASE
    if len == 1 {
        return vec!["x", "/", "."].into_iter().map(String::from).collect::<Vec<String>>();
    }

    // RECURSIVE CASE
    for sub in get_all_combinations(len - 1) {
        result.push(sub.clone() + "x");
        result.push(sub.clone() + "/");
        result.push(sub.clone() + ".");
    }
    result
}

// Computes the contribution to information entropy of a given colored word & combination
pub fn compute_contribution(colored_word: String, wordlist: Vec<String>, combination: String) -> f64 {
    let new_wordlist = get_matches(colored_word.clone(), wordlist.clone(), combination.clone());
    if new_wordlist.len() == 0 {
        return 0.0;
    }
    let p: f64 = (new_wordlist.len() as f64)/(wordlist.len() as f64);
    let mut logp: f64 = 0.0;
    if p > 0.0 {
        logp = p.ln();
    }
    return -p * logp;
}

// Computes the expected amount of information (information entropy) from a given guess
pub fn compute_entropy(colored_word: String, wordlist: Vec<String>) -> f64 {
    let combos: Vec<String> = get_all_combinations(5);
    let mut entropy: f64 = 0.0;
    for combo in combos {
        entropy += compute_contribution(colored_word.clone(), wordlist.clone(), combo.clone());
    }
    entropy
}

// Guesses a word and returns the reduced wordlist
pub fn guess(wordlist: Vec<String>, verbose: bool) -> (String, f64) {
    let num_of_words = wordlist.len();
    let mut guesses: Vec<Guess> = Vec::new();

    let bar = ProgressBar::new(num_of_words as u64);

    if verbose {
        println!("Searching {} words for the optimal guess...", num_of_words);
    }

    for word in wordlist.clone().into_iter() {
        guesses.push(Guess {value: word.clone(), entropy: compute_entropy(word.clone(), wordlist.clone())});
        if verbose {bar.inc(1)}
    }
    if verbose {
        bar.finish();
        println!("Done searching!\n");
    }

    let sorted_guesses = &mut guesses[..];
    if sorted_guesses.len() == 0 {
        return (String::new(), 0.0);
    }
    sorted_guesses.sort_by(|x, y| y.entropy.partial_cmp(&x.entropy).unwrap());

    let top_guess = sorted_guesses[0].value.clone();
    let guess_entropy = sorted_guesses[0].entropy;

    return (top_guess.to_string(), guess_entropy);
}