# Absurdler
A simple Absurdle helper bot.  Both backend and frontend are written in Rust.

# About Absurdle
Absurdle was developed by QNTM (whom is not affiliated with and does not endorse this software in any way).  It is a variant of the new viral game Wordle.

Absurdle is identical to Wordle except for two subtle changes.  The first (and simpler one) is that there is no word limit.  This accomodates the increased difficulty of the game due to the second change.

The second (and much more interesting change) is in how the game is "scored".  The computer does not decide on a secret word before the game begins; instead, it decides on the secret word as the player plays, intentionally prolonging the game for as many turns as possible.

You can play Absurdle here on QNTM's website: https://qntm.org/files/absurdle/absurdle.html

# Disclaimer
As mentioned above, QNTM is not affiliated with the developers of this program and does not endorse it in any way.

# Instructions
Downloading Absurdler is done in two steps and can be fully performed in about 5 to 10 minutes.  See below for instructions.

Absurdler installation requires `cargo`.

# Step 1: Downloading the Wordle bot
Run `cargo install absurdler` in any directory.  This will download and compile the `absurdler` binary.

# Step 2: Downloading the allowed word list
Download the allowed word list (as of 27 Mar 2022) from the GitHub repository `hobbsbros/absurdler`.  The word list can be found at `src/words/words.txt`.

You may install this in any directory but Desktop is recommended.

# Playing
From the directory that you installed `words.txt` run `absurdler words.txt`.

If the word list file is named `words.txt` and is in your current directory, you do not need to specify the filename.

Absurdler is designed to work with an initial random guess.  However, you may make your own guess if you so please.

For each subsequent play, Absurdler will make a recommendation, but still requires you to input the actual guess that you make.
After inputting your guess, Absurdler will ask you for feedback.

`x` corresponds to gray/black (letter not in word)
`/` corresponds to yellow (letter in word at wrong position)
`.` corresponds to green (letter in word at correct position)

For example:

`I guess CASTS`
`What do you guess? >> CHEST`
`How did you do? >> xx/..`

# What's the best starting word?
This is a good question, and one that I don't claim to know the answer to.
However, I searched the list of possible answers (as found in Wordle source code) and as of 22 Mar 2022 I believe the best starting word is `raise`, which yields the player an expected `4.075 bits` of information.

# Bugs
Words with repeated letters seem to mess it up sometimes and I'm working on correcting that now.