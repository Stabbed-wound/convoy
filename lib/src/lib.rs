pub mod board;
pub mod constants;
pub mod coord;
pub mod pieces;
pub mod player;
pub mod tile;

use crate::pieces::PieceColour;
use crate::player::Player;
use board::Board;

#[derive(Clone, Debug)]
pub struct Game {
    board: Board,
    players: [Player; 2],
    cur_player: PieceColour,
}

impl Game {
    #[must_use]
    pub const fn board(&self) -> &Board {
        &self.board
    }

    #[must_use]
    pub const fn cur_player(&self) -> &Player {
        match self.cur_player {
            PieceColour::Black => &self.players[0],
            PieceColour::White => &self.players[1],
        }
    }

    #[must_use]
    pub const fn other_player(&self) -> &Player {
        match self.cur_player {
            PieceColour::Black => &self.players[1],
            PieceColour::White => &self.players[0],
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::default(),
            players: Default::default(),
            cur_player: PieceColour::White,
        }
    }
}

impl Game {
    fn mut_cur_player(&mut self) -> &mut Player {
        match self.cur_player {
            PieceColour::Black => &mut self.players[0],
            PieceColour::White => &mut self.players[1],
        }
    }

    fn mut_other_player(&mut self) -> &mut Player {
        match self.cur_player {
            PieceColour::Black => &mut self.players[1],
            PieceColour::White => &mut self.players[0],
        }
    }
}
