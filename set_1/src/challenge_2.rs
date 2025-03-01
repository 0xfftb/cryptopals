use crate::challenge_1::{binary_to_hex, hex_to_binary};
use std::iter::zip;

pub fn run() {
    println!("Challenge 2: XOR");

    let a_string = "1c0111001f010100061a024b53535009181c";
    let b_string = "686974207468652062756c6c277320657965";

    let expected = "746865206b696420646f6e277420706c6179";

    let result = xor(a_string, b_string).unwrap();

    assert_eq!(result, expected);
}

fn xor(a: &str, b: &str) -> Result<String, String> {
    let a_bin = hex_to_binary(a);
    let b_bin = hex_to_binary(b);

    if a_bin.len() != b_bin.len() {
        return Err(String::from("Invalid inputs length"));
    }

    let a_chars = a_bin.chars();
    let b_chars = b_bin.chars();

    let xord_bits: String = zip(a_chars, b_chars)
        .map(|bit_pair| match bit_pair {
            ('0', '0') => '0',
            ('0', '1') => '1',
            ('1', '0') => '1',
            ('1', '1') => '0',
            _ => '0',
        })
        .collect();

    Ok(binary_to_hex(&xord_bits))
}
