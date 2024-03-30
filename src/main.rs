use serde::de::value;
use serde_json::{self, Number};
use core::num;
use std::env;

use serde_bencode;

fn convert_bencode_to_json(value: serde_bencode::value::Value) -> serde_json::Value {
    match value {
        serde_bencode::value::Value::Bytes(b) => {
            let string = String::from_utf8(b).unwrap();

            serde_json::Value::String(string)
        }
        serde_bencode::value::Value::Int(i) => {
            serde_json::Value::Number(Number::from(i))
        }
        serde_bencode::value::Value::List(l) => {
            let array = l
                .into_iter()
                .map(|item| convert_bencode_to_json(item))
                .collect();

            serde_json::Value::Array(array)
        }
        _ => {
            panic!("Unknown type")
        }
    }
}

fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let deserialized: serde_bencode::value::Value = serde_bencode::from_str(encoded_value).unwrap();

    convert_bencode_to_json(deserialized)
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
