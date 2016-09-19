pub fn english_letter_frequency(letter: char) -> usize {
    match letter {
        'A' | 'E' | 'I' | 'O' | 'U' => 5,
        'a' | 'e' | 'i' | 'o' | 'u' => 5,
        'R' | 'S' | 'T' | 'L' | 'N' => 2,
        'r' | 's' | 't' | 'l' | 'n' => 2,
        ' ' | ',' | '-' | ':' | '.' => 1,
        _                           => 0
    }
}
