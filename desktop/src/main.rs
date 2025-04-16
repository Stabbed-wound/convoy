mod game;

use iced::{Element, Fill, Size, application, widget::container, window, window::Position};

fn main() -> iced::Result {
    application("Convoy", App::update, App::view)
        .window(window::Settings {
            size: Size {
                width: 700f32,
                height: 700f32,
            },
            position: Position::Centered,
            min_size: Some(Size {
                width: 700f32,
                height: 700f32,
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
enum AppMessage {
    Game(game::Message),
}

impl App {
    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::Game(message) => self.game.update(message),
        }
    }

    pub fn view(&self) -> Element<AppMessage> {
        Element::<AppMessage>::from(container(self.game.view().map(AppMessage::Game)).center(Fill))
            .explain(iced::color!(0x77_77_77))
    }
}
