use crate::api_tools::tools;
use std::process;
use std::error;
use std::collections::BTreeMap;
use std::convert::TryInto;

fn populate_map(page: &serde_json::Value, map: &mut BTreeMap<String,String>) {
  for pokemon in page["results"].as_array().unwrap() {
    let pokemon_object = pokemon.as_object().unwrap();
        map.insert(
            pokemon_object["name"].to_string().replace("\"", ""),
            pokemon_object["url"].to_string().replace("\"","")
        );
  }
}

fn get_page(url: &str) -> serde_json::Result<serde_json::Value> {
  let data = tools::curl_url(&url).unwrap_or_else( |err| {
      println!("Error {}", err);
      process::exit(1);
    });
  let response = tools::parse_response(&data).unwrap();
  Ok(response)
}

///Returns JSON formatted String containing information about the given pokemon
pub fn get_info(name: &str, map: &BTreeMap<String, String>) -> Result<serde_json::Value, String> {
    if !map.contains_key(&name.to_string()){
        return Err("Name not found".to_string());
    }
    let url = map.get(&name.to_string()).unwrap();
    let url = url.to_string();
    let info = get_page(&url).unwrap();
    Ok(info)
}

///Returns a HashMap<String,String> of all of the pokemon and their urls
///# Example
///```
///let map = poketerm::pokedex::lists::get_all_pokemon().unwrap();
///```
pub fn get_all_pokemon() -> Result<BTreeMap<String,String>, Box<dyn error::Error>>{ 
  let mut pokemon_map:BTreeMap<String, String> = BTreeMap::new();
  let url = "https://pokeapi.co/api/v2/pokemon/";
  let mut data = get_page(&url).unwrap();
  //Get total number of pokemon
  let num_pokemon_i64 = data["count"].as_i64().unwrap();
  let num_pokemon: usize = (num_pokemon_i64 as usize).try_into().unwrap();
  
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
