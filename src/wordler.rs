// crate::wordler

use std::io;
use std::io::Write;

use util::words;

fn main() {
    println!("\nInitializing...\n");
    let mut words: Vec<String> = words::get_words();
    let all_words: Vec<String> = words.clone();

    let mut top_guess;
    let mut combination: String;

    let mut i = 0;

    loop {
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
            println!("Input your first guess below.  You may choose this voluntarily or randomly.\n");
        }

        let mut human_guess: String = String::new();
        while !all_words.iter().any(|x| *x == human_guess.clone()) {
            human_guess = String::new();

            // Get human's guess
            print!("What do you guess? >> ");
            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut human_guess).expect("Could not read input");
            human_guess.truncate(5);
            human_guess = human_guess.to_lowercase();
        }

        // Get Wordle's response from the user
        print!("How did you do? >> ");
        io::stdout().flush().unwrap();
        combination = String::new();
        io::stdin().read_line(&mut combination).expect("Could not read line from stdin");
        println!("\n");

        combination.truncate(5);

        if combination == "....." {
            println!("Success!");
            break;
        }

        words = words::get_matches(human_guess.to_string().clone(), words.clone(), combination.clone());

        i += 1;
    }
    println!("\nThanks for playing!\n");
}