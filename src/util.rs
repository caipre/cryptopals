use std::ascii::AsciiExt;

pub fn english_letter_frequency(letter: char) -> usize {
    match letter.to_ascii_lowercase() {
        'e' | 'a' | 't' | 'o' | ' ' => 10,
        'i' | 'n' | 's' | 'h' | 'r' => 6,
        'd' | 'l' | 'c' | 'u' | 'm' => 3,
        'w' | 'f' | 'g' | 'y' | 'p' => 2,
        'b' | 'v' | 'k'             => 1,
        _                           => 0
    }
}
