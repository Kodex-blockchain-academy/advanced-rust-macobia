// Word Counter
// Write a program that:
// 1. Reads a text file provided by the user.
// 2. Counts the occurrences of each word and displays the top 10 most frequent words.

se std::collections::HashMap;
use std::fs;
use std::io;

fn main() {
    // Prompt user for the file path
    println!("Enter the path to the text file:");

    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read input");
    let file_path = file_path.trim();

    // Read the file content
    let content = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };


    let mut word_count = HashMap::new();
    for word in content.split_whitespace() {
        let cleaned_word = word
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        *word_count.entry(cleaned_word).or_insert(0) += 1;
    }

   
    let mut word_vec: Vec<(&String, &usize)> = word_count.iter().collect();
    word_vec.sort_by(|a, b| b.1.cmp(a.1));

    // Display the top 10 most frequent words
    println!("\nTop 10 most frequent words:");
    for (i, (word, count)) in word_vec.iter().take(10).enumerate() {
        println!("{}. {} - {}", i + 1, word, count);
    }
}

