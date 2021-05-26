use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

pub fn word_count() {
    let (file, content) = if let Some(path) = env::args().nth(1) {
        match fs::read_to_string(Path::new(&path)) {
            Ok(content) => (path, content.to_lowercase()),
            Err(e) => {
                eprintln!("Failed to read file {}: {:?}", path, e);
                std::process::exit(1);
            }
        }  
    } else {
        eprintln!("You must add a file as argument!");
        std::process::exit(2);
    };
    let words: Vec<&str> = content.split_whitespace().collect();
    process_words(words, &file);
}

fn process_words(words: Vec<&str>, file: &str) {
    let mut top_count = 0;
    let mut top_word = "";
    let mut words_aggregation: HashMap<&str, u32> = HashMap::new();
    for word in words {
        *words_aggregation.entry(word).or_insert(0) += 1;
        if words_aggregation.get(word).unwrap() > &top_count {
            top_count = *words_aggregation.get(word).unwrap();
            top_word = word;
        }
    }
    println!("the word '{}' is the most common word in '{}'. It appears {} times!", top_word, file, top_count);
}