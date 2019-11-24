use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

///Handles drawing the ui.
pub fn draw_ui() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
        
    terminal.draw(|mut f| {
        let size = f.size();
        Block::default()
            .title("Pokedex")
            .borders(Borders::ALL)
            .render(&mut f, size);
    })  
}