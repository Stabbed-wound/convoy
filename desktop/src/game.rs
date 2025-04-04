use convoy::{board::Board, coord::Coord, tile::Tile, Command, Player};
use iced::widget::text;
use iced::{
    alignment::Horizontal, color, font::Weight, widget::{button, column, container, container::background, rich_text, row, span},
    Element,
    Fill,
    Font,
    Shrink,
};
use std::fmt::Debug;

#[derive(Default)]
pub struct State {
    game: convoy::Game,
    select_mode: SelectMode,
    selected_tile: Option<Coord>,
}

#[derive(Clone, Debug)]
pub enum Message {
    ChangeSelectMode(SelectMode),
    GameCommand(Command),
    TileClicked(usize, usize),
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum SelectMode {
    #[default]
    Move,
    Purchase,
    Battle,
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeSelectMode(select_mode) => {
                self.select_mode = select_mode;
                self.selected_tile = None;
            }
            Message::GameCommand(command) => self
                .game
                .do_command(command)
                .expect("Not handling errors yet"),
            Message::TileClicked(row, col) => {
                let new = Coord::new(row, col);

                if self.selected_tile == new {
                    self.selected_tile = None;
                } else {
                    self.selected_tile = new;
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let board = view_board(self.game.board(), self.selected_tile);

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

        let info = container(column![players]).center_y(Fill);

        let mode_selectors = container(
            column![
                row![
                    view_mode_selector(SelectMode::Move, self.select_mode),
                    view_mode_selector(SelectMode::Purchase, self.select_mode)
                ]
                .spacing(5),
                view_mode_selector(SelectMode::Battle, self.select_mode)
            ]
            .align_x(Horizontal::Center)
            .spacing(5),
        )
        .center_y(Fill);

        let end_turn_button =
            container(button("End Turn").on_press(Message::GameCommand(Command::EndTurn)))
                .center_y(Fill);

        let controls = column![mode_selectors, end_turn_button]
            .align_x(Horizontal::Center)
            .spacing(5)
            .padding(5)
            .height(Fill);

        let info_and_controls = container(
            column![info, controls]
                .spacing(15)
                .align_x(Horizontal::Center)
                .height(Fill),
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

fn view_board(board: &Board, selected_tile: Option<Coord>) -> Element<Message> {
    let tile_row = |(row_index, tile_row): (usize, &[Tile])| {
        row(tile_row.iter().enumerate().map(|(col_index, tile)| {
            view_tile(
                *tile,
                (row_index + col_index) % 2 == 0,
                selected_tile == Coord::new(row_index, col_index),
            )
            .map(move |()| Message::TileClicked(row_index, col_index))
        }))
        .into()
    };

    column(board.rows().enumerate().map(tile_row)).into()
}

fn view_tile(tile: Tile, light: bool, selected: bool) -> Element<'static, ()> {
    let background = color!(match (light, selected) {
        (_, true) => 0xba_ca_44,
        (true, _) => 0xee_ee_d2,
        (false, _) => 0x76_96_56,
    })
    .into();

    button("")
        .on_press_maybe(tile.piece_option.map(|_| ()))
        .width(40)
        .height(40)
        .style(move |_, _| button::Style {
            background: Some(background),
            ..button::Style::default()
        })
        .into()
}

fn view_player(player: Player, money: u8, is_current: bool) -> Element<'static, Message> {
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

fn view_mode_selector(select_mode: SelectMode, current: SelectMode) -> Element<'static, Message> {
    button(
        text(match select_mode {
            SelectMode::Move => "Move",
            SelectMode::Purchase => "Purchase",
            SelectMode::Battle => "Battle",
        })
        .align_x(Horizontal::Center),
    )
    .on_press_maybe(if select_mode == current {
        None
    } else {
        Some(Message::ChangeSelectMode(select_mode))
    })
    .width(100)
    .into()
}
