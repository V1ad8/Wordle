use std::io;

// Read a word from the user
pub fn read_word() -> String {
    // Ask the user to enter the word they tried
    println!("Enter the word:");
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read word");
    word = word.trim().to_string();

    // Check if the word is valid
    while word.len() != 5 {
        println!("Invalid word. Try again:");
        word.clear();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read word");
        word = word.trim().to_string();
    }

    // Print a new line
    println!();

    // Return the word
    word
}

// Check if a result is valid
fn is_result_valid(result: &String) -> bool {
    // Check if the result has 5 characters
    if result.len() != 5 {
        return false;
    }

    // Check if the result only contains the colors w, y, and g
    for i in 0..5 {
        match result.chars().nth(i).unwrap() {
            'w' | 'y' | 'g' => {}
            _ => return false,
        }
    }

    // The result is valid
    true
}

// Read the result from the user
pub fn read_result() -> String {
    // Ask the user to enter the result they got
    println!("Enter the result:");
    let mut result = String::new();
    io::stdin()
        .read_line(&mut result)
        .expect("Failed to read result");
    result = result.trim().to_string();

    // Check if the result is valid
    while !is_result_valid(&result) {
        println!("Invalid result. Try again:");
        result.clear();
        io::stdin()
            .read_line(&mut result)
            .expect("Failed to read result");
        result = result.trim().to_string();
    }

    // Print a new line
    println!();

    // Return the result
    result
}
