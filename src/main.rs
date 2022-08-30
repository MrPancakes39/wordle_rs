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

fn game_setup() {
    // configure ansi support for colors on windows.
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();
}

fn game_loop(should_loop: &mut bool) {
    // print header
    {
        println!("+---------------------------------------+");
        println!("|                Wordle                 |");
        println!("+---------------------------------------+");
    }
    *should_loop = false;
}

fn main() {
    let mut should_loop: bool = true;
    game_setup();
    while should_loop {
        game_loop(&mut should_loop);
    }
}
