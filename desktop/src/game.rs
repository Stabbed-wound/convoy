use convoy::board::Board;
use convoy::tile::Tile;
use iced::widget::container::background;
use iced::widget::{column, container, row};
use iced::{color, Element, Fill, Shrink};

#[derive(Default)]
pub struct State {
    game: convoy::Game,
}

#[derive(Debug)]
pub enum Message {
    Board(BoardMessage),
}

impl State {
    #[allow(clippy::unused_self)]
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub const fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Element<Message> {
        container(view_board(self.game.board()).map(Message::Board))
            .padding([10, 0])
            .center_x(Shrink)
            .center_y(Fill)
            .into()
    }
}

#[derive(Debug)]
enum BoardMessage {
    Tile(usize, usize, TileMessage),
}

fn view_board(board: &Board) -> Element<BoardMessage> {
    column(board.rows().enumerate().map(|(row_index, tile_row)| {
        row(tile_row.iter().enumerate().map(|(col_index, tile)| {
            view_tile(tile, (row_index + col_index) % 2 == 0)
                .map(move |message| BoardMessage::Tile(row_index, col_index, message))
        }))
        .into()
    }))
    .into()
}

#[derive(Debug)]
enum TileMessage {}

fn view_tile(_tile: &Tile, light: bool) -> Element<TileMessage> {
    let style = background(color!(if light { 0xee_ee_d2 } else { 0x76_96_56 }));

    container("")
        .width(40)
        .height(40)
        .style(move |_| style)
        .into()
}
