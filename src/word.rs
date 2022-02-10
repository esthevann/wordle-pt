use std::fmt::Display;

#[derive(Debug)]
pub struct Word {
    pub string: String,
    pub color: Vec<Color>,
}

impl Word {
    pub fn new(str: &str, color: &[Color]) -> Self {
        Word {
            string: str.to_owned(),
            color: color.to_vec(),
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.color{
            write!(f, "{}", i)?
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Yellow,
    Green,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Black => {
                write!(f, "{}", emoji::symbols::geometric::WHITE_LARGE_SQUARE.glyph)
            }
            Color::Yellow => {
                write!(f, "{}", emoji::symbols::geometric::YELLOW_SQUARE.glyph)
            }
            Color::Green => {
                write!(f, "{}", emoji::symbols::geometric::GREEN_SQUARE.glyph)
            }
        }
    }
}
