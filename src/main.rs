use poketerm::ui::driver;

fn main() {
<<<<<<< Updated upstream
    let map = poketerm::Pokedex::lists::get_all_pokemon().unwrap();
    for (name, url) in &map {
        println!("{}:{}", name, url);
    }
=======
  driver::draw_ui().unwrap();
>>>>>>> Stashed changes
}
