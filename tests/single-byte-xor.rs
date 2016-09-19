extern crate cryptopals;

use std::iter;

use cryptopals::basics::ByteArray;
use cryptopals::util;

const CIPHER: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

fn score_text(bytearray: &ByteArray) -> usize {
    bytearray.clone().into_iter()
        .fold(0, |sum, byte| sum + util::english_letter_frequency(byte as char) )
}

#[test]
fn main() {
    let cipher = ByteArray::from_hex(CIPHER);
    let mut plaintext = ByteArray::new();
    let mut max = None;
    for i in 0..255 {
        let key = ByteArray(iter::repeat(i as u8).take(cipher.len()).collect());
        let text = key ^ cipher.clone();
        let score = score_text(&text);
        if max.is_none() || score > max.unwrap() {
            max = Some(score);
            plaintext = text;
        }
    }
    println!("Highest scoring text: {}", plaintext.to_ascii());
}
