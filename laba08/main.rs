use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
fn main() -> io::Result<()> {
    p1()
}

fn p1() -> io::Result<()> {
    let path = "file.txt";
    let file = File::open(path)?;

    let mut word_freq = HashMap::<String, i32>::with_capacity(100);

    for line in io::BufReader::new(file).lines() {
        let line = line?;

        let words: Vec<&str> = line
            .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
            .collect();

        for word in words {
            *word_freq
                .entry(word.to_string().to_lowercase())
                .or_insert(0) += 1;
        }
    }
    word_freq.remove("");

    let mut sorted_words: Vec<(String, i32)> = word_freq.into_iter().collect();

    sorted_words.sort_by_key(|el| -el.1);
    let mut longest_word = 0u8;
    for el in sorted_words.iter() {
        if el.0.len()  > longest_word as usize {longest_word = el.0.len() as u8} 
    } 
    for el in sorted_words.iter() {
        println!("{:<w$}=> {}", el.0, el.1, w = longest_word as usize + 1);
    }

    Ok(())
}
