use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

pub fn word_count() {
    let (file, content) = read_content_file();
    let words = content.split_whitespace().collect();
    let words_aggregation = aggregate_words(words);
    display_top_5_most_common_words(&words_aggregation, &file);
}

fn read_content_file() -> (String, String) {
    let (file, content) = if let Some(file) = env::args().nth(1) {
        match fs::read_to_string(Path::new(&file)) {
            Ok(content) => (file, content.to_lowercase()),
            Err(e) => {
                eprintln!("Failed to read file {}: {:?}", file, e);
                std::process::exit(1);
            }
        }  
    } else {
        eprintln!("You must add a file as argument!");
        std::process::exit(2);
    };
    (file, content)
}

fn aggregate_words(words: Vec<&str>) -> Vec<(String, u32)> {
    let mut words_aggregation: HashMap<String, u32> = HashMap::new();
    for word in words {
        *words_aggregation.entry(word.to_string()).or_insert(0) += 1;
    }
    words_aggregation.iter().map(|(w, c)| (w.clone(), c.to_owned())).collect()
}

fn display_top_5_most_common_words(words: &[(String, u32)], file: &str) {
    let mut sorted_words = words.to_owned();
    sorted_words.sort_by(|(_, c1), (_, c2)| c2.cmp(&c1));
    println!("Top 5 of the most common words in '{}'", file);
    for (word, count) in sorted_words.iter().take(5) {
        println!("'{}' appears {} times", word, count)
    }

}