// crate::main

use std::io;
use std::io::Write;

use indicatif::ProgressBar;

use util::words;

fn main() {
    println!("\nInitializing...\n");
    let mut words: Vec<String> = words::get_words();

    let mut top_guess: String = String::from("raise");
    let mut guess_entropy: f64;

    'inner: for i in 0..6 {
        // Guess the word in words with the highest entropy
        let num_of_words = words.len();
        if i != 0 {
            println!("Searching {} words for the optimal guess...", num_of_words);
            let mut guesses: Vec<words::Guess> = Vec::new();
            let bar = ProgressBar::new(num_of_words as u64);
            for word in words.clone().into_iter() {
                guesses.push(words::Guess {value: word.clone(), entropy: words::compute_entropy(word.clone(), words.clone())});
                bar.inc(1);
            }
            bar.finish();
            println!("Done searching!\n");
            let sorted_guesses = &mut guesses[..];

            if sorted_guesses.len() == 0 {
                println!("Could not find a matching word");
                break 'inner;
            }

            sorted_guesses.sort_by(|x, y| y.entropy.partial_cmp(&x.entropy).unwrap());

            top_guess = sorted_guesses[0].value.clone();
            guess_entropy = sorted_guesses[0].entropy;

            println!("I guess {} with entropy {}\n", top_guess.to_string().to_uppercase(), guess_entropy);
        } else {
            println!("I guess {}\n", top_guess.to_string().to_uppercase());
        }

        // Get Wordle's response from the user
        let mut combo: String = String::new();
        print!("How did I do? >> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut combo).expect("Could not read line from stdin");

        combo.pop();  combo.pop();

        if combo == "....." {
            println!("Success!");
            break 'inner;
        }

        words = words::get_matches(top_guess.to_string().clone(), words.clone(), combo.clone());
    }
    println!("\nThanks for playing!\n");
}