use std::process;

fn main() {
    let url = "https://pokeapi.co/api/v2/pokemon/ditto/";
    let data = pokedot::curl_url(&url).unwrap_or_else( |err| {
        println!("Error: {}", err);
        process::exit(1);
    });
    println!("{}",data);
}
