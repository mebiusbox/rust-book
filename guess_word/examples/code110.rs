#![allow(dead_code)] // 未使用の警告を抑制
// このguess_word_libモジュールは、ここまでの src/lib.rs の内容になっています
mod guess_word_lib {
    use rand::seq::SliceRandom;
    use std::collections::{HashMap, HashSet};

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

    pub const GUESS_LENGTH: usize = 5;  // 単語の文字列
    pub const GUESS_MAX: usize = 6;     // 推理の試行最大数

    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
    pub enum HitAccuracy {
        InRightPlace, // 位置が正しい
        InWord,       // 単語に含まれている
        NotInWord,    // 単語に含まれていない
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct GuessLetter {
        pub letter: char,
        pub accuracy: HitAccuracy,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct WordGuess {
        pub letters: Vec<GuessLetter>,
    }

    impl WordGuess {
        pub fn word(&self) -> String {
            self.letters.as_slice().iter().map(|gl| gl.letter).collect()
        }
        pub fn letters(&self) -> &[GuessLetter] {
            self.letters.as_slice()
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum GuessResult {
        DuplicateGuess,  // 推理単語が重複している
        IncorrectLength, // 文字数が不正
        NotInDictionary, // 単語辞書にない
        Valid,           // 有効
        GameOver,        // すでにゲームが終了している
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum GameStatus {
        Won,        // 勝利(推理が当たった)
        InProgress, // 推理中
        Lost,       // 敗北(推理がすべて当たらなかった)
    }

    #[derive(Debug, Clone)]
    pub enum GameError {
        GameNotLostError,
    }

    pub struct Game {
        guesses: Vec<WordGuess>,
        answer: String,
        game_status: GameStatus,
        dictionary: Dictionary,
    }

    impl Default for Game {
        fn default() -> Self {
            let dict = Dictionary::new();
            Game {
                guesses: Vec::with_capacity(GUESS_MAX),
                answer: dict.get_random_word(),
                game_status: GameStatus::InProgress,
                dictionary: dict,
            }
        }
    }

    impl Game {
        pub fn in_dictionary(&self, word: &str) -> bool {
            self.dictionary.words.get(word).is_some()
        }
        pub fn game_status(&self) -> GameStatus {
            self.game_status
        }
        pub fn get_answer(&self) -> Result<String, GameError> {
            if self.game_status == GameStatus::Lost {
                Ok(self.answer.to_string())
            } else {
                Err(GameError::GameNotLostError)
            }
        }
        pub fn guesses(&self) -> &[WordGuess] {
            self.guesses.as_slice()
        }
        fn build_letter_counts(&self, word: &str) -> HashMap<char, usize> {
            let mut counts = HashMap::new();
            for c in word.chars() {
                match counts.get_mut(&c) {
                    Some(v) => *v += 1,
                    None => {
                        counts.insert(c, 1);
                    }
                };
            }
            counts
        }
        fn answer_char_at_index(&self, index: usize) -> char {
            self.answer.chars().nth(index).unwrap()
        }
        fn matches_answer_at_index(&self, index: usize, letter: char) -> bool {
            letter == self.answer_char_at_index(index)
        }
        fn build_guess_letter_with_accuracy(
            &mut self,
            letter_index: usize,
            letter: char,
            available_letters: &mut HashMap<char, usize>,
        ) -> GuessLetter {
            let accuracy = match &self.answer.contains(letter) {
                true => {
                    let in_same_place = self.matches_answer_at_index(
                        letter_index, letter);
                    if in_same_place {
                        if let Some(ch) = available_letters.get_mut(&letter) {
                            *ch -= 1;
                        }
                        HitAccuracy::InRightPlace
                    } else if let Some(ch) = available_letters.get_mut(&letter) {
                        if (*ch) >= 1 {
                            *ch -= 1;
                            HitAccuracy::InWord
                        } else {
                            HitAccuracy::NotInWord
                        }
                    } else {
                        HitAccuracy::NotInWord
                    }
                }
                false => HitAccuracy::NotInWord,
            };

            GuessLetter { letter, accuracy }
        }
        fn build_guess(&mut self, guess_input: &str) -> WordGuess {
            let mut available_letters = self.build_letter_counts(&self.answer);
            let mut guess_letters: Vec<Option<GuessLetter>> = 
                vec![None; GUESS_LENGTH];

            for (idx, c) in guess_input.chars().enumerate() {
                if self.matches_answer_at_index(idx, c) {
                    guess_letters[idx] =
                        Some(self.build_guess_letter_with_accuracy(
                            idx, c, &mut available_letters))
                }
            }

            for (idx, c) in guess_input.chars().enumerate() {
                if guess_letters[idx].is_none() {
                    guess_letters[idx] =
                        Some(self.build_guess_letter_with_accuracy(
                            idx, c, &mut available_letters))
                }
            }

            WordGuess {
                letters: guess_letters.iter().map(|o| o.unwrap()).collect(),
            }
        }
        fn guess_already_exists(&self, guess_input: &str) -> bool {
            self.guesses
                .iter()
                .map(|g| g.word())
                .any(|x| x.eq(guess_input))
        }
        pub fn guess(&mut self, guess_input: &str) -> (GameStatus, GuessResult) {
            if self.game_status == GameStatus::Won || 
               self.game_status == GameStatus::Lost {
                return (self.game_status, GuessResult::GameOver);
            }
            if guess_input.len() != GUESS_LENGTH {
                return (self.game_status, GuessResult::IncorrectLength);
            }
            if self.guess_already_exists(guess_input) {
                return (self.game_status, GuessResult::DuplicateGuess);
            }
            if !self.in_dictionary(guess_input) {
                return (self.game_status, GuessResult::NotInDictionary);
            }

            let guess = self.build_guess(guess_input);
            self.guesses.push(guess);

            if guess_input == self.answer {
                self.game_status = GameStatus::Won;
                return (self.game_status, GuessResult::Valid);
            }
            if self.guesses.len() == GUESS_MAX {
                self.game_status = GameStatus::Lost;
            }

            (self.game_status, GuessResult::Valid)
        }
    }
}
// このguess_word_guiモジュールは、ここまでの src/gui.rs の内容になっています
mod guess_word_gui {
    use iced::alignment::Horizontal;
    use iced::{Application, Command, Element, Length, Text};

    #[derive(Debug, Default)]
    struct State;

    pub struct GuessWord {
        state: State,
    }

    impl Application for GuessWord {
        type Executor = iced::executor::Default;
        type Message = ();
        type Flags = ();

        fn new(_flags: ()) -> (GuessWord, Command<Self::Message>) {
            (
                GuessWord {
                    state: State::default(),
                },
                Command::none(),
            )
        }

        fn title(&self) -> String {
            "GuessWord".to_string()
        }

        fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
            Command::none()
        }

        fn view(&mut self) -> Element<Self::Message> {
            Text::new("GuessWord")
                .width(Length::Fill)
                .size(60)
                .color([0.5; 3])
                .horizontal_alignment(Horizontal::Center)
                .into()
        }
    }
}

use iced::{Application, Settings};
use crate::guess_word_gui::GuessWord;   // src/gui.rs の場合は mod gui;

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (400, 200);
    GuessWord::run(settings)            // src/gui.rs の場合は gui::GuessWord::run(settings);
}
