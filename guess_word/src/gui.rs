use guess_word::*;
use iced::alignment::{Alignment, Horizontal};
use iced::text_input::TextInput;
use iced::{Application, Column, Command, Element, Length, Row, Text};

#[derive(Debug, Default)]
struct State {
    input: iced::text_input::State,
    input_value: String,
    announce: String,
    guesses: Vec<GuessResultWidget>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Guess,
}

#[derive(Default)]
pub struct GuessWord {
    game: Game,
    state: State,
}

impl GuessWord {
    fn create_guess_result_widget(&mut self) {
        let word = self.game.guesses().last().unwrap();
        let mut widget = GuessResultWidget::new();
        widget.letters = word.letters().iter().map(|l| l.clone()).collect();
        self.state.guesses.push(widget);
    }
}

impl Application for GuessWord {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (GuessWord, Command<Self::Message>) {
        (GuessWord::default(), Command::none())
    }

    fn title(&self) -> String {
        "GuessWord".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::InputChanged(value) => {
                self.state.input_value = value;
            }
            Message::Guess => {
                let old_status = self.game.game_status();
                let (status, result) = self.game.guess(&self.state.input_value);
                match status {
                    GameStatus::Won => {
                        if old_status != status {
                            self.create_guess_result_widget();
                        }
                        self.state.announce = "You Win!".to_string();
                    }
                    GameStatus::Lost => {
                        if old_status != status {
                            self.create_guess_result_widget();
                        }
                        self.state.announce = format!("You Lost! (answer:{})", 
                            self.game.get_answer().unwrap());
                    }
                    GameStatus::InProgress => match result {
                        GuessResult::DuplicateGuess => {
                            self.state.announce = "Duplicate Guess".to_string()
                        }
                        GuessResult::IncorrectLength => {
                            self.state.announce = "Incorrect Length".to_string()
                        }
                        GuessResult::NotInDictionary => {
                            self.state.announce = "Invalid word".to_string()
                        }
                        GuessResult::Valid => {
                            self.create_guess_result_widget();
                            self.state.announce.clear();
                        }
                        _ => (),
                    },
                }

                self.state.input_value.clear();
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let title = Text::new("GuessWord")
            .width(Length::Fill)
            .size(60)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(Horizontal::Center);

        let announce = Text::new(&self.state.announce)
            .width(Length::Fill)
            .size(20)
            .color([0.0, 0.4, 1.0])
            .horizontal_alignment(Horizontal::Center);

        let input = TextInput::new(
            &mut self.state.input,
            "input word...",
            &mut self.state.input_value,
            Message::InputChanged,
        )
        .padding(15)
        .size(30)
        .on_submit(Message::Guess);

        let results: Element<_> = self
            .state
            .guesses
            .iter_mut()
            .fold(Column::new().spacing(20), |column, guess| {
                column.push(guess.view())
            })
            .into();

        let content = Column::new()
            .padding(40)
            .max_width(800)
            .spacing(20)
            .push(title)
            .push(announce)
            .push(input)
            .push(results);
        content.into()
    }
}

#[derive(Debug, Clone)]
struct GuessResultWidget {
    letters: Vec<GuessLetter>,
}

impl GuessResultWidget {
    fn new() -> Self {
        Self {
            letters: Vec::<GuessLetter>::with_capacity(GUESS_LENGTH),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let new_text = |label, size, color| {
            Text::new(label)
                .width(Length::Fill)
                .size(size)
                .color(color)
                .horizontal_alignment(Horizontal::Center)
        };
        self.letters
            .iter_mut()
            .fold(
                Row::new().spacing(20).align_items(Alignment::Center),
                |row, l| {
                    row.push(match l.accuracy {
                        HitAccuracy::InRightPlace => new_text(
                            l.letter.to_string(), 30, [1.0, 0.0, 0.0]),
                        HitAccuracy::InWord => new_text(
                            l.letter.to_string(), 30, [0.0, 0.5, 0.0]),
                        HitAccuracy::NotInWord => new_text(
                            l.letter.to_string(), 30, [0.9; 3]),
                    })
                },
            )
            .into()
    }
}
