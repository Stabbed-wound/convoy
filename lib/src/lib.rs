pub mod board;
pub mod constants;
pub mod coord;
pub mod pieces;
pub mod tile;

use board::Board;
use coord::{Coord, Move};
use pieces::{Piece, PieceType};
use std::ops::Index;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Player {
    #[default]
    P1,
    P2,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Command {
    Purchase(PieceType, Coord),
    Move(Move),
    Battle {
        attacker_moves: Vec<Move>,
        defenders: Vec<Coord>,
        target: Coord,
    },
    #[default]
    EndTurn,
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[expect(
        dead_code,
        reason = "Generic error should be removed when all uses are replaced"
    )]
    #[error("Generic error")]
    Error,
}

#[derive(Clone, Debug, Default)]
pub struct Game {
    board: Board,
    player_money: [u8; 2],
    cur_player: Player,
}

impl Game {
    #[must_use]
    pub const fn board(&self) -> &Board {
        &self.board
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
    pub fn do_command(&mut self, command: Command) -> Result<(), CommandError> {
        match command {
            Command::Purchase(piece_type, coord) => self.do_purchase(piece_type, coord),
            Command::Move(Move { from, to }) => self.do_move(from, to),
            Command::Battle {
                attacker_moves,
                defenders,
                target,
            } => self.do_battle(attacker_moves, defenders, target),
            Command::EndTurn => {
                self.do_resupply();
                self.do_upkeep();
                Ok(())
            }
        }
    }

    fn do_purchase(&mut self, piece_type: PieceType, coord: Coord) -> Result<(), CommandError> {
        if piece_type.cost() > self[self.cur_player] {
            return Err(CommandError::Error);
        }

        if !self.board[coord].produces_troops() {
            return Err(CommandError::Error);
        }

        if self.board[coord].piece_option.is_some() {
            return Err(CommandError::Error);
        }

        *self.index_mut(self.cur_player) -= piece_type.cost();
        self.board[coord].piece_option = Some(Piece::new(self.cur_player, piece_type));

        Ok(())
    }

    fn do_move(&mut self, from: Coord, to: Coord) -> Result<(), CommandError> {
        let piece = self.board[from].piece_option.ok_or(CommandError::Error)?;

        if !piece.get_moves(from, &self.board).contains(&to) {
            return Err(CommandError::Error);
        }

        self.board[from].piece_option = None;
        self.board[to].piece_option = Some(piece);

        Ok(())
    }

    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::unused_self)]
    #[allow(clippy::needless_pass_by_ref_mut)]
    fn do_battle(
        &mut self,
        _attacker_moves: Vec<Move>,
        _defenders: Vec<Coord>,
        _target: Coord,
    ) -> Result<(), CommandError> {
        Ok(())
    }

    fn do_upkeep(&mut self) {
        *self.index_mut(self.cur_player) = self
            .board
            .iter()
            .filter(|tile| {
                tile.piece_option.is_some_and(|piece| {
                    piece.piece_type == PieceType::Convoy && tile.gives_income()
                })
            })
            .count()
            .try_into()
            .expect("There are no more than 256 towns");

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
        /* get log_net placeholder */
        let () = ();

        self.cur_player = match self.cur_player {
            Player::P1 => Player::P1,
            Player::P2 => Player::P2,
        };
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
