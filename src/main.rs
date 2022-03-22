// crate::main


use std::io;

use util::{math, words};

fn main() {
    println!("Initializing...");
    let mut words: Vec<String> = words::get_words("words.txt");
    let combinations: Vec<String> = words::get_all_combinations(5);
    let mut solved: bool = false;

    while !solved {
        // Guess the word in words with the highest entropy
        println!("Searching {} words for the optimal guess...", words.len());
        let mut guesses: Vec<words::Guess> = Vec::new();
        for (i, word) in words.clone().into_iter().enumerate() {
            guesses.push(words::Guess {value: word.clone(), entropy: words::guess_entropy(word.clone(), words.clone())});
            if (i+1)%100 == 0 && i != 0 {
                println!("Searched {} words so far", i+1);
            }
        }
        println!("Done searching!\n");
        
        let sorted_guesses = &mut guesses[..];
        sorted_guesses.sort_by(|x, y| x.entropy.partial_cmp(&y.entropy).unwrap());

        let top_guess = &sorted_guesses[0].value;

        println!("I guess >> {}\n", top_guess);

        // Get Wordle's response from the user
        let mut combo: String = String::new();
        print!("How did I do? >> ");
        io::stdin().read_line(&mut combo).expect("Could not read line from stdin");

        if combo == "....." {
            solved = true;
        }

        words = words::get_matches(top_guess.clone(), words.clone(), combo.clone());
    }
}