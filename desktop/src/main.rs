mod game;

use iced::widget::column;
use iced::window::Position;
use iced::{application, window, Element, Size};

fn main() -> iced::Result {
    application("Convoy", App::update, App::view)
        .window(window::Settings {
            size: Size {
                width: 960f32,
                height: 540f32,
            },
            position: Position::Centered,
            min_size: Some(Size {
                width: 480f32,
                height: 270f32,
            }),
            ..window::Settings::default()
        })
        .run()
}

#[derive(Default)]
struct App {
    game: game::State,
}

#[derive(Debug)]
enum Message {
    GameMessage(game::Message),
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::GameMessage(message) => self.game.update(message),
        }
    }

    pub fn view(&self) -> Element<Message> {
        column!["App", self.game.view().map(Message::GameMessage)].into()
    }
}
