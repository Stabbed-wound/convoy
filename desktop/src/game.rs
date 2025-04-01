use iced::Element;

#[derive(Default)]
pub struct State {
    game: convoy::Game,
}

#[derive(Debug)]
pub enum Message {}

impl State {
    #[allow(clippy::unused_self)]
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub const fn update(&mut self, _message: Message) {}

    #[allow(clippy::unused_self)]
    pub fn view(&self) -> Element<Message> {
        "Game".into()
    }
}
