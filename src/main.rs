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

enum LetterState {
    Correct,
    Present,
    Absent,
    Unknown,
}

struct Letter {
    ch: char,
    state: LetterState,
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
    // generate word
    let _word = String::from("Hello");

    // ====== Game Loop ======
    while should_loop {
        // print header
        {
            println!("+---------------------------------------+");
            println!("|                Wordle                 |");
            println!("+---------------------------------------+");
        }
        should_loop = false;
    }
}
