use convoy::pieces::PieceType;
use convoy::{board::Board, coord::Coord, tile::Tile, Command, Player};
use iced::alignment::Vertical;
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
    action_mode: ActionMode,
    selected_tile: Option<Coord>,
    selected_piece: Option<PieceType>,
}

#[derive(Clone, Debug)]
pub enum Message {
    ChangeActionMode(ActionMode),
    ChangeSelectedPiece(PieceType),
    GameCommand(Command),
    TileClicked(usize, usize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActionMode {
    Move(Option<Coord>),
    Purchase(PieceType),
    Battle,
}

impl Default for ActionMode {
    fn default() -> Self {
        Self::Move(None)
    }
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeActionMode(select_mode) => {
                self.action_mode = select_mode;
                self.selected_tile = None;
                self.selected_piece = None;
            }
            Message::ChangeSelectedPiece(piece_type) => {
                if let Some(selected_piece) = self.selected_piece {
                    if selected_piece == piece_type {
                        self.selected_piece = None;
                    } else {
                        self.selected_piece = Some(piece_type);
                    }
                } else {
                    self.selected_piece = Some(piece_type);
                }
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
        let board = match self.action_mode {
            ActionMode::Move(piece_option) => {
                view_move_action_board(self.game.board(), piece_option)
            }
            ActionMode::Purchase(_) => {
                view_purchase_action_board(self.game.board(), self.game.cur_player())
            }
            ActionMode::Battle => view_battle_action_board(self.game.board()),
        };

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
        .spacing(10)
        .height(Fill)
        .align_y(Vertical::Center);

        let piece_selector = |piece_type| {
            const BUTTON_SIZE: u16 = 40;

            button(
                text(match piece_type {
                    PieceType::Artillery => "A",
                    PieceType::Convoy => "C",
                    PieceType::Infantry => "I",
                    PieceType::Recon => "R",
                })
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center),
            )
            .on_press_maybe(if matches!(self.action_mode, ActionMode::Purchase(_)) {
                Some(Message::ChangeSelectedPiece(piece_type))
            } else {
                None
            })
            .width(BUTTON_SIZE)
            .height(BUTTON_SIZE)
        };

        let piece_selectors = container(
            column![
                row![
                    piece_selector(PieceType::Infantry),
                    piece_selector(PieceType::Convoy)
                ]
                .spacing(5),
                row![
                    piece_selector(PieceType::Artillery),
                    piece_selector(PieceType::Recon)
                ]
                .spacing(5)
            ]
            .spacing(5),
        )
        .center_y(Fill);

        let action_selectors = container(
            column![
                row![
                    view_action_selector(
                        ActionMode::Move(None),
                        matches!(self.action_mode, ActionMode::Move(_))
                    ),
                    view_action_selector(
                        ActionMode::Purchase(PieceType::Infantry),
                        matches!(self.action_mode, ActionMode::Purchase(_))
                    )
                ]
                .spacing(5),
                view_action_selector(
                    ActionMode::Battle,
                    matches!(self.action_mode, ActionMode::Battle)
                )
            ]
            .align_x(Horizontal::Center)
            .spacing(5),
        )
        .center_y(Fill);

        let end_turn_button =
            container(button("End Turn").on_press(Message::GameCommand(Command::EndTurn)))
                .center_y(Fill);

        let sidebar = container(
            column![players, piece_selectors, action_selectors, end_turn_button]
                .spacing(15)
                .align_x(Horizontal::Center)
                .height(Fill),
        )
        .padding(15)
        .center_y(Fill);

        row![board, sidebar]
            .width(Shrink)
            .height(Shrink)
            .padding(5)
            .into()
    }
}

fn view_move_action_board(board: &Board, selected_tile: Option<Coord>) -> Element<Message> {
    let tile_row = |(row_index, tile_row): (usize, &[Tile])| {
        row(tile_row.iter().enumerate().map(|(col_index, tile)| {
            view_tile(
                *tile,
                (row_index + col_index) % 2 == 0,
                selected_tile == Coord::new(row_index, col_index),
                tile.piece_option.is_some(),
            )
            .map(move |()| Message::TileClicked(row_index, col_index))
        }))
        .into()
    };

    column(board.rows().enumerate().map(tile_row)).into()
}

fn view_purchase_action_board(board: &Board, player: Player) -> Element<Message> {
    let tile_row = |(row_index, tile_row): (usize, &[Tile])| {
        row(tile_row.iter().enumerate().map(|(col_index, tile)| {
            view_tile(
                *tile,
                (row_index + col_index) % 2 == 0,
                tile.produces_troops(player),
                tile.produces_troops(player),
            )
            .map(move |()| Message::TileClicked(row_index, col_index))
        }))
        .into()
    };

    column(board.rows().enumerate().map(tile_row)).into()
}

fn view_battle_action_board(board: &Board) -> Element<Message> {
    let tile_row = |(row_index, tile_row): (usize, &[Tile])| {
        row(tile_row.iter().enumerate().map(|(col_index, tile)| {
            view_tile(*tile, (row_index + col_index) % 2 == 0, false, false)
                .map(move |()| Message::TileClicked(row_index, col_index))
        }))
        .into()
    };

    column(board.rows().enumerate().map(tile_row)).into()
}

fn view_tile(tile: Tile, light: bool, selected: bool, enabled: bool) -> Element<'static, ()> {
    const TILE_SIZE: u16 = 30;

    let background = color!(match (light, selected) {
        (_, true) => 0xba_ca_44,
        (true, _) => 0xee_ee_d2,
        (false, _) => 0x76_96_56,
    })
    .into();

    button(
        text(
            tile.piece_option
                .map_or("", |piece| match piece.piece_type {
                    PieceType::Artillery => "A",
                    PieceType::Convoy => "C",
                    PieceType::Infantry => "I",
                    PieceType::Recon => "R",
                }),
        )
        .center(),
    )
    .on_press_maybe(if enabled { Some(()) } else { None })
    .width(TILE_SIZE)
    .height(TILE_SIZE)
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

fn view_action_selector(action_mode: ActionMode, inactive: bool) -> Element<'static, Message> {
    button(
        text(match action_mode {
            ActionMode::Move(_) => "Move",
            ActionMode::Purchase(_) => "Purchase",
            ActionMode::Battle => "Battle",
        })
        .align_x(Horizontal::Center),
    )
    .on_press_maybe(if inactive {
        None
    } else {
        Some(Message::ChangeActionMode(action_mode))
    })
    .width(100)
    .into()
}
