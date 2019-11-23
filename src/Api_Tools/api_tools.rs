use serde_json::{Result, Value};
use curl::easy::Easy;

///Parses json-strings into json serde structs.
///# Arguments
///* `data` - A json formatted string.
///
///# Example
///```
///let data = r#"{"name":"Sally", "age":20}"#;
///let response = poketerm::Api_Tools::api_tools::parse_response(&data).unwrap();
///```
pub fn parse_response(data: &str) -> Result<Value> {
    //Rust cannot infer this type (v)
    let v:Value = serde_json::from_str(data)?;
    Ok(v)
}

///Retrieves response from url endpoint.
///# Arguments
///* `url` - A string slice
///
///# Example
///```
///let url = "https://pokeapi.co/api/v2/pokemon/ditto/";
///let response = poketerm::Api_Tools::api_tools::curl_url(&url).unwrap();
///```
pub fn curl_url(url: &str) -> Result<String> {
    let mut handle = Easy::new();
    let mut data = Vec::new();
    handle.url(url).unwrap();

    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let response = String::from_utf8(data).unwrap();
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_response() {
        let data = r#"
        {
          "name":"Pikachu",
          "weight":120.25,
          "type":"electric"
        }"#;
        let response = parse_response(&data).unwrap();

        assert_eq!(response["weight"], 120.25);
        assert_eq!(response["name"], "Pikachu");
        assert_eq!(response["type"], "electric");
    }

    #[test]
    fn test_curl_url() {
        let url = "https://pokeapi.co/api/v2/pokemon/ditto/";
        let response = curl_url(&url).unwrap();
        let response_is_valid = response.contains("abilities");
        assert!(response_is_valid);
    }
}




