// HANGMAN GAME IN RUST
use std::io;

fn main() {
    // secert word
    let secret_word: String = "alian".to_ascii_lowercase();

    // prep guess word
    let mut guessed_word: String = String::from("");
    for _ in 0..secret_word.len() {
        guessed_word.push('_');
    }
    // lives
    let mut lives = 7;
    let user_char: char = read_user_input().to_ascii_lowercase();
}

fn read_user_input() -> char {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => match user_input.chars().next() {
            Some(c) => return c,
            None => {
                println!("please input a char");
                return '*';
            }
        },
        Err(e) => {
            println!("{e}");
            return '*';
        }
    }
}
