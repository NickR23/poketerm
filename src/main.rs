use std::process;

fn main() {
    //r# negates the need to escape double quotes in a string!
    let data = r#"
    {
        "name":"Bob",
        "age": 20
    }"#;


    let response = pokidot::parse_response(&data).unwrap_or_else( |err| {
        println!("Error parsing data: {}",err);
        process::exit(1);
    });
    println!("Hey my name is {} and I'm {} years old!", response["name"], response["age"]);
}
