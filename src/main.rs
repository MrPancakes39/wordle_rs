use ansi_term::{Color, Style};
use clearscreen;
use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

fn clear() {
    clearscreen::clear().expect("couldn't clear the screen");
}

fn pause() {
    print!("Press Enter to continue...");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut String::new())
        .expect("couldn't read input");
}

#[derive(Debug, Clone, Copy)]
enum LetterState {
    Correct,
    Present,
    Absent,
    Unknown,
}

#[derive(Debug, Clone)]
struct Word {
    text: String,
    state: [LetterState; 5],
}

impl Word {
    fn new() -> Word {
        Word {
            text: String::new(),
            state: [LetterState::Unknown; 5],
        }
    }
}

fn compare_words(word: &str, to_match: &str) -> Result<[LetterState; 5], String> {
    // this handles if word isn't 5 letters long
    // to_match is should always be 5 letters long
    if word.len() != to_match.len() && to_match.len() == 5 {
        return Err(format!("'{word}' should be 5 letters long."));
    }

    // get letters and their count
    let mut letters: HashMap<char, u32> = HashMap::new();
    for ch in to_match.chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }

    // initialize the array
    let mut states: [LetterState; 5] = [LetterState::Unknown; 5];

    // find the state of each letter
    let match_bytes = to_match.as_bytes();
    for (i, ch) in word.chars().enumerate() {
        // if the letter isn't in the to_match word
        // or if it's count is 0
        if !letters.contains_key(&ch) || *letters.get(&ch).unwrap() == 0 {
            states[i] = LetterState::Absent;
        } else {
            // else we compare the letters
            let comp: char = match_bytes[i] as char;
            states[i] = if ch == comp {
                LetterState::Correct
            } else {
                LetterState::Present
            };
            // and we decrease the count of the letter to account for duplicates
            letters.entry(ch).and_modify(|e| *e -= 1);
        }
    }

    Ok(states)
}

fn main() {
    // ====== Game Setup ======
    // configure ansi support for colors on windows.
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();
    // setup loop
    let mut should_loop: bool = true;
    // setup tries
    let tries: u32 = 6;
    let tried_word = vec![Word::new(); 6];
    // generate word
    let unknown_word = String::from("ONSET");

    // ====== Game Loop ======
    while should_loop {
        // print header
        {
            println!("+---------------------------------------+");
            println!("|                Wordle                 |");
            println!("+---------------------------------------+");
        }
        let states = compare_words("MOIST", &unknown_word).unwrap();
        println!("{:?}", states);
        should_loop = false;
    }
}
