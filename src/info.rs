use std::collections::HashMap;

use crate::{matcher, Record};

// Calculate the expected bits of information for a word
fn bits_of_information(
    original: &str,
    possibilities: &Vec<String>,
    words: &HashMap<String, Record>,
) -> f32 {
    // Get the total number of words
    let total_words = words.len() as i32;
    let mut total = 0.0;

    // Iterate over all the possibilities
    for solution in possibilities.iter() {
        // Count the number of words that match the possibility
        let matches = words
            .keys()
            .filter(|word| matcher::matches_possibility(original, solution, word))
            .count() as f32;

        // Calculate the chances of the possibility
        let chances = matches / total_words as f32;

        // Add the bits of information to the total
        if chances > 0.0 {
            total += -chances * chances.log2();
        }
    }

    // Return the total bits of information
    total
}

// Update the bits of information for all the words
pub fn update_bits(
    possibilities: &Vec<String>,
    words: &HashMap<String, Record>,
) -> HashMap<String, Record> {
    // Create a new hashmap to store the words with updated bits of information
    let mut new_words: HashMap<String, Record> = HashMap::new();

    // Iterate over all the words
    for (word, info) in words.iter() {
        // Calculate the bits of information for the word
        let new_info = Record {
            word: word.clone(),
            bits: bits_of_information(&word, &possibilities, &words),
            freq: info.freq,
        };

        // Add the word to the new hashmap
        new_words.insert(word.clone(), new_info);
    }

    // Return the new hashmap
    new_words
}

// Calculate the bits of information for a tried word
pub fn bits_of_information_for_a_possibility(
    tried_word: &str,
    result: &String,
    words: &HashMap<String, Record>,
) -> f32 {
    // Count the number of words that match the result
    let mut matches = 0;

    // Create a copy of the words hashmap
    let words_copy = words.clone();

    // Get the total number of words
    let total_words = words_copy.len() as i32;

    // Iterate over all the words and check if they match the result
    for word in words_copy.keys() {
        if matcher::matches_possibility(tried_word, result, &word) {
            matches += 1;
        }
    }

    // If no words match the result, return 0 bits
    if matches == 0 {
        return 0.0;
    }

    // Calculate the bits of information
    -(matches as f32 / total_words as f32).log2()
}

// Get the top words with the highest bits of information
pub fn top_words(words: &HashMap<String, Record>, top: i32) -> Vec<Record> {
    // Create a vector to store the best words
    let mut best_words: Vec<Record> = Vec::new();

    // Create a record to store the best word
    let mut best: Record = Record {
        word: String::new(),
        bits: 0.0,
        freq: 0.0,
    };

    // Calculate the number of words to display
    let new_top = if top != 0 { top } else { words.len() as i32 };

    // Iterate over the spots in the vector
    for _ in 0..new_top {
        let mut max = 0.0;

        // Iterate over the words and get the one with the highest bits of information
        for info in words.values() {
            if info.bits > max && (info.bits < best.bits || best.bits == 0.0) {
                max = info.bits;
                best.word = info.word.clone();
                best.freq = info.freq;
            }
        }
        best.bits = max;

        // If the maximum bits of information is 0, break the loop
        if max == 0.0 {
            break;
        }

        // Make a copy of the best word
        let copy = Record {
            word: best.word.clone(),
            bits: best.bits,
            freq: best.freq,
        };

        // Add the best word to the vector
        best_words.push(copy);
    }

    // Return the vector of best words
    best_words
}
