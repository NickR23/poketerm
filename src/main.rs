use poketerm::ui::app;
use poketerm::pokedex::lists;
use futures::executor::block_on;
use std::collections::BTreeMap;
fn main() {
  app::runner().unwrap();
  // let pokedex = block_on(lists::Pokedex::new());
  // let map = pokedex.get_all_pokemon().unwrap();
  // for (name, value) in map {
  //   println!("{}:{}",name, value);
  // }
  
}
