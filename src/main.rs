use std::error::Error;
use std::fs;
use std::path::Path;

use deunicode::deunicode;
use rand::prelude::SliceRandom;

use crate::validation::get_word;
use crate::word::{Color, Word};

pub mod input;
mod validation;
mod word;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let word_list = get_word_list("br-utf8.txt").expect("Couldn't open the word list");
    let possible_words =
        get_word_list("possible_words.txt").expect("Couldn't open the possible answers list");
    let selected_word = possible_words.choose(&mut rng).unwrap();

    let mut attempts = 0;
    let mut attempted_words: Vec<Word> = Vec::new();
    let mut abc = "abcdefghijklmnopqrstuvwxyzç".to_owned();

    loop {
        print_score(&attempted_words);

        println!("letras possíveis: {}", abc);
        let word = get_word(&word_list)?;
        

        let (deunicoded_input, deunicoded_selected) =
            (deunicode(word.trim()), deunicode(selected_word));

        let (colors, letters) = get_score(&deunicoded_input, &deunicoded_selected);
        abc.retain(|x| !letters.contains(&x));
        attempted_words.push(Word::new(word.trim(), &colors));

        if deunicoded_input == deunicoded_selected {
            print!("\x1B[2J\x1B[1;1H");
            print_score(&attempted_words);
            println!("Você venceu.");
            break;
        }

        attempts += 1;
        if attempts >= 6 {
            print!("\x1B[2J\x1B[1;1H");
            print_score(&attempted_words);
            println!("A palavra era {}", selected_word);
            break;
        } else {
            print!("\x1B[2J\x1B[1;1H");
        }
    }

    Ok(())
}

fn print_score(attempted_words: &[Word]) {
    for i in attempted_words.iter() {
        for j in &i.color {
            print!("{}", j)
        }
        print!(": {}", i.string);
        println!()
    }
}

pub fn get_score(deunicoded_input: &str, deunicoded_selected: &str) -> (Vec<Color>, Vec<char>) {
    let rejected_chars: Vec<_> = deunicoded_input
        .chars()
        .filter(|&x| !deunicoded_selected.contains(x))
        .collect();

    let colors: Vec<_> = deunicoded_input
        .chars()
        .zip(deunicoded_selected.chars())
        .map(|(ic, sc)| {
            if deunicoded_selected.contains(ic) {
                if ic == sc {
                    Color::Green
                } else {
                    Color::Yellow
                }
            } else {
                Color::Black
            }
        })
        .collect();
    (colors, rejected_chars)
}

fn get_word_list<P>(path: P) -> Result<Vec<String>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let file = fs::read_to_string(path)?;
    let five_letters: Vec<_> = file
        .lines()
        .filter(|&x| x.chars().count() == 5)
        .map(|x| x.to_owned())
        .collect();
    Ok(five_letters)
}
