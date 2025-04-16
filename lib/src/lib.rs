pub mod board;
pub mod constants;
pub mod coord;
mod errors;
pub mod pieces;
pub mod tile;

use board::Board;
use coord::{Coord, Move};
pub use errors::{BattleError, CommandError, MoveError, PurchaseError};
use pieces::{Piece, PieceType};
use std::ops::Index;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Player {
    #[default]
    P1,
    P2,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Action {
    Command(Command),
    EndTurn,
}

#[must_use]
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ActionOutcome {
    Ongoing(Box<Game>),
    Draw,
    Winner(Player),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Command {
    Move(Move),
    Purchase(PieceType, Coord),
    Battle {
        attack_commands: Vec<AttackCommand>,
        defense_commands: Vec<DefenseCommand>,
        target: Coord,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AttackCommand {
    Attack(Coord),
    MoveAttack(Move),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DefenseCommand {
    Defend(Coord),
    Retreat(Move),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Game {
    board: Board,
    player_money: [u8; 2],
    cur_player: Player,
}

impl Game {
    #[must_use]
    pub fn new() -> Box<Self> {
        Box::new(Self::default())
    }

    #[must_use]
    pub const fn board(&self) -> &Board {
        &self.board
    }

    #[must_use]
    pub const fn cur_player(&self) -> Player {
        self.cur_player
    }
}

impl Game {
    ///
    ///
    /// # Arguments
    ///
    /// *
    ///
    /// returns:
    ///
    /// # Errors
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn take_action(
        mut self: Box<Self>,
        action: Action,
    ) -> Result<ActionOutcome, (Box<Self>, CommandError)> {
        match action {
            Action::Command(command) => match self.do_command(command) {
                Ok(()) => Ok(ActionOutcome::Ongoing(self)),
                Err(err) => Err((self, err)),
            },
            Action::EndTurn => Ok(self.end_turn()),
        }
    }

    ///
    ///
    /// # Arguments
    ///
    /// *
    ///
    /// returns:
    ///
    /// # Errors
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn do_command(&mut self, command: Command) -> Result<(), CommandError> {
        match command {
            Command::Move(r#move) => self.do_move(r#move).map_err(CommandError::Move),
            Command::Purchase(piece_type, coord) => self
                .do_purchase(piece_type, coord)
                .map_err(CommandError::Purchase),
            Command::Battle {
                attack_commands,
                defense_commands,
                target,
            } => self
                .do_battle(attack_commands, defense_commands, target)
                .map_err(CommandError::Battle),
        }
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `command`:
    ///
    /// returns: Result<(), `CommandError`>
    ///
    /// # Errors
    ///
    /// Will return Err if `command` is not possible
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn do_move(&mut self, Move { from, to }: Move) -> Result<(), MoveError> {
        if !self
            .board
            .get_moves(from)
            .is_some_and(|moves| moves.contains(&to))
        {
            return Err(MoveError);
        }

        self.board[to].piece_option = self.board[from].piece_option.map(|piece| Piece {
            exhausted: true,
            ..piece
        });
        self.board[from].piece_option = None;

        Ok(())
    }

    ///
    ///
    /// # Arguments
    ///
    /// *
    ///
    /// returns:
    ///
    /// # Errors
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn do_purchase(
        &mut self,
        piece_type: PieceType,
        coord: Coord,
    ) -> Result<(), PurchaseError> {
        if piece_type.cost() > self[self.cur_player] {
            return Err(PurchaseError);
        }

        if !self.board[coord].produces_troops(self.cur_player) {
            return Err(PurchaseError);
        }

        if self.board[coord].piece_option.is_some() {
            return Err(PurchaseError);
        }

        *self.index_mut(self.cur_player) -= piece_type.cost();
        self.board[coord].piece_option = Some(Piece::new(self.cur_player, piece_type));

        Ok(())
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `command`:
    ///
    /// returns: Result<(), `CommandError`>
    ///
    /// # Errors
    ///
    /// Will return Err if `command` is not possible
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn do_battle(
        &mut self,
        _attack_commands: Vec<AttackCommand>,
        _defense_commands: Vec<DefenseCommand>,
        _target: Coord,
    ) -> Result<(), BattleError> {
        Err(BattleError)
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `command`:
    ///
    /// returns: Result<(), `CommandError`>
    ///
    /// # Errors
    ///
    /// Will return Err if `command` is not possible
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn end_turn(mut self: Box<Self>) -> ActionOutcome {
        self.do_resupply();
        self.do_upkeep();

        ActionOutcome::Ongoing(self)
    }

    fn do_upkeep(&mut self) {
        *self.index_mut(self.cur_player) += u8::try_from(
            self.board
                .iter()
                .filter(|tile| {
                    tile.piece_option.is_some_and(|piece| {
                        piece.piece_type == PieceType::Convoy && tile.gives_income()
                    })
                })
                .count(),
        )
        .expect("There are no more than 256 towns")
            + 2;

        self.board
            .iter_mut()
            .filter_map(|tile| tile.piece_option.as_mut())
            .for_each(|piece| {
                if piece.owner == self.cur_player {
                    piece.exhausted = false;
                }
            });
    }

    fn do_resupply(&mut self) {
        // TODO calculate logistics network

        self.cur_player = match self.cur_player {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        };
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::new(),
            player_money: [3; 2],
            cur_player: Player::P1,
        }
    }
}

impl Index<Player> for Game {
    type Output = u8;

    fn index(&self, index: Player) -> &Self::Output {
        match index {
            Player::P1 => &self.player_money[0],
            Player::P2 => &self.player_money[1],
        }
    }
}

impl Game {
    const fn index_mut(&mut self, index: Player) -> &mut <Self as Index<Player>>::Output {
        match index {
            Player::P1 => &mut self.player_money[0],
            Player::P2 => &mut self.player_money[1],
        }
    }
}
