use std::{error::Error, ops::Index};
const NOTES: [char; 12] = ['c', ' ', 'd', ' ', 'e', 'f', ' ', 'g', ' ', 'a', ' ', 'b'];

pub fn note_str_to_num(str: String) -> Result<u8, Box<dyn Error>> {
    let len = &str.len();
    if !(2..=3).contains(len) {
        return Err("Incorrect amount of characters in note name".into());
    }
    let chars: Vec<char> = str.chars().collect();
    let note = &chars[0];
    if !&NOTES.contains(note) {
        return Err("Invalid note name, must be A-G".into());
    }
    if *len == 3 && !['b', '#'].contains(&chars[1]) {
        return Err("Invalid second character, must be # or b".into());
    }
    let octave = chars.last().expect("Slice can't be empty");
    if !octave.is_ascii_digit() {
        return Err("Invalid octave, must be number between 0 and 9".into());
    }
    let octave = (octave.to_digit(10).unwrap() + 1) * 12;
    let note = NOTES.iter().position(|n| n == note).unwrap();
    let modifier = match &chars[1] {
        'b' => -1,
        '#' => 1,
        _ => 0,
    };
    Ok((octave as i32 + note as i32 + modifier) as u8)
}
