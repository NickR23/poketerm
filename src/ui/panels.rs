//This module import is probably too verbose...
use super::super::pokedex::lists;
use tui::widgets::{Widget, Block, Borders, Text, Paragraph, SelectableList};

pub struct Namelist {
    pub names: Vec<String>,
    pub selected: Option<usize>,
}

impl Namelist {
    pub async fn new(pokedex: &lists::Pokedex) -> Namelist {
        Namelist {
            names: pokedex.get_pokemon_names(),
            selected: Some(0)
        }
    }   
}

pub struct Info_text<'b> {
    pub texts: Vec<Text<'b>>
}

impl Info_text<'_> {
    pub fn add_text(&mut self, s: String) {
        let text = Text::raw(s);
        self.texts.push(text);
    }
    pub fn clear_text(&mut self) {
        self.texts = Vec::new();
    }
}