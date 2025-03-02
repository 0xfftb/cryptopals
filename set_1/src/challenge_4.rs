use crate::challenge_1::HexString;
use crate::challenge_2::xor;
use crate::challenge_3::break_xor_cipher;
use std::fs::read_to_string;

pub fn run() {
    let mut messages = Vec::new();

    for line in read_to_string("4.txt").unwrap().lines() {
        messages.push(line.to_string())
    }

    let mut winning_score = 0 as f64;

    let mut results = Vec::new();

    for msg in messages {
        let result = break_xor_cipher(msg.as_str());

        if result.1 > winning_score {
            winning_score = result.1;
        }

        if result.0.key != "000000000000000000000000000000000000000000000000000000000000" {
            results.push((result, msg));
        }
    }

    for i in &results {
        let decypted_msg = xor(i.1.as_str(), i.0.0.key.as_str()).unwrap();
        println!("Decrypted message: {:?}", decypted_msg.as_str().to_ascii());
        println!("Decrypted message: {:?}", i.0.1);
    }

    let final_result = results.iter().find(|res| res.0.1 == winning_score).unwrap();

    println!("Encrytion key found: {}", final_result.0.0.key);
    let decypted_msg = xor(final_result.1.as_str(), final_result.0.0.key.as_str()).unwrap();
    println!("Decrypted message: {:?}", decypted_msg.as_str().to_ascii());
}
