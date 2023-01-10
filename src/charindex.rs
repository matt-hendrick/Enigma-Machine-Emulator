pub fn char_to_index(letter: char) -> u8 {
    assert!(letter.is_ascii_uppercase());
    letter as u8 - 65
}

pub fn index_to_char(int: u8) -> char {
    assert!(int < 26);
    (int as u8 + 65) as char
}

pub fn alphabet_string_to_u8_array(string: &str) -> [u8; 26] {
    if string.len() != 26 {
        panic!("String of characters is invalid length")
    }

    let mut array: [u8; 26] = [0; 26];

    for (i, letter) in string.char_indices() {
        array[i] = char_to_index(letter);
    }

    array
}
