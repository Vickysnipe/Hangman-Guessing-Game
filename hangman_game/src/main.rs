use crossterm::{
    cursor::{Hide, MoveTo},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::Result;

const MAX_ERRORS: usize = 6;

fn read_lines(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let lines: Result<Vec<String>> = reader.lines().collect();

    lines
}

fn draw_gallows(errors: usize) {
    print!("  ___\n");
    print!(" |   |\n");
    print!(" |   {}\n", if errors > 0 { "O" } else { " " });
    print!(" |  {}{}{}\n", if errors > 1 { "/" } else { " " }, if errors > 3 { "|" } else { " " }, if errors > 2 { "\\" } else { " " });
    print!(" |  {} {}\n", if errors > 4 { "/" } else { " " }, if errors > 5 { "\\" } else { " " });
    print!("_|_\n");
    print!("\n");
}

fn draw_word(word: &str, guessed_letters: &[char]) {
    for c in word.chars() {
        if guessed_letters.contains(&c) {
            print!("{} ", c);
        } else {
            print!("_ ");
        }
    }
    print!("\n");
}

fn draw_interface(word: &str, guessed_letters: &[char], errors: usize) {
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        MoveTo(1, 1),
        Hide,
        Print("Hangman Game"),
        MoveTo(1, 3),
        Print("Guess the word:"),
        MoveTo(1, 4),
    ).unwrap();

    draw_gallows(errors);

    execute!(
        io::stdout(),
        MoveTo(1, 8),
        Print("Word: "),
    ).unwrap();

    draw_word(word, guessed_letters);

    execute!(
        io::stdout(),
        MoveTo(1, 12),
        Print("Enter a letter:"),
    ).unwrap();
}

fn main() {
    enable_raw_mode().unwrap();

    let filename = "words.txt";
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut errors = 0;

    let lines = match read_lines(filename) {
        Ok(lines) => lines,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    let word = match lines.choose(&mut rand::thread_rng()) {
        Some(random_line) => random_line.trim(),
        None => {
            eprintln!("No lines available in the file.");
            return;
        }
    };

    loop {
        draw_interface(word, &guessed_letters, errors);

        match event::read().unwrap() {
            Event::Key(event) => {
                if let KeyCode::Char(c) = event.code {
                    if c.is_alphabetic() {
                        guessed_letters.push(c);

                        if !word.contains(c) {
                            errors += 1;
                        }
                    }
                }
            }
            Event::Mouse(_) | Event::Resize(_, _) => {}
        }

        if errors >= MAX_ERRORS {
            break;
        }

        if word.chars().all(|c| guessed_letters.contains(&c)) {
            break;
        }
    }

    disable_raw_mode().unwrap();

    execute!(
        io::stdout(),
        Clear(ClearType::All),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        MoveTo(1, 1),
        ResetColor,
    ).unwrap();

    if errors >= MAX_ERRORS {
        println!("You lost! The word was: {}", word);
    } else {
        println!("Congratulations! You won!");
    }
}
