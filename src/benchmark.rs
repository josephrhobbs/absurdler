// crate::benchmark

use std::env;

use util::words;

use indicatif::ProgressBar;

fn get_combination(colored_word: String, correct_word: String) -> String {
    let mut combo: String = String::new();
    for (i, letter) in colored_word.chars().enumerate() {
        if correct_word.as_bytes()[i] as char == letter {
            combo.push('.');
        } else if correct_word.contains(letter) {
            combo.push('/');
        } else {
            combo.push('x');
        }
    }
    combo.clone()
}

fn play_wordle(correct_word: String, wordlist: Vec<String>) -> u8 {
    let mut guess = String::from("raise");
    let mut words = wordlist.clone();
    let mut score: u8 = 0;
    for i in 0..6 {
        if i != 0 {
            let (next_guess, _entropy) = words::guess(words.clone(), false);
            score += 1;
            guess = next_guess
        }
        let combo: String = get_combination(guess.clone(), correct_word.clone());
        if combo == "....." {
            return score;
        }
        words = words::get_matches(guess.clone(), words.clone(), combo.clone());
    }
    return 6;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n_str: String = args[2].clone();
    let n: u64 = n_str.parse::<u64>().expect("Could not parse argument");
    let wordlist = words::get_words();
    let bar = ProgressBar::new(n);
    let mut score: u8 = 0;
    let mut tries: u8 = 0;
    for secret_word in wordlist[..(n as usize)].into_iter() {
        score += play_wordle(secret_word.to_string(), wordlist.clone());
        tries += 1;
        bar.inc(1);
    }
    bar.finish();
    println!("Completed {} Wordles with an average score of {}\n", wordlist.len(), (score as f64/tries as f64));
}