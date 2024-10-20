use std::{collections::HashMap, fs::File};

use crate::Record;

pub fn open_file() -> HashMap<String, Record> {
    // Create a hashmap to store the words
    let mut words: HashMap<String, Record> = HashMap::new();

    // Create a CSV reader to read the words from the file
    let mut rdr = csv::Reader::from_reader(File::open("files/words.csv").unwrap());

    // Iterate over the words and add them to the hashmap
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();

        let info = Record {
            word: record.word.clone(),
            bits: record.bits,
            freq: record.freq,
        };

        words.insert(record.word.clone(), info);
    }

    // Return the hashmap
    words
}