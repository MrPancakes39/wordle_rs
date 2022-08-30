use ansi_term;
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

fn main() {
    // ====== Game Setup ======
    // configure ansi support for colors on windows.
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    // setup loop
    let mut should_loop: bool = true;

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
