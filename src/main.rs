use serde_json;
use std::env;


enum BencodeError {
    EmptyInput,
    MissingColon,
    InvalidLength,
    InvalidFormat
}

fn decode_string(encoded_value: &str) -> Result<serde_json::Value, BencodeError> {
    let colon_index = encoded_value.find(':').ok_or(BencodeError::MissingColon)?;
    let number = encoded_value[..colon_index].parse::<i64>().map_err(|_| BencodeError::InvalidLength)?;
    let end_index = colon_index + 1 + number as usize;
    if end_index > encoded_value.len(){
        return Err(BencodeError::InvalidLength);
    }
    let string = &encoded_value[colon_index + 1..end_index];
    return Ok(serde_json::Value::String(string.to_string()));
}

fn decode_integer(encoded_value: &str) -> Result<serde_json::Value, BencodeError> {
    if encoded_value.chars().last() != 'e' {
        return Err(BencodeError::InvalidFormat);
    }
    let integer = &encoded_value[1..encoded_value.len() - 1].parse::<i64>().map_err(|_| BencodeError::InvalidLength)?;
    return Ok(serde_json::Value::Number(integer));
}

fn decode_list(encoded_value: &str) -> Result<serde_json::Value, BencodeError> {}
fn decode_dictionary(encoded_value: &str) -> Result<serde_json::Value, BencodeError> {}

fn decode_bencoded_value(encoded_value: &str) -> Result<serde_json::Value, BencodeError> {
    if encoded_value.is_empty() {
        return Err(BencodeError::EmptyInput);
    }
    match encoded_value.chars().next().unwrap() {
        '0'..='9' => decode_string(encoded_value),
        'i' => decode_integer(encoded_value),
        'l' => decode_list(encoded_value),
        'd' => decode_dictionary(encoded_value)
        invalid_character => {
            eprintln("Invalid initial character: {}", invalid_character)
            Err(BencodeError::InvalidFormat);
        }
    }
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
