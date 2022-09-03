use ansi_term::{Color, Style};
use clearscreen;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use std::{
    collections::HashMap,
    fmt,
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

lazy_static! {
    static ref A: String = read_to_string("src/data/answers.txt").expect("missing answers.txt");
    static ref ANSWERS: Vec<&'static str> = A.lines().collect();
    static ref B: String = read_to_string("src/data/allowed.txt").expect("missing allowed.txt");
    static ref ALLOWED: Vec<&'static str> = B.lines().collect();
}

fn choose_word() -> String {
    String::from(*ANSWERS.choose(&mut rand::thread_rng()).unwrap())
}

fn valid_word(word: &str) -> bool {
    ALLOWED.contains(&word)
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn new(size: usize) -> Self {
        Word {
            text: " ".repeat(size),
            state: vec![LetterState::Unknown; size],
        }
    }

    fn padding_print(&self, padding: usize) {
        let txt = format!("{}", self);
        let pad = " ".repeat(padding);
        for line in txt.lines() {
            println!("{}{}", pad, line);
        }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ceiling
        for _ in self.text.chars() {
            write!(f, "+---")?;
        }
        write!(f, "+\n")?;

        // letter
        for (i, ch) in self.text.chars().enumerate() {
            let style = letter_color(*self.state.get(i).unwrap());
            write!(f, "|{}", style.paint(format!(" {ch} ")))?;
        }
        write!(f, "|\n")?;

        // floor
        for _ in self.text.chars() {
            write!(f, "+---")?;
        }
        write!(f, "+\n")
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

    // Find all Correct Letters
    for (i, ch) in word.chars().enumerate() {
        let comp: char = match_bytes[i] as char;
        if ch == comp {
            states[i] = LetterState::Correct;
            // and we decrease the count of the letter to account for duplicates
            letters.entry(ch).and_modify(|e| *e -= 1);
        }
    }

    // Then assign Present or Absent to the rest
    for (i, ch) in word.chars().enumerate() {
        if states[i] != LetterState::Correct {
            // if the letter isn't in the to_match word
            // or if it's count is 0
            if !letters.contains_key(&ch) || *letters.get(&ch).unwrap() == 0 {
                states[i] = LetterState::Absent;
            } else {
                states[i] = LetterState::Present;
                letters.entry(ch).and_modify(|e| *e -= 1);
            }
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
        5 => {
            line.make_ascii_uppercase();
            match valid_word(&line) {
                true => Ok(line),
                false => Err(format!("'{line}' is not a word.")),
            }
        }
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

fn print_keyboard(map: &HashMap<char, LetterState>) {
    let rows = ["QWERTYUIOP", "ASDFGHJKL", "ZXCVBNM"];
    let padding = [0, 2, 6];
    for i in 0..3 {
        let str = String::from(rows[i]);
        let mut states: Vec<LetterState> = vec![LetterState::Unknown; str.len()];
        for (i, ch) in str.chars().enumerate() {
            states[i] = *(*map).get(&ch).unwrap();
        }
        let word = Word {
            text: str,
            state: states,
        };
        word.padding_print(padding[i]);
    }
}

fn screen_play() {
    // ====== Game Setup ======
    // setup tries
    let mut tries: u32 = 6;
    let mut tried_words = vec![Word::new(5); 6];
    // generate word
    let word_to_guess = choose_word();
    // map letters to states
    let mut map: HashMap<char, LetterState> = HashMap::new();
    for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        map.insert(ch, LetterState::Unknown);
    }

    // ====== Game Loop ======
    let mut won: bool = false;

    while tries != 0 && !won {
        // clear the screen
        clear();
        // print header
        {
            println!("+---------------------------------------+");
            println!("|                Wordle                 |");
            println!("+---------------------------------------+");
        }
        // print all tries
        for tword in &mut tried_words {
            tword.padding_print(10);
        }
        // print the keyboard
        print_keyboard(&map);

        // read a word
        match read_word(&word_to_guess) {
            Err(e) => {
                eprintln!("{e} try again.");
                pause();
            }
            Ok(guess) => {
                // lowers amount of tries
                let index: usize = 6 as usize - tries as usize;
                tried_words[index] = guess;
                tries -= 1;
                // check if the word is found
                if &tried_words[index].text == &word_to_guess {
                    won = true;
                }
                // update mapping
                for (i, ch) in (*&tried_words[index]).text.chars().enumerate() {
                    let state = tried_words[index].state[i];
                    map.entry(ch).and_modify(|e| {
                        *e = match *e {
                            LetterState::Unknown => state,
                            LetterState::Absent | LetterState::Correct => *e,
                            LetterState::Present => match state {
                                LetterState::Absent | LetterState::Unknown => *e,
                                _ => state,
                            },
                        }
                    });
                }
                // end update mapping
            }
        }
    }

    // ====== Game End ======
    // redraw the screen
    {
        // clear the screen
        clear();
        // print header
        {
            println!("+---------------------------------------+");
            println!("|                Wordle                 |");
            println!("+---------------------------------------+");
        }
        // print all tries
        for tword in &mut tried_words {
            tword.padding_print(10);
        }
        // print the keyboard
        print_keyboard(&map);
    }
    if won {
        println!("Congradulations, YOU WON!!! 🎉️");
    } else {
        println!("Too Bad! You lose...😢️");
        println!("The word was: {}", word_to_guess);
    }
}

fn screen_tutorial() {
    clear();
    println!("HOW TO PLAY");
    println!("===========");
    println!("Guess the WORDLE in 6 tries.");
    println!("Each guess must be a valid 5-letter word. Hit the enter button to submit.");
    println!("After each guess, the color of the tiles will change to show how close your guess was to the word.");
    println!("---");
    println!("Example:");
    let mut word = Word {
        text: String::from("WEARY"),
        state: vec![LetterState::Unknown; 5],
    };
    word.state[0] = LetterState::Correct;
    print!("{}", word);
    println!("The letter W is in the word and in the correct spot.");
    let mut word = Word {
        text: String::from("PILLS"),
        state: vec![LetterState::Unknown; 5],
    };
    word.state[1] = LetterState::Present;
    print!("{}", word);
    println!("The letter I is in the word but in the wrong spot.");
    let mut word = Word {
        text: String::from("VAGUE"),
        state: vec![LetterState::Unknown; 5],
    };
    word.state[3] = LetterState::Absent;
    print!("{}", word);
    println!("The letter U is not in the word in any spot.");
    println!("---");
}

fn main() {
    // configure ansi support for colors on windows.
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    pause();

    let mut should_loop = true;

    while should_loop {
        // main screen
        clear();
        let header: String = format!("Wordle v{}", env!("CARGO_PKG_VERSION"));
        println!("{}", header);
        println!("{}", "=".repeat(header.len()));
        println!("1. Play The Game.");
        println!("2. How To Play.");
        println!("3. Exit.");

        // read answer
        let choice: i32;
        loop {
            let mut buffer = String::new();
            print!(": ");
            stdout().flush().unwrap();
            stdin()
                .read_line(&mut buffer)
                .expect("couldn't read from user");
            let buffer = buffer.trim();
            let n: i32 = match buffer.parse() {
                Ok(n) => n,
                Err(_e) => {
                    eprintln!("Not a valid number. try again");
                    continue;
                }
            };
            if n < 1 || n > 3 {
                eprintln!("Not a valid choice. try again");
                continue;
            }
            choice = n;
            break;
        }

        match choice {
            1 => screen_play(),
            2 => screen_tutorial(),
            3 => should_loop = false,
            _ => eprintln!("unreachable"),
        }

        if choice != 3 {
            pause();
        }
    }
}
