pub fn char_to_index(letter: char) -> u8 {
    assert!(letter.is_ascii_uppercase());
    letter as u8 - 65
}

pub fn index_to_char(int: u8) -> char {
    assert!(int < 26);
    (int as u8 + 65) as char
}
