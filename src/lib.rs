use serde_json::{Result, Value};

///Parses json-strings into json serde structs.
///# Arguments
///* `data` - A json formatted string.
///
///# Example
///```
///let data = r#"{"name":"Sally", "age":20}"#;
///let response = pokidot::parse_response(&data).unwrap();
///```
pub fn parse_response(data: &str) -> Result<Value> {
    //Rust cannot infer this type (v)
    let v:Value = serde_json::from_str(data)?;
    Ok(v)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_response(){
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
}
