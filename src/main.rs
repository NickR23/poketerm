fn main() {
    let map = poketerm::pokedex::lists::get_all_pokemon().unwrap();
    for (name, url) in &map {
        println!("{}:{}", name, url);
    }
}
