//This module import is probably too verbose...
use super::super::pokedex::lists;
use tui::widgets::{Widget, Block, Borders, Text, Paragraph, SelectableList};



pub trait Panel {
    fn draw(&self);
}

///Incapsulating struct for holding panels.
pub struct Window {
    chunks: std::vec::Vec<tui::layout::Rect>,
    panels: std::vec::Vec<Box<dyn Panel>>
}

impl Window {
    pub fn new()  {
        println!("WOOH");
        
    }
    pub fn render() {
        println!("rendering...")
    }
}

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

impl Panel for Namelist {
    fn draw(&self){
        println!("Drawing!");
    }
}

pub struct InfoText<'b> {
    pub texts: Vec<Text<'b>>
}

impl Panel for InfoText<'_> {
    fn draw(&self){
        println!("Drawing!");
    }
}

impl InfoText<'_> {
    pub fn add_text(&mut self, s: String) {
        let text = Text::raw(s);
        self.texts.push(text);
    }
    pub fn clear_text(&mut self) {
        self.texts = Vec::new();
    }
}
