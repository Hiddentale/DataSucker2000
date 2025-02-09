use serde_json;
use std::env;
use std::fmt;

enum BencodeError {
    EmptyInput,
    MissingColon,
    InvalidLength,
    InvalidFormat
}

impl fmt::Display for BencodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BencodeError::EmptyInput => write!(f, "Input string is empty"),
            BencodeError::MissingColon => write!(f, "Expected colon in string format"),
            BencodeError::InvalidLength => write!(f, "Invalid length in string format"),
            BencodeError::InvalidFormat => write!(f, "Invalid bencode format"),
        }
    }
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
    if encoded_value.chars().last() != Some('e') {
        return Err(BencodeError::InvalidFormat);
    }
    let integer_in_string_format = &encoded_value[1..encoded_value.len() - 1];
    let integer = integer_in_string_format.parse::<i64>().map_err(|_| BencodeError::InvalidFormat)?;
    return Ok(serde_json::Value::Number(integer.into()));
}

fn decode_list(_encoded_value: &str) -> Result<serde_json::Value, BencodeError> {
    unimplemented!("List decoding has not been implemented yet!");
}
fn decode_dictionary(_encoded_value: &str) -> Result<serde_json::Value, BencodeError> {
    unimplemented!("Dictionary decoding has not been implemented yet!");
}

fn decode_bencoded_value(encoded_value: &str) -> Result<serde_json::Value, BencodeError> {
    if encoded_value.is_empty() {
        return Err(BencodeError::EmptyInput);
    }
    match encoded_value.chars().next().unwrap() {
        '0'..='9' => decode_string(encoded_value),
        'i' => decode_integer(encoded_value),
        'l' => decode_list(encoded_value),
        'd' => decode_dictionary(encoded_value),
        invalid_character => {
            eprintln!("Invalid initial character: {}", invalid_character);
            return Err(BencodeError::InvalidFormat);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        match decoded_value {
            Ok(value) => println!("{}", value.to_string()),
            Err(error) => eprintln!("Error: {}", error)
        }
        
    } else {
        println!("unknown command: {}", args[1])
    }
}
