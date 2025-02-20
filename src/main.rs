pub use serde_json;
pub use std::env;
pub use std::fmt;
pub use std::error::Error;

pub enum BencodeError {
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

pub fn decode_string(encoded_value: &str) -> Result<serde_json::Value, BencodeError> {
    let colon_index = encoded_value.find(':').ok_or(BencodeError::MissingColon)?;
    let number = encoded_value[..colon_index].parse::<i64>().map_err(|_| BencodeError::InvalidLength)?;
    let end_index = colon_index + 1 + number as usize;
    if end_index > encoded_value.len(){
        return Err(BencodeError::InvalidLength);
    }
    let string = &encoded_value[colon_index + 1..end_index];
    return Ok(serde_json::Value::String(string.to_string()));
}

pub fn decode_integer(encoded_value: &str) -> Result<serde_json::Value, BencodeError> {
    if encoded_value.chars().last() != Some('e') {
        return Err(BencodeError::InvalidFormat);
    }
    let integer_in_string_format = &encoded_value[1..encoded_value.len() - 1];
    let integer = integer_in_string_format.parse::<i64>().map_err(|_| BencodeError::InvalidFormat)?;
    return Ok(serde_json::Value::Number(integer.into()));
}

pub fn decode_list(encoded_value: &str) -> Result<(serde_json::Value, usize), BencodeError> {
    let mut list_data = Vec::new();
    let mut i = 1;
    while i < encoded_value.len() - 1 {
        match encoded_value.chars().nth(i).ok_or(BencodeError::InvalidFormat)? {
            '0'..='9' => { 
                let relative_colon_index = encoded_value[i..].find(":").ok_or(BencodeError::MissingColon)?;
                let true_colon_index = i + relative_colon_index;
                let length_str = &encoded_value[i..true_colon_index];
                let length = length_str.parse::<usize>().map_err(|_| BencodeError::InvalidLength)?;
                let string_end = true_colon_index + 1 + length;
                list_data.push(decode_string(&encoded_value[i..string_end])?);
                i = string_end;
            },
            'i' => {
                let mut integer_end = i;
                while encoded_value.chars().nth(integer_end).ok_or(BencodeError::EmptyInput)? != 'e' {
                    integer_end += 1
                }
                integer_end +=1;
                list_data.push(decode_integer(&encoded_value[i..integer_end])?);
                i = integer_end;
            },
            'l' => {
                let (nested_list, new_index) = decode_list(&encoded_value[i..])?;
                list_data.push(nested_list);
                i += new_index;
            },
            'e' => break,

            invalid_character => {
                eprintln!("Invalid initial character: {}", invalid_character);
                return Err(BencodeError::InvalidFormat);
            }                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         
        }
    }
    return Ok((serde_json::Value::Array(list_data), i + 1));

}

pub fn decode_dictionary(_encoded_value: &str) -> Result<serde_json::Value, BencodeError> {
    unimplemented!("Dictionary decoding has not been implemented yet!");
}

pub fn decode_bencoded_value(encoded_value: &str) -> Result<serde_json::Value, BencodeError> {
    match encoded_value.chars().next() {
        None => return Err(BencodeError::EmptyInput),
        Some(first_character) => match first_character {
            '0'..='9' => decode_string(encoded_value),
            'i' => decode_integer(encoded_value),
            'l' => decode_list(encoded_value).map(|(value, _)| value),
            'd' => decode_dictionary(encoded_value),
            invalid_character => {
                eprintln!("Invalid initial character: {}", invalid_character);
                return Err(BencodeError::InvalidFormat);
            }
        }
    }     
}

fn main() {
    //let encoded_value = "lli4ei5eee";
    //let decoded_value = decode_bencoded_value(encoded_value);
    //match decoded_value {
        //Ok(value) => println!("{}", value.to_string()),
        //Err(error) => eprintln!("Error: {}", error)
    //}
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
