use crate::input::read_input;
use std::error::Error;

pub fn get_word(word_list: &[String]) -> Result<String, Box<dyn Error>> {
    let word = loop {
        let word = read_input()?;
        let is_valid = validate_word(word.trim(), word_list);
        if is_valid {
            break word;
        }
        if word.chars().count() < 5 {
            println!("Word must be five letters long");
        } else {
            println!("Word not in word list.");
        }
    };
    Ok(word)
}

fn validate_word(word: &str, word_list: &[String]) -> bool {
    match word_list.iter().find(|&x| x == word) {
        Some(_word) => true,
        None => false,
    }
}
