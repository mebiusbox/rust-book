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
}

use guess_word_lib::*; // src/lib.rs の場合は use guess_word::*;

fn main() {
    println!("{}", Dictionary::new().get_random_word());
}
