use std::{
    error::Error,
    io::{self, Write},
};

pub fn read_input() -> Result<String, Box<dyn Error>> {
    print!("Palavra de cinco letras:  ");
    io::stdout().flush()?;
    let mut string = String::new();
    io::stdin().read_line(&mut string)?;
    Ok(string)
}
