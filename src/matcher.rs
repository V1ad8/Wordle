use std::collections::HashMap;

use crate::Record;

// Check if a letter appears more than once in a possibility
fn repeats(original: &str, colors: &String, possibility: &str, place: &usize) -> bool {
    for i in 0..5 {
        // Skip the current place
        if i == *place {
            continue;
        }

        // Check if the letter appears in the possibility
        if possibility.chars().nth(i).unwrap() == original.chars().nth(*place).unwrap() {
            // Check if the letter is not white
            if colors.chars().nth(i).unwrap() != 'w' {
                true;
            }
        }
    }

    // No repetitions
    false
}

// Check if a possibility matches the result of a word
pub fn matches_possibility(original: &str, colors: &String, possibility: &str) -> bool {
    for i in 0..5 {
        match colors.chars().nth(i).unwrap() {
            'w' => {
                // Eliminate the possibility if the white letter appears in it
                if possibility.contains(original.chars().nth(i).unwrap())
                    && !repeats(original, colors, possibility, &i)
                {
                    return false;
                }
            }
            'y' => {
                // Eliminate the possibility if the yellow letter does not appear in it or if it appears in the original place
                if possibility.chars().nth(i).unwrap() == original.chars().nth(i).unwrap()
                    || !possibility.contains(original.chars().nth(i).unwrap())
                {
                    return false;
                }
            }
            'g' => {
                // Eliminate the possibility if the green letter does not appear in it
                if possibility.chars().nth(i).unwrap() != original.chars().nth(i).unwrap() {
                    return false;
                }
            }
            _ => {}
        }
    }

    // The possibility matches the result
    true
}

pub fn match_words(
    tried_word: &str,
    result: &String,
    words: &HashMap<String, Record>,
) -> HashMap<String, Record> {
    // Create a new hashmap to store the words that match the result
    let mut new_words: HashMap<String, Record> = HashMap::new();

    // Iterate over the words and check if they match the result
    for (word, info) in words.iter() {
        if matches_possibility(tried_word, result, &word) {
            // If the word matches the result, add it to the new hashmap
            new_words.insert(word.clone(), info.clone());
        }
    }

    // Return the new hashmap
    new_words
}

// Generate all the possibilities
pub fn generate_possibilities() -> Vec<String> {
    // Create a vector to store the possibilities
    let mut possibilities = Vec::new();

    // Iterate over all the possibilities (white, yellow, green) of the 5 letters
    for first in 0..3 {
        for second in 0..3 {
            for third in 0..3 {
                for fourth in 0..3 {
                    for fifth in 0..3 {
                        let mut solution =
                            format!("{}{}{}{}{}", first, second, third, fourth, fifth);

                        // Replace the numbers with the colors
                        for i in 0..5 {
                            match solution.chars().nth(i).unwrap() {
                                '0' => solution.replace_range(i..i + 1, "w"),
                                '1' => solution.replace_range(i..i + 1, "y"),
                                '2' => solution.replace_range(i..i + 1, "g"),
                                _ => {}
                            }
                        }

                        // Add the possibility to the vector
                        possibilities.push(solution);
                    }
                }
            }
        }
    }

    // Return the vector of possibilities
    possibilities
}
