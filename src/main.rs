pub use std::env;
use datasucker2000::src::decode_bencoded_value;

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
