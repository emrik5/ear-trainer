use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiOutput, MidiOutputPort};
use note_parse::note_str_to_num;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub mod note_parse;
struct Options {
    range: (u8, u8),
    seq_mode: SeqMode,
    note_mode: NoteMode,
    game_mode: GameMode,
    allow_repeat: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            // C3-C5
            range: (48, 72),
            seq_mode: SeqMode::TrueRandom,
            note_mode: NoteMode::Sequential,
            game_mode: GameMode::Intervals,
            allow_repeat: true,
        }
    }
}
#[derive(FromPrimitive)]
enum SeqMode {
    TrueRandom,
    NoRepeat,
}
#[derive(FromPrimitive)]
enum NoteMode {
    Cluster,
    Sequential,
    Random,
}
#[derive(FromPrimitive)]
enum GameMode {
    Notes,
    Intervals,
    Chords,
    Scales,
}

fn main() {
    loop {
        match run() {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("Ear Trainer Output")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err(
            "No MIDI output port found, please connect a MIDI device or open a virtual instrument"
                .into(),
        ),
        1 => {
            println!(
                "Using the only available MIDI output: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nPlease choose a MIDI output:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            loop {
                print!("Enter port number: ");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                if let Ok(port_num) = input.trim().parse::<usize>() {
                    if let Some(port) = out_ports.get(port_num) {
                        break port;
                    }
                }
                println!("Invalid port selection");
            }
        }
    };

    println!("\nOpening MIDI connection...");
    let mut conn_out = midi_out.connect(out_port, "ear-trainer-out")?;
    sleep(Duration::from_secs(1));

    // Clear the terminal
    println!("\x1bc");
    // Print fancy title
    println!("\x1b[0m  \x1b[100m                               ");
    print!("\x1b[47m\x1b[30m");
    println!(" --Welcome to Ear Trainer v0.1!-- ");
    println!("\x1b[0m  \x1b[100m                               ");
    // Reset colors
    println!("\x1b[0m");

    let mut options = Options::default();
    {
        // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
        let mut play_note = |note: u8, duration: u64| {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const VELOCITY: u8 = 100;
            // We're ignoring errors in here
            let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
            sleep(Duration::from_millis(duration * 150));
            let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
        };

        // loop {
        //     let mut inp = String::new();
        //     stdin().read_line(&mut inp).unwrap();
        //     match note_str_to_num(inp.trim().to_owned()) {
        //         Ok(n) => play_note(n, 2),
        //         Err(err) => println!("Error: {}", err),
        //     }
        // }
    }
    sleep(Duration::from_millis(150));
    new_game(&mut options);
    println!("\nClosing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    conn_out.close();
    println!("Connection closed");

    Ok(())
}
fn new_game(options: &mut Options) -> Result<(), Box<dyn Error>> {
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

    let prompt_for_range = |prompt: &str| -> u8 {
        loop {
            print!("{}: ", prompt);
            if let Err(_) = stdout().flush() {
                println!("A read error occurred, please try again");
                continue;
            };
            match || -> Result<u8, Box<dyn Error>> {
                let mut inp = String::new();
                stdin().read_line(&mut inp)?;
                let inp = inp.trim();
                let note = note_str_to_num(inp)?;
                Ok(note)
            }() {
                Ok(note) => break note,
                Err(e) => println!("Error: {}", e),
            }
        }

    };
    println!("Please select a note range: ");
    loop {
        options.range.0 = prompt_for_range("Range lower bound");
        options.range.1 = prompt_for_range("Range upper bound");
        if options.range.0 < options.range.1 {
            break;
        } else {
            println!("Upper bound can't be same as or lower than first bound");
        }
    }
    println!("Allow repetition of questions?");
    println!("0: ");
    prompt_option(&mut options.allow_repeat);


    Ok(())
}
fn prompt_option<T: FromPrimitive>(option: &mut T) {
    loop {
        print!("Enter number: ");
        if let Err(_) = stdout().flush() {
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
