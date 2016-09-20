extern crate itertools;
extern crate cryptopals;

use std::fs::File;
use std::iter;
use std::io::{BufRead, BufReader};
use std::thread;

use itertools::Itertools;

use cryptopals::basics::ByteArray;
use cryptopals::util;

fn score_text(bytearray: &ByteArray) -> usize {
    bytearray.clone().into_iter()
        .fold(0, |sum, byte| sum + util::english_letter_frequency(byte as char) )
}

fn score_line(line: &str) -> (usize, String) {
    let cipher = ByteArray::from_hex(line);
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
    (max.unwrap(), plaintext.to_ascii())
}

#[test]
fn main() {
    let file = File::open("./etc/single-byte-xor.in").unwrap();
    let reader = BufReader::new(file);
    let mut threads = Vec::new();
    for line in reader.lines() {
        threads.push(thread::spawn(|| score_line(&line.unwrap())));
    }

    let mut scores = threads.into_iter()
        .map(|guard| guard.join().ok().unwrap())
        .collect_vec();
    scores.sort_by_key(|&(score, _)| score);
    let plaintext = scores.into_iter()
        .map(|(_, plaintext)| plaintext)
        .last().unwrap();

    println!("Highest scoring text: {}", plaintext);
}
