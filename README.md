# EZWordle
A simple Wordle helper bot.  Both backend and frontend are written in Rust.

# Instructions
Downloading EZWordle is done in two steps and can be fully performed in about 5 to 10 minutes.  See below for instructions.

EZWordle installation requires `cargo`.

# Step 1: Downloading the Wordle bot
Run `cargo install ezwordle` in any directory.  This will download and compile the `ezwordle` binary.

# Step 2: Downloading the allowed word list
Download the allowed word list (as of 27 Mar 2022) from the GitHub repository `hobbsbros/ezwordle`.  The word list can be found at `src/words/words.txt`.

You may install this in any directory but Desktop is recommended.

# Playing
From the directory that you installed `words.txt` run `ezwordle words.txt`.

EZWordle will make a recommendation, but still requires you to input the actual guess that you make.
After inputting your guess, EZWordle will ask you for feedback.

`x` corresponds to gray/black (letter not in word)
`/` corresponds to yellow (letter in word at wrong position)
`.` corresponds to green (letter in word at correct position)

For example, if the word of the day were `BEAST`:

`I guess CASTS`
`What do you guess? >> CHEST`
`How did you do? >> xx/..`

# What's the best starting word?
This is a good question, and one that I don't claim to know the answer to.
However, I searched the list of possible answers (as found in Wordle source code) and as of 22 Mar 2022 I believe the best starting word is `raise`, which yields the player an expected `4.075 bits` of information.

# Bugs
Words with repeated letters seem to mess it up sometimes and I'm working on correcting that now.