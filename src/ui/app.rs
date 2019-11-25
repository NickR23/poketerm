use std::io;
use std::collections::BTreeMap;
use futures::executor::block_on;
use tui::Terminal;
use termion::event::Key;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use tui::widgets::{Widget, Block, Borders, Text, Paragraph, SelectableList};
use tui::layout::{Layout, Constraint, Direction};
use termion::screen::AlternateScreen;
use termion::input::MouseTerminal;
use crate::ui::utils::event::{Event, Events};
use tui::style::{Color, Modifier, Style};
//This module import is probably too verbose...
use super::super::pokedex::lists;

fn name_ify (s1: String) -> String {
    let mut c = s1.chars();
    //Get the first char 
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}


fn pokemon_names() -> Vec<String> {
    let map: BTreeMap<String, String> = lists::get_all_pokemon().unwrap();
    let mut names: Vec<String> = Vec::new();
    for name in map.keys() {
        let mut s = name.clone();
        s = name_ify(s);
        names.push(s);
    }
    names
}

struct Namelist {
    names: Vec<String>,
    selected: Option<usize>,
}

impl Namelist {
    async fn new() -> Namelist {
        Namelist {
            names: pokemon_names(),
            selected: None
        }
    }   
}

struct Info_text<'b> {
    texts: Vec<Text<'b>>
}

impl Info_text<'_> {
    fn add_text(&mut self, s: String) {
        let text = Text::raw(s);
        self.texts.push(text);
    }
    fn clear_text(&mut self) {
        self.texts = Vec::new();
    }
}




pub fn runner() -> Result<(), io::Error> {
    println!("... Loading pokemon dictionary ...");
    let mut name_list = block_on(Namelist::new());
    draw_ui(name_list)?;
    Ok(())
}

///Handles drawing the ui.
fn draw_ui(mut name_list: Namelist) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();
    
    let mut info_text = Info_text{texts: Vec::new()};
    
    //Main event loop
    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
                )
                .split(f.size());
            
            let name_list_style = Style::default().fg(Color::Black).bg(Color::White);
            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("Pokemon"))
                .items(&name_list.names)
                .select(name_list.selected)
                .style(name_list_style)
                .highlight_style(name_list_style.fg(Color::LightGreen).modifier(Modifier::BOLD))
                .highlight_symbol(">")
                .render(&mut f, chunks[0]);
            
            Paragraph::new(info_text.texts.iter())
                .block(Block::default().borders(Borders::ALL).title("Info"))
                .wrap(true)
                .render(&mut f, chunks[1]);
        });

        match events.next().unwrap() {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Down => {
                    name_list.selected = if let Some(selected) = name_list.selected {
                        if selected >= name_list.names.len() - 1 {
                            Some(0)
                        } else {
                            Some(selected + 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                Key::Right => {
                    info_text.add_text("TEST".to_string());
                }
                Key::Up => {
                    name_list.selected = if let Some(selected) = name_list.selected {
                        if selected <= 0 {
                            Some(name_list.names.len() - 1)
                        } else {
                            Some(selected - 1)

                        }
                    } else  {
                        Some(0)
                    }
                }
                _ => {}
            },
            Event::Tick => {
            
            }
        }
    }
     
    Ok(())
}
