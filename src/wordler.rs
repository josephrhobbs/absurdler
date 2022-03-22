// crate::wordler

use std::io;
use std::io::Write;

use util::words;

fn main() {
    println!("\nInitializing...\n");
    let mut words: Vec<String> = words::get_words();

    let mut top_guess: String = String::from("raise");
    let mut combination: String;

    for i in 0..6 {
        // Guess the word in words with the highest entropy
        if i != 0 {
            let (guess, entropy) = words::guess(words.clone(), true);
            top_guess = guess;
            if top_guess == String::new() {
                println!("Could not find a matching word");
                break;
            }
            println!("I guess {} with entropy {}\n", top_guess.to_string().to_uppercase(), entropy);
        } else {
            println!("I guess {}\n", top_guess.to_string().to_uppercase());
        }

        // Get human's guess
        print!("What do you guess? >> ");
        io::stdout().flush().unwrap();

        let mut human_guess: String = String::new();
        io::stdin().read_line(&mut human_guess).expect("Could not read input");
        human_guess = human_guess.to_lowercase();

        // Get Wordle's response from the user
        print!("How did you do? >> ");
        io::stdout().flush().unwrap();
        combination = String::new();
        io::stdin().read_line(&mut combination).expect("Could not read line from stdin");
        println!("\n");

        combination.pop();  combination.pop();

        if combination == "....." {
            println!("Success!");
            break;
        }

        words = words::get_matches(human_guess.to_string().clone(), words.clone(), combination.clone());
    }
    println!("\nThanks for playing!\n");
}