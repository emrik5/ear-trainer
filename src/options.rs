use std::{
    error::Error,
    fmt::Display,
    io::{stdin, stdout, Write}, ops::{Range, RangeInclusive},
};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::note_parse::note_str_to_num;

pub struct Options {
    range: (u8, u8),
    range_names: (String, String),
    seq_mode: SeqMode,
    note_mode: NoteMode,
    game_mode: GameMode,
    allow_repeat: bool,
}
impl Options {
	pub fn range(&self) -> RangeInclusive<u8> {
		self.range.0..= self.range.1
	}
}
impl Default for Options {
    fn default() -> Self {
        Options {
            // C3-C5
            range: (48, 72),
            range_names: ("C3".into(), "C5".into()),
            seq_mode: SeqMode::TrueRandom,
            note_mode: NoteMode::Sequential,
            game_mode: GameMode::Intervals,
            allow_repeat: true,
        }
    }
}
impl Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Game Mode: {:?}
Note Mode: {:?}
Sequence: {:?}
Range: {}-{}
Allow Repeat: {}",
            self.game_mode,
            self.note_mode,
            self.seq_mode,
            self.range_names.0,
            self.range_names.1,
            self.allow_repeat
        )
    }
}
#[derive(FromPrimitive, Debug)]
enum SeqMode {
    TrueRandom,
    NoRepeat,
}
#[derive(FromPrimitive, Debug)]
enum NoteMode {
    Cluster,
    Sequential,
    Random,
}
#[derive(FromPrimitive, Debug)]
enum GameMode {
    Notes,
    Intervals,
    Chords,
    Scales,
}
pub fn new_game(options: &mut Options) -> Result<(), Box<dyn Error>> {
    println!("Please select game mode:");
    println!("0: Notes      -   Hear individual notes and enter their names");
    println!("1: Intervals  -   Hear two notes and enter the interval between them");
    println!("2: Chords     -   Hear chords and enter their names");
    println!("3: Scales     -   Hear scales and enter their names");
    prompt_option(&mut options.game_mode);

    println!("Please select note mode:");
    println!("0: Cluster    -   Notes are played simultaneously in chords/chordiods");
    println!("1: Sequential -   Notes are played in sequence");
    println!("2: Random     -   Pick mode randomly for each question");
    prompt_option(&mut options.note_mode);

    println!("Please select sequence mode");
    println!("0: True Random    -   Notes/Chords are picked at random");
    println!("1: No Repeat      -   Randomize, but make sure all questions appear before repetition");
    prompt_option(&mut options.seq_mode);

    let prompt_for_range = |prompt: &str| -> (u8, String) {
        loop {
            print!("{}: ", prompt);
            if stdout().flush().is_err() {
                println!("A read error occurred, please try again");
                continue;
            };
            match || -> Result<(u8, String), Box<dyn Error>> {
                let mut inp = String::new();
                stdin().read_line(&mut inp)?;
                let inp = inp.trim();
                let note = note_str_to_num(inp)?;
                let mut inp: Vec<_> = inp.chars().collect();
                inp[0].make_ascii_uppercase();
                let inp = inp.into_iter().collect();
                Ok((note, inp))
            }() {
                Ok(note) => break note,
                Err(e) => println!("Error: {}", e),
            }
        }
    };
    println!("Please select a note range: ");
    loop {
        (options.range.0, options.range_names.0) = prompt_for_range("Range lower bound");
        (options.range.1, options.range_names.1) = prompt_for_range("Range upper bound");
        if options.range.0 < options.range.1 {
            break;
        } else {
            println!("Upper bound can't be same as or lower than first bound");
        }
    }

    println!("Allow repetition of questions?");
    println!("0: Disallow   -   the sound for a question will only be heard once");
    println!("1: Allow      -   the sound for a question can be replayed at will");
    options.allow_repeat = loop {
        print!("Enter number: ");
        stdout().flush()?;
        let mut inp = String::new();
        stdin().read_line(&mut inp)?;
        match inp.trim().parse::<u8>() {
            Ok(0) => break false,
            Ok(1) => break true,
            Ok(_) | Err(_) => println!("Invalid input"),
        }
    };

    Ok(())
}
fn prompt_option<T: FromPrimitive>(option: &mut T) {
    loop {
        print!("Enter number: ");
        if stdout().flush().is_err() {
            println!("A read error occurred, please try again");
            continue;
        };
        match || -> Result<(), Box<dyn Error>> {
            let mut inp = String::new();
            stdin().read_line(&mut inp)?;
            let inp: u8 = inp.trim().parse()?;
            *option = FromPrimitive::from_u8(inp).ok_or("Failed cast to enum variant")?;
            Ok(())
        }() {
            Ok(_) => break,
            Err(_) => println!("Invalid input"),
        };
    }
}
