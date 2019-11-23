use serde_json::{Result, Value};

///Parses json-strings into json serde structs.
///# Arguments
///* `data` - A json formatted string.
///
///# Example
///```
///let response = pokidot::parse_response(&data).unwrap();
///```
pub fn parse_response(data: &str) -> Result<Value> {
    //Rust cannot infer this type (v)
    let v:Value = serde_json::from_str(data)?;
    Ok(v)
}
