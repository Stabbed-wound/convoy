mod game;

use iced::{
    alignment::Horizontal, application, widget::{column, container, text}, window,
    window::Position,
    Element,
    Fill,
    Shrink,
    Size,
};

fn main() -> iced::Result {
    application("Convoy", App::update, App::view)
        .window(window::Settings {
            size: Size {
                width: 600f32,
                height: 675f32,
            },
            position: Position::Centered,
            min_size: Some(Size {
                width: 600f32,
                height: 675f32,
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
        Element::<AppMessage>::from(
            column![
                text("App").size(20),
                container(self.game.view().map(AppMessage::Game))
                    .center_y(Fill)
                    .center_x(Shrink)
            ]
            .padding([10, 0])
            .spacing(10)
            .align_x(Horizontal::Center)
            .height(Fill)
            .width(Fill),
        )
        // .explain(iced::color!(0x77_77_77))
    }
}
