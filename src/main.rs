use poketerm::UI::ui;
use std::process;
fn main() {
    ui::draw_ui().unwrap_or_else( |err| {
        println!("We ran into a problem with drawing the ui: {}", err);
        process::exit(1);
    });
}
