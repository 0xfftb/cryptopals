pub fn run() {
    println!("Challenge 1: Convert hex to base64");

    // Input hex string
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    // Expected base64 output
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let result = HexString::to_base64(&input);

    assert_eq!(result, expected);
}

pub trait BinaryString {
    fn to_hex(&self) -> String;
    fn to_base64(&self) -> String;
}

impl BinaryString for &str {
    fn to_hex(&self) -> String {
        let chunks = self.split_in_chunks(4);

        chunks
            .into_iter()
            .map(|bits| match bits {
                "0000" => "0",
                "0001" => "1",
                "0010" => "2",
                "0011" => "3",
                "0100" => "4",
                "0101" => "5",
                "0110" => "6",
                "0111" => "7",
                "1000" => "8",
                "1001" => "9",
                "1010" => "a",
                "1011" => "b",
                "1100" => "c",
                "1101" => "d",
                "1110" => "e",
                "1111" => "f",
                _ => "", // Default case for any unexpected bit pattern
            })
            .collect()
    }

    fn to_base64(&self) -> String {
        let chunks = self.split_in_chunks(6);

        chunks
            .into_iter()
            .map(|bits| match bits {
                "000000" => "A",
                "000001" => "B",
                "000010" => "C",
                "000011" => "D",
                "000100" => "E",
                "000101" => "F",
                "000110" => "G",
                "000111" => "H",
                "001000" => "I",
                "001001" => "J",
                "001010" => "K",
                "001011" => "L",
                "001100" => "M",
                "001101" => "N",
                "001110" => "O",
                "001111" => "P",
                "010000" => "Q",
                "010001" => "R",
                "010010" => "S",
                "010011" => "T",
                "010100" => "U",
                "010101" => "V",
                "010110" => "W",
                "010111" => "X",
                "011000" => "Y",
                "011001" => "Z",
                "011010" => "a",
                "011011" => "b",
                "011100" => "c",
                "011101" => "d",
                "011110" => "e",
                "011111" => "f",
                "100000" => "g",
                "100001" => "h",
                "100010" => "i",
                "100011" => "j",
                "100100" => "k",
                "100101" => "l",
                "100110" => "m",
                "100111" => "n",
                "101000" => "o",
                "101001" => "p",
                "101010" => "q",
                "101011" => "r",
                "101100" => "s",
                "101101" => "t",
                "101110" => "u",
                "101111" => "v",
                "110000" => "w",
                "110001" => "x",
                "110010" => "y",
                "110011" => "z",
                "110100" => "0",
                "110101" => "1",
                "110110" => "2",
                "110111" => "3",
                "111000" => "4",
                "111001" => "5",
                "111010" => "6",
                "111011" => "7",
                "111100" => "8",
                "111101" => "9",
                "111110" => "+",
                "111111" => "/",
                _ => "", // Default case for any unexpected bit pattern
            })
            .collect()
    }
}

pub trait HexString {
    fn to_bin(&self) -> String;
    fn to_base64(&self) -> String;
    fn to_ascii(&self) -> String;
}

impl HexString for &str {
    fn to_bin(&self) -> String {
        self.chars()
            .map(|hex_byte| match hex_byte {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'a' => "1010",
                'b' => "1011",
                'c' => "1100",
                'd' => "1101",
                'e' => "1110",
                'f' => "1111",
                _ => "",
            })
            .collect()
    }

    fn to_base64(&self) -> String {
        let binary_string = self.to_bin();
        // Encode to base64
        BinaryString::to_base64(&binary_string.as_str())
    }

    fn to_ascii(&self) -> String {
        let chunks = self.split_in_chunks(2);

        chunks
            .into_iter()
            .map(|hex_chunk| {
                // Parse the hex string into a u32 value
                let value = u32::from_str_radix(hex_chunk, 16).expect("Invalid hex string");

                // Convert the numeric value to a character
                char::from_u32(value).expect("Invalid Unicode scalar value")
            })
            .collect()
    }
}

trait Chunkable {
    fn split_in_chunks(&self, chunk_size: usize) -> Vec<&str>;
}

impl Chunkable for &str {
    fn split_in_chunks(&self, chunk_size: usize) -> Vec<&str> {
        let mut chunks = Vec::new();

        // Check if string is empty
        if self.is_empty() {
            return chunks;
        }

        let mut start = 0;
        while start < self.len() {
            // Calculate end position, ensuring we don't go past the string length
            let mut end = start + chunk_size;
            if end >= self.len() {
                end = self.len();
            } else {
                // Ensure we don't split in the middle of a character
                while !self.is_char_boundary(end) {
                    end -= 1;
                }
            }

            // Add this chunk to our result vector
            chunks.push(&self[start..end]);

            // Move to next chunk
            start = end;
        }

        chunks
    }
}
