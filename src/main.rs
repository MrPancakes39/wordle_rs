use ansi_term::{Color, Style};
use clearscreen;
use std::{
    collections::HashMap,
    fmt,
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
    state: Vec<LetterState>,
}

impl Word {
    fn new(size: usize) -> Word {
        Word {
            text: " ".repeat(size),
            state: vec![LetterState::Unknown; size],
        }
    }

    fn padding_print(&self, padding: usize) {
        let padding = " ".repeat(padding);
        // ceiling
        print!("{padding}");
        for _ in self.text.chars() {
            print!("+---");
        }
        print!("+\n");

        // letter
        print!("{padding}");
        for (i, ch) in self.text.chars().enumerate() {
            let style = letter_color(*self.state.get(i).unwrap());
            print!("|{}", style.paint(format!(" {ch} ")));
        }
        print!("|\n");

        // floor
        print!("{padding}");
        for _ in self.text.chars() {
            print!("+---");
        }
        print!("+\n");
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.padding_print(0);
        Ok(())
    }
}

fn letter_color(state: LetterState) -> Style {
    match state {
        LetterState::Correct => Style::new().on(Color::RGB(83, 141, 78)).fg(Color::White),
        LetterState::Present => Style::new().on(Color::RGB(181, 159, 59)).fg(Color::White),
        LetterState::Absent => Style::new().on(Color::RGB(58, 58, 60)).fg(Color::White),
        LetterState::Unknown => Style::new().on(Color::RGB(18, 18, 19)).fg(Color::White),
    }
}

fn compare_words(word: &str, to_match: &str) -> Result<Vec<LetterState>, String> {
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
    let mut states: Vec<LetterState> = vec![LetterState::Unknown; 5];

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

fn read_input() -> Result<String, String> {
    print!("Enter a word: ");
    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.truncate(line.trim_end().len());

    match line.len() {
        5 => Ok(line),
        _ => Err(format!("'{line}' is not 5 letters long.")),
    }
}

fn read_word(word: &str) -> Result<Word, String> {
    let guess = read_input()?;
    let states = compare_words(&guess, word)?;
    Ok(Word {
        text: guess,
        state: states,
    })
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
    let tried_words = vec![Word::new(5); 6];
    // generate word
    let word_to_guess = String::from("ONSET");

    // ====== Game Loop ======
    while should_loop {
        // print header
        {
            println!("+---------------------------------------+");
            println!("|                Wordle                 |");
            println!("+---------------------------------------+");
        }
        // print all tries
        for tword in &tried_words {
            tword.padding_print(10);
        }

        // read a word
        match read_word(&word_to_guess) {
            Err(e) => eprint!("{e} try again."),
            Ok(guess) => {
                println!("{}", guess);
            }
        }

        should_loop = false;
    }
}
