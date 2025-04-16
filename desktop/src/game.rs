use convoy::{board::Board, coord::Coord, coord::Move, pieces::PieceType, tile::Tile, Player};
use iced::{
    alignment::{Horizontal, Vertical}, color, font::Weight, widget::{button, column, container, container::background, rich_text, row, span, text},
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
}

#[derive(Clone, Debug)]
pub enum Message {
    ChangeActionMode(ActionMode),
    ChangePieceType(PieceType),
    EndTurn,
    TileClicked(usize, usize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActionMode {
    Move(Option<Coord>),
    Purchase(Option<PieceType>),
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
            }
            Message::ChangePieceType(new_type) => {
                if let ActionMode::Purchase(piece_type) = &mut self.action_mode {
                    match piece_type {
                        &mut Some(cur_type) if cur_type == new_type => *piece_type = None,
                        _ => *piece_type = Some(new_type),
                    }
                }
            }
            Message::EndTurn => {
                self.action_mode = ActionMode::default();
                self.game.end_turn();
            }
            Message::TileClicked(row, col) => match &mut self.action_mode {
                ActionMode::Move(piece_option) => {
                    let tile_coord = Coord::new(row, col)
                        .expect("A TileClicked message is always a valid coord");

                    match piece_option {
                        &mut Some(piece) if piece == tile_coord => *piece_option = None,
                        &mut Some(piece) => {
                            let _ = self.game.do_move(Move {
                                from: piece,
                                to: tile_coord,
                            });
                            *piece_option = None;
                        }
                        None => *piece_option = Some(tile_coord),
                    }
                }
                ActionMode::Purchase(piece_opt) => {
                    if let &mut Some(piece_type) = piece_opt {
                        let _ = self.game.do_purchase(
                            piece_type,
                            Coord::new(row, col)
                                .expect("A TileClicked message is always a valid coord"),
                        );
                        if self.game[self.game.cur_player()] < piece_type.cost() {
                            *piece_opt = None;
                        }
                    }
                }
                ActionMode::Battle => {}
            },
        }
    }

    pub fn view(&self) -> Element<Message> {
        let board = match self.action_mode {
            ActionMode::Move(piece_option) => {
                view_move_action_board(self.game.board(), piece_option, self.game.cur_player())
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

        let piece_selectors = container(
            column![
                row![
                    view_piece_selector(PieceType::Infantry, self),
                    view_piece_selector(PieceType::Convoy, self)
                ]
                .spacing(5),
                row![
                    view_piece_selector(PieceType::Artillery, self),
                    view_piece_selector(PieceType::Recon, self)
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
                        ActionMode::Purchase(None),
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
            container(button("End Turn").on_press(Message::EndTurn)).center_y(Fill);

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

fn view_move_action_board(
    board: &Board,
    selected_piece: Option<Coord>,
    cur_turn: Player,
) -> Element<Message> {
    let board_tile = |row, col, tile: &Tile| {
        let selectable = selected_piece.map_or_else(
            || {
                tile.piece_option
                    .is_some_and(|piece| piece.owner == cur_turn && !piece.exhausted)
            },
            |piece| {
                let coord = Coord::new(row, col).expect("row and col are always a valid Coord");

                piece == coord
                    || board
                        .get_moves(piece)
                        .expect("piece will always index a valid Piece")
                        .contains(&coord)
            },
        );

        view_tile(*tile, (row + col) % 2 == 0, selectable, selectable)
            .map(move |()| Message::TileClicked(row, col))
    };

    column(
        board
            .rows()
            .enumerate()
            .map(|(row_index, tile_row): (usize, &[Tile])| {
                row(tile_row
                    .iter()
                    .enumerate()
                    .map(|(col_index, tile)| board_tile(row_index, col_index, tile)))
                .into()
            }),
    )
    .into()
}

fn view_purchase_action_board(board: &Board, player: Player) -> Element<Message> {
    let tile_row = |(row_index, tile_row): (usize, &[Tile])| {
        row(tile_row.iter().enumerate().map(|(col_index, tile)| {
            view_tile(
                *tile,
                (row_index + col_index) % 2 == 0,
                tile.produces_troops(player),
                tile.produces_troops(player) && tile.piece_option.is_none(),
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
                .map_or_else(String::new, |piece| piece.piece_type.to_string()),
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

fn view_piece_selector(piece_type: PieceType, state: &State) -> Element<Message> {
    const BUTTON_SIZE: u16 = 40;

    let selector = button(
        text(piece_type.to_string())
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
    )
    .width(BUTTON_SIZE)
    .height(BUTTON_SIZE);

    let ActionMode::Purchase(current_type) = state.action_mode else {
        return selector.into();
    };

    selector
        .on_press_maybe(
            if state.game[state.game.cur_player()] >= piece_type.cost() {
                Some(Message::ChangePieceType(piece_type))
            } else {
                None
            },
        )
        .style(move |theme, status| {
            if current_type == Some(piece_type) {
                button::primary(theme, button::Status::Hovered)
            } else {
                button::primary(theme, status)
            }
        })
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
