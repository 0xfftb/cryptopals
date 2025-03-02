use crate::challenge_1::HexString;
use crate::challenge_2::xor;

use std::collections::HashMap;

struct KeyResult {
    key: String,
    chars: HashMap<char, usize>,
}

pub fn run() {
    println!("Challenge 3: find XOR'd value");

    let hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let hex_length = hex_string.to_bin().len();

    let mut results: Vec<KeyResult> = Vec::new();

    for value in 0..255 {
        let hex_value = format!("{:x}", value);

        let mut key = String::from(&hex_value);

        while key.as_str().to_bin().len() < hex_length {
            key.push_str(hex_value.as_str());
        }

        let xord = xor(hex_string, key.as_str()).unwrap();
        let result = xord.as_str().to_ascii();

        let char_count = count_chars(result.as_str());

        results.push(KeyResult {
            key,
            chars: char_count,
        });
    }

    let mut winning_key = results.first().unwrap();
    let mut winning_score: f64 = 0.0;

    for result in &results {
        let score = score_text(&result.chars);
        if score > winning_score {
            winning_score = score;
            winning_key = result;
        }
    }

    println!("Encrytion key found: {}", winning_key.key);

    let decypted_msg = xor(hex_string, winning_key.key.as_str()).unwrap();
    println!("Decrypted message: {:?}", decypted_msg.as_str().to_ascii());
}

fn count_chars(text: &str) -> HashMap<char, usize> {
    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for c in text.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    char_counts
}

fn score_text(char_freqs: &HashMap<char, usize>) -> f64 {
    let english_freqs = get_english_frequencies();
    let total_chars: usize = char_freqs.values().sum();

    if total_chars == 0 {
        return 0.0;
    }

    let mut score = 0.0;

    // Calculate score based on character frequency
    for (c, &count) in char_freqs {
        let observed_freq = count as f64 / total_chars as f64;

        // For letters, compare with English frequency
        if c.is_ascii_alphabetic() {
            let lowercase = c.to_ascii_lowercase();
            if let Some(&expected_freq) = english_freqs.get(&lowercase) {
                // Higher score for characters with frequencies closer to English
                score += expected_freq * observed_freq;
            }
        }
        // For space, give a bonus
        else if *c == ' ' {
            if let Some(&expected_freq) = english_freqs.get(c) {
                score += expected_freq * observed_freq;
            }
        }
        // Penalize non-printable ASCII and control characters
        else if !c.is_ascii_graphic() && *c != ' ' {
            score -= observed_freq;
        }
    }

    score
}

fn get_english_frequencies() -> HashMap<char, f64> {
    let mut freq = HashMap::new();
    freq.insert('e', 0.12702);
    freq.insert('t', 0.09056);
    freq.insert('a', 0.08167);
    freq.insert('o', 0.07507);
    freq.insert('i', 0.06966);
    freq.insert('n', 0.06749);
    freq.insert('s', 0.06327);
    freq.insert('h', 0.06094);
    freq.insert('r', 0.05987);
    freq.insert('d', 0.04253);
    freq.insert('l', 0.04025);
    freq.insert('u', 0.02758);
    freq.insert('c', 0.02782);
    freq.insert('m', 0.02406);
    freq.insert('w', 0.02360);
    freq.insert('f', 0.02228);
    freq.insert('g', 0.02015);
    freq.insert('y', 0.01974);
    freq.insert('p', 0.01929);
    freq.insert('b', 0.01492);
    freq.insert('v', 0.00978);
    freq.insert('k', 0.00772);
    freq.insert('j', 0.00153);
    freq.insert('x', 0.00150);
    freq.insert('q', 0.00095);
    freq.insert('z', 0.00074);
    freq.insert(' ', 0.13000);
    freq
}
