use std::io::{stdin, stdout, Write};

use clearscreen;

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
    clear();
    println!("Hello Screen 1");
    pause();

    clear();
    println!("Hello Screen 2");
    pause();
}

// fn game_loop() {}

fn main() {
    game_setup();
    // loop {
    //     game_loop();
    // }
}
