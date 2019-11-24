use std::io;
use std::collections::HashMap;
use tui::Terminal;
use termion::event::Key;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use tui::widgets::{Widget, Block, Borders, SelectableList};
use tui::layout::{Layout, Constraint, Direction, Corner};
use termion::screen::AlternateScreen;
use termion::input::MouseTerminal;
use crate::ui::utils::event::{Event, Events};
use tui::style::{Color, Modifier, Style};
//This module import is probably too verbose...
use super::super::pokedex::lists;

fn pokemon_names () -> Vec<String> {
    let map: HashMap<String, String> = lists::get_all_pokemon().unwrap();
    map.keys().cloned().collect()
}

///Handles drawing the ui.
pub fn draw_ui() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();
    let names = pokemon_names();

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                [
                    Constraint::Percentage(100)
                ].as_ref()
                )
                .split(f.size());
            
            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("Pokemon"))
                .items(&names)
                .render(&mut f, chunks[0]);
        });

        match events.next().unwrap() {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                _ => {}
            },
            Event::Tick => {
            
            }
        }
    }
     
    Ok(())
}
