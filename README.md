# EZWordle
A simple Wordle helper bot.  Both backend and frontend are written in Rust.

# Instructions
Download the repository and run `cargo build --release` in any subdirectory.
You will need an Internet connection to download `indicatif` dependency (for CLI progress bar).
Run `./target/release/wordler.* ./src/words/words.txt` from the root directory to play.

# What's the best starting word?
This is a good question, and one that I don't claim to know the answer to.
However, I searched the list of possible answers (as found in Wordle source code) and as of 22 Mar 2022 I believe the best starting word is `raise`, which yields the player an expected `4.075 bits` of information.

# Bugs
Words with repeated letters seem to mess it up sometimes and I'm working on correcting that now.