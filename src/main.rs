use std::process;
use poketerm::apiTools;

fn main() {
    let url = "https://pokeapi.co/api/v2/pokemon/ditto/";
    let data = apiTools::curl_url(&url).unwrap_or_else( |err| {
        println!("Error: {}", err);
        process::exit(1);
    });
    println!("{}",data);
}
