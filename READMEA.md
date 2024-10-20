# **Wordle Solver**

## Task Description

The program is solver for the online [Worlde](https://www.nytimes.com/games/wordle/index.html) solver, based on [the theory of information](https://www.youtube.com/watch?v=v68zYyaEmEA).

### Functionalities
The program provides the following functionalities:
* generates a list of top picks and their expected informational output
* reads the word the user tried and the game's response
* adapts the list of words
* continues until the game is won (the word is guessed) or lost (6 rounds have gone by)

### Input
The program expects the following input:
* the tried word: 5-letter english word in lowercase
* the solution: 5-letter code in lowercase where:
    * `w` is white, the letter is not in the word
    * `y` is yellow, the letter is in the word, but not in this place
    * `g` is green, the letter is in the right place 

## Usage
To use the program, run this command:
```Bash
vlad@laptop:~Wordle$ cargo run
```

## Implementation Information
The code is spread trough five Rust source files to make reading them individually easier. The functions are divided as follows:
* `src/main.rs`: the main function of the program
* `src/file.rs`: opening a file and reading the initial words
* `src/info.rs`: functions that compute the bits of a word, update them and pick the words with the most bits
* `src/matcher.rs`: functions that update the possible words based on the last tried word
* `src/read.rs`: functions that read the data from the user

## Resources / Bibliography:
* [Solving Wordle using information theory](https://www.youtube.com/watch?v=v68zYyaEmEA)
