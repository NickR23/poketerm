use std::io;
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
use std::process;
use serde_json;
use crate::ui::panels;
//This module import is probably too verbose...
use super::super::pokedex::lists;


struct frame_list{
    frames: Vec<Box<dyn Widget>>
}

pub fn runner() -> Result<(), io::Error> {
    println!("... Loading pokemon dictionary ...");
    let pokedex = block_on(lists::Pokedex::new());
    draw_ui(pokedex)?;
    Ok(())
}

///Handles drawing the ui.
fn draw_ui(pokedex: lists::Pokedex) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();
    
    let mut name_list = block_on(panels::Namelist::new(&pokedex));
    let mut info_text = panels::Info_text{texts: Vec::new()};
    
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
            let title_text = format!("Pokemon ({})", name_list.names.len());
            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title(&title_text))
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
        }).unwrap_or_else( |err| {
            println!("Error rendering tui: {:?}", err);
            process::exit(1);
        });

        match events.next().unwrap() {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Down => {
                    info_text.clear_text();
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
                    let selected_name = &name_list.names[name_list.selected.unwrap()];
                    let name_info = pokedex.get_info(selected_name).unwrap_or_else( |err| {
                        println!("{:?}", err);
                        process::exit(1);
                    });
                    info_text.add_text(serde_json::to_string_pretty(&name_info).unwrap());
                }
                Key::Up => {
                    info_text.clear_text();
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
