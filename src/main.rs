use ansi_term::{Color, Style};
use clearscreen;
use std::io::{stdin, stdout, Write};

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

#[derive(Clone, Copy)]
enum LetterState {
    Correct,
    Present,
    Absent,
    Unknown,
}

#[derive(Clone, Copy)]
struct Letter {
    ch: char,
    state: LetterState,
}

#[derive(Clone, Copy)]
struct Word {
    word: [Letter; 6],
}

impl Word {
    fn new() -> Word {
        Word {
            word: [Letter {
                ch: ' ',
                state: LetterState::Unknown,
            }; 6],
        }
    }
}

fn print_letter(letter: &Letter) {
    let style = match letter.state {
        LetterState::Correct => Style::new().on(Color::RGB(83, 141, 78)).fg(Color::White),
        LetterState::Present => Style::new().on(Color::RGB(181, 159, 59)).fg(Color::White),
        LetterState::Absent => Style::new().on(Color::RGB(58, 58, 60)).fg(Color::White),
        LetterState::Unknown => Style::new().on(Color::RGB(18, 18, 19)).fg(Color::White),
    };
    print!("{}", style.paint(format!(" {} ", letter.ch.to_string())));
}

fn read_word() -> Result<String, String> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.truncate(input.trim_end().len());

    let len = input.len();
    match len {
        5 => {
            input.make_ascii_uppercase();
            Ok(input)
        }
        _ => Err("The entered word is not 5 letters long.".to_string()),
    }
}

fn main() {
    // ====== Game Setup ======
    // configure ansi support for colors on windows.
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();
    // setup loop
    let mut should_loop: bool = true;
    // generate letters
    let mut letters: Vec<Letter> = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        .chars()
        .map(|letter| Letter {
            ch: letter,
            state: LetterState::Correct,
        })
        .collect();
    // generate tries
    let tries = 6;
    let tried_words: [Word; 6] = [Word::new(); 6];
    // generate word
    let _word = String::from("Hello");

    // ====== Game Loop ======
    while should_loop {
        clear();
        let mut should_pause = true;
        // print header
        {
            println!("+---------------------------------------+");
            println!("|                Wordle                 |");
            println!("+---------------------------------------+");
        }
        // read a word
        print!("Enter a word: ");
        stdout().flush().unwrap();
        match read_word() {
            Err(e) => println!("{e} try again."),
            Ok(w) => {
                let entered_word = w;
                println!("'{entered_word}'");
                should_pause = false;
                should_loop = false;
            }
        }
        if (should_pause) {
            pause();
        }
    }
}
