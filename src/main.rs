use std::error::Error;
use std::fs;


use deunicode::deunicode;
use rand::prelude::SliceRandom;

use crate::validation::get_word;
use crate::word::{Color, Word};

pub mod input;
mod validation;
mod word;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let word_list = get_word_list().expect("Couldn't open the word list");
    let selected_word = word_list.choose(&mut rng).unwrap();

    let mut game_over = false;
    let mut attempts = 0;
    let mut attempted_words: Vec<Word> = Vec::new();

    while !game_over {
        print_score(&attempted_words);
        
        let word = get_word(&word_list)?;

        let (deunicoded_input, deunicoded_selected) =
            (deunicode(word.trim()), deunicode(selected_word));

        let colors = get_score(&deunicoded_input, &deunicoded_selected);
        attempted_words.push(Word::new(word.trim(), &colors));

        if deunicoded_input == deunicoded_selected {
            print!("\x1B[2J\x1B[1;1H");
            game_over = true;
            print_score(&attempted_words);
            break;
        }

        attempts += 1;
        if attempts >= 6 {
            print!("\x1B[2J\x1B[1;1H");
            print_score(&attempted_words);
            game_over = true;
            break;
        } else{
            print!("\x1B[2J\x1B[1;1H");
        }
    }


    Ok(())
}

fn print_score(attempted_words: &[Word]) {
    for i in attempted_words.iter(){
        for j in &i.color{
            print!("{}", j)
        }
        print!(": {}", i.string);
        println!()
    }
}

pub fn get_score(deunicoded_input: &str, deunicoded_selected: &str) -> Vec<Color> {
    let mut colors = vec![];
    for (ic, sc) in deunicoded_input.chars().zip(deunicoded_selected.chars()) {
        if deunicoded_selected.contains(ic) {
            if ic == sc {
                colors.push(Color::Green)
            } else {
                colors.push(Color::Yellow)
            }
        } else {
            colors.push(Color::Black)
        }
    }
    colors
}

fn get_word_list() -> Result<Vec<String>, Box<dyn Error>> {
    let file = fs::read_to_string("br-utf8.txt")?;
    let five_letters: Vec<_> = file
        .lines()
        .filter(|&x| x.chars().count() == 5)
        .map(|x| x.to_owned())
        .collect();
    Ok(five_letters)
}
