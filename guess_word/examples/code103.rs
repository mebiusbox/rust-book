#![allow(dead_code)] // 未使用の警告を抑制
// このguess_word_libモジュールは、ここまでの src/lib.rs の内容になっています
mod guess_word_lib {
    use rand::seq::SliceRandom;
    use std::collections::HashSet;

    const WORDS: &str = include_str!("words.txt");

    pub struct Dictionary {
        words: HashSet<&'static str>,
    }

    impl Dictionary {
        pub fn new() -> Self {
            let words: HashSet<&str> = WORDS.split("\r\n").collect();
            Self { words }
        }
        pub fn get_random_word(&self) -> String {
            Vec::from_iter(self.words.iter())
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string()
        }
    }

    pub struct Game {
        answer: String,
        dictionary: Dictionary,
    }

    impl Default for Game {
        fn default() -> Self {
            let dict = Dictionary::new();
            Game {
                answer: dict.get_random_word(),
                dictionary: dict,
            }
        }
    }

    impl Game {
        pub fn get_answer(&self) -> String {
            self.answer.to_string()
        }
        pub fn in_dictionary(&self, word: &str) -> bool {
            self.dictionary.words.get(word).is_some()
        }
    }
}

use guess_word_lib::*; // src/lib.rs の場合は use guess_word::*;
use std::io;

fn main() {
    let game = Game::default();
    let mut guess = String::new();
    let answer = game.get_answer();
    println!("({})", answer);

    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let trimmed_guess = guess.trim();
        if game.in_dictionary(&trimmed_guess) {
            if answer == trimmed_guess {
                println!("You Win!");
                break;
            } else {
                println!("Not match word...");
            }
        } else {
            println!("Well... What's?");
        }

        guess.clear();
    }
}
