use crate::Api_Tools::api_tools;
use std::process;
use std::error;
use std::collections::HashMap;
use std::convert::TryInto;
fn populate_map(page: &serde_json::Value, map: &mut HashMap<String,String>) {
  for pokemon in page["results"].as_array().unwrap() {
    let pokemon_object = pokemon.as_object().unwrap();
        map.insert(
            pokemon_object["name"].to_string(),
            pokemon_object["url"].to_string()
        );
  }
}

fn get_page(url: &str) -> serde_json::Result<serde_json::Value> {
  let data = api_tools::curl_url(&url).unwrap_or_else( |err| {
      println!("Error {}", err);
      process::exit(1);
    });
  let response = api_tools::parse_response(&data).unwrap();
  Ok(response)
}

///Returns a HashMap<String,String> of all of the pokemon and their urls
///# Example
///```
///let map = poketerm::Pokedex::lists::get_all_pokemon().unwrap();
///```
pub fn get_all_pokemon() -> Result<HashMap<String,String>, Box<dyn error::Error>>{ 
  let mut pokemon_map:HashMap<String, String> = HashMap::new();
  let url = "https://pokeapi.co/api/v2/pokemon/";
  let mut data = get_page(&url).unwrap();
  //Get total number of pokemon
  let num_pokemon_i64 = data["count"].as_i64().unwrap();
  let num_pokemon: usize = (num_pokemon_i64 as usize).try_into().unwrap();
  
  println!("{:?}", data);
  while num_pokemon != pokemon_map.len() {
    populate_map(&data, &mut pokemon_map);
    if !data["next"].is_null(){
      let url = data["next"].to_string().replace("\"", "");
      data = get_page(&url).unwrap();   
    }
  }
  Ok(pokemon_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_pokemon() {
        let url = "https://pokeapi.co/api/v2/pokemon/";
        let data = get_page(&url).unwrap();
        let num_i64 = data["count"].as_i64().unwrap();
        let num_pokemon: usize = (num_i64 as usize).try_into().unwrap();
        let map = get_all_pokemon().unwrap();
        let map_len = map.len();
        assert_eq!(map_len, num_pokemon);
    }
}
