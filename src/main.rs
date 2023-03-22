// HANGMAN GAME IN RUST
use rand::Rng;
use std::fs;
use std::io;

fn main() {
    // secert word
    let secret_word: String = get_secret_word().to_ascii_lowercase();

    // prep guess word
    let mut guessed_word: String = String::from("");
    for _ in 0..secret_word.len() {
        guessed_word.push('_');
    }
    // lives
    let mut lives = 11;

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
        println!("Tries left {lives}");
        println!("{}", guessed_word);
    }
    if secret_word == guessed_word {
        println!("Congrats you guessed the correct word `{secret_word}`")
    }
}
fn get_secret_word() -> String {
    let contents = fs::read_to_string("random_words.txt").unwrap();
    let mut list_of_words = vec![];
    for line in contents.lines() {
        list_of_words.push(line)
    }
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=1000);

    return String::from(list_of_words[random_number]);
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
