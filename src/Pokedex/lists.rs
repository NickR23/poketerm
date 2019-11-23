use crate::Api_Tools::api_tools;
use std::process;
use std::error;
use std::collections::HashMap;

///Returns a HashMap<String,String> of all of the pokemon and their urls
///# Example
///```
///let map = poketerm::Pokedex::lists::get_all_pokemon();
///```
pub fn get_all_pokemon() -> Result<HashMap<String,String>, Box<dyn error::Error>>{
  let url = "https://pokeapi.co/api/v2/pokemon/";
  let data = api_tools::curl_url(&url).unwrap_or_else( |err| {
    println!("Error {}", err);
    process::exit(1);
  });
  let response = api_tools::parse_response(&data).unwrap();
  let mut pokemon_map:HashMap<String, String> = HashMap::new();
  for pokemon in response["results"].as_array().unwrap() {
      let pokemon_object = pokemon.as_object().unwrap();
      pokemon_map.insert(
          pokemon_object["name"].to_string(),
          pokemon_object["url"].to_string()
          );
  }
  Ok(pokemon_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_pokemon() {
        let map = get_all_pokemon().unwrap();
        let map_len = map.len();
        assert_eq!(map_len, 20);
    }
}
