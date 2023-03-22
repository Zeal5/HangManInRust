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

    while lives > 0 && (secret_word != guessed_word) {
        println!("Guess a letter");
        let user_char: char = read_user_input().to_ascii_lowercase();

        if secret_word.contains(user_char) {
            lives -= 1;
            let mut v: Vec<usize> = vec![];
            for (i, c) in secret_word.char_indices() {
                if c == user_char {
                    v.push(i)
                }
            }
            for index in v {
                guessed_word.replace_range(index..index + 1, user_char.to_string().as_str())
            }
        } else {
            lives -= 1
        }
        println!("{}", guessed_word);
    }
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
