use crate::api_tools::tools;
use futures::executor::block_on;
use std::process;
use std::error;
use std::collections::BTreeMap;
use std::convert::TryInto;

fn name_ify (s1: String) -> String {
  let mut c = s1.chars();
  //Get the first char 
  match c.next() {
      None => String::new(),
      Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
  }
}

fn populate_map(page: &serde_json::Value, map: &mut BTreeMap<String,String>) {
  for pokemon in page["results"].as_array().unwrap() {
    let pokemon_object = pokemon.as_object().unwrap();
        map.insert(
            name_ify(pokemon_object["name"].to_string().replace("\"", "")),
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

pub struct Pokedex {
    lookup: BTreeMap<String, String>
}
impl Pokedex {
  pub async fn new() -> Pokedex {
    let mut pokedex = Pokedex{ lookup: BTreeMap::new()};
    pokedex.lookup = pokedex.get_all_pokemon().unwrap();
    pokedex
  }

  ///Returns JSON formatted String containing information about the given pokemon
  pub fn get_info(&self, name: &str) -> Result<serde_json::Value, String> {
      if !self.lookup.contains_key(&name.to_string()){
          return Err("Name not found".to_string());
      }
      let url = self.lookup.get(&name.to_string()).unwrap();
      let url = url.to_string();
      let info = get_page(&url).unwrap();
      Ok(info)
  }

  ///Returns a HashMap<String,String> of all of the pokemon and their urls
  pub fn get_all_pokemon(&self) -> Result<BTreeMap<String,String>, Box<dyn error::Error>>{ 
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

  pub fn get_pokemon_names(&self) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for name in self.lookup.keys() {
        let mut s = name.clone();
        names.push(s);
    }
    names
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_ify() {
        let mut name = String::from("nick");
        name = name_ify(name);
        assert_eq!(name, "Nick");
    }

    #[test]
    fn test_get_info() {
        let pokedex = block_on(Pokedex::new());
        let info = pokedex.get_info(&"Zubat".to_string()).unwrap();
        assert_eq!(info["id"], 41);
    }

    #[test]
    fn test_get_pokemon_names() {
        let url = "https://pokeapi.co/api/v2/pokemon/";
        let data = get_page(&url).unwrap();
        let num_i64 = data["count"].as_i64().unwrap();
        let num_pokemon: usize = (num_i64 as usize).try_into().unwrap();

        let pokedex = block_on(Pokedex::new());

        let map_len = pokedex.lookup.len();
        assert_eq!(map_len, num_pokemon);
    }
}
