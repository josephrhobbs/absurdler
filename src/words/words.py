# words.py
#
# Parses raw words list from Wordle source code

raw = open("raw.txt", "r").read()
words = open("words.txt", "w+")

rawlist = raw.split(",")

wordlist = [w.strip("[]\" ") for w in rawlist]

print(len(wordlist))

for w in wordlist:
    words.write(w)
    words.write("\n")

words.close()