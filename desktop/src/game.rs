use convoy::board::Board;
use convoy::constants::{BOARD_FILES, BOARD_RANKS};
use convoy::tile::Tile;
use convoy::{Command, Player};
use iced::alignment::Horizontal;
use iced::font::Weight;
use iced::widget::container::background;
use iced::widget::{button, column, container, rich_text, row, span};
use iced::{color, Element, Fill, Font, Shrink};

#[derive(Default)]
pub struct State {
    game: convoy::Game,
    selected_tiles: [[bool; BOARD_FILES as usize]; BOARD_RANKS as usize],
}

#[derive(Clone, Debug)]
pub enum Message {
    TileClicked(usize, usize),
    GameCommand(Command),
}

impl State {
    #[allow(clippy::needless_pass_by_value)]
    pub fn update(&mut self, message: Message) {
        match message {
            Message::TileClicked(row, col) => {
                self.selected_tiles[row][col] = !self.selected_tiles[row][col];
            }
            Message::GameCommand(command) => self
                .game
                .do_command(command)
                .expect("Not handling errors yet"),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let board = view_board(self.game.board(), &self.selected_tiles);

        let players = row![
            view_player(
                Player::P1,
                self.game[Player::P1],
                self.game.cur_player() == Player::P1
            ),
            view_player(
                Player::P2,
                self.game[Player::P2],
                self.game.cur_player() == Player::P2
            )
        ]
        .spacing(10);

        let info = column![players];

        let end_turn_button = button("End Turn").on_press(Message::GameCommand(Command::EndTurn));

        let controls = column![end_turn_button];

        let info_and_controls = container(
            column![info, controls]
                .spacing(15)
                .align_x(Horizontal::Center),
        )
        .padding(15)
        .center_y(Fill);

        row![board, info_and_controls]
            .width(Shrink)
            .height(Shrink)
            .padding(5)
            .into()
    }
}

fn view_board<'a>(
    board: &'a Board,
    selected_tiles: &[[bool; BOARD_FILES as usize]; BOARD_RANKS as usize],
) -> Element<'a, Message> {
    column(board.rows().enumerate().map(|(row_index, tile_row)| {
        row(tile_row.iter().enumerate().map(|(col_index, tile)| {
            view_tile(
                tile,
                (row_index + col_index) % 2 == 0,
                selected_tiles[row_index][col_index],
            )
            .map(move |()| Message::TileClicked(row_index, col_index))
        }))
        .into()
    }))
    .into()
}

fn view_tile(_tile: &Tile, light: bool, selected: bool) -> Element<()> {
    let background = color!(match (light, selected) {
        (_, true) => 0xff_00_00,
        (true, _) => 0xee_ee_d2,
        (false, _) => 0x76_96_56,
    })
    .into();

    button("")
        .on_press(())
        .width(40)
        .height(40)
        .style(move |_, _| button::Style {
            background: Some(background),
            ..button::Style::default()
        })
        .into()
}

fn view_player<'a>(player: Player, money: u8, is_current: bool) -> Element<'a, Message> {
    let player_text = rich_text![
        span(match player {
            Player::P1 => "Player 1",
            Player::P2 => "Player 2",
        })
        .font_maybe(if is_current {
            Some(Font {
                weight: Weight::Bold,
                ..Font::default()
            })
        } else {
            None
        })
    ]
    .width(70)
    .align_x(Horizontal::Center);

    container(column![player_text, rich_text![span(money)]].align_x(Horizontal::Center))
        .padding(5)
        .style(|_| background(color!(0x99_99_99)))
        .into()
}
