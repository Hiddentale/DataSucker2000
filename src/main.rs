use serde_json;
use std::env;


enum BencodeError {
    EmptyInput,
    MissingColon,
    InvalidLength,
    InvalidFormat
}


fn find_colon(encoded_value: &str) {

}

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    if encoded_value.is_empty() {
        return Err(BencodeError::EmptyInput);
    }
    match encoded_value.chars().next().unwrap() {
        '0'..='9' => decode_string(),
        'i' => decode_integer(),
        'l' => decode_list(),
        'd' => decode_dictionary()
    }

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        eprintln!("Logs");

        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
