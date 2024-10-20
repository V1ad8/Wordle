use serde_derive::Deserialize;

mod file;
mod info;
mod matcher;
mod read;

#[derive(Debug, Clone, Deserialize)]
struct Record {
    word: String,
    bits: f32,
    freq: f64,
}

fn main() {
    let mut words = file::open_file();

    // Generate all the possibilities
    let possibilities = matcher::generate_possibilities();

    // Get the initial number of words and print it
    let initial_total: usize = words.len();
    println!("Initial total: {}", initial_total);

    // Iterate over the turns of the game
    for _turn in 0..6 {
        // Print the best words and their expected bits
        println!("Best words and their expected bits:");
        for entry in info::top_words(&words, 10) {
            println!("{}: {:.3} ({})", entry.word, entry.bits, entry.freq);
        }
        println!();
        println!();

        // Read the word and result from the user
        let tried_word = read::read_word();
        let result = read::read_result();

        // Get the expected bits of information for the word
        let expected_information = words[&tried_word].bits;

        // Print the actual bits of information and the expected bits
        println!(
            "Actual information: {:.3} bits / {:.3} bits",
            info::bits_of_information_for_a_possibility(&tried_word, &result, &words),
            expected_information
        );

        // Update the words that match the result
        words = matcher::match_words(&tried_word, &result, &words);

        // Get the current number of words and check if the game is over
        let total_words: usize = words.len();
        if total_words == 0 {
            println!("No words match the result.");
            return;
        } else if total_words == 1 {
            println!("The word is: {}", words.keys().next().unwrap());
            return;
        }

        // Update the bits of information for all the words
        words = info::update_bits(&possibilities, &words);

        // Print the remaining number of possibilities and the percentage of the initial total
        println!(
            "Remaining possibilities: {}/{} ~ {:.3}%",
            total_words,
            initial_total,
            total_words as f32 / initial_total as f32 * 100.0
        );
    }
}
