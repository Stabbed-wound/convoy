pub mod board;
pub mod constants;
pub mod coord;
pub mod pieces;
pub mod player;
pub mod tile;

use crate::pieces::PieceColour;
use crate::player::Player;
use board::Board;

#[derive(Clone, Debug, Default)]
pub struct Game {
    board: Board,
    players: [Player; 2],
}

impl Game {
    #[must_use]
    pub const fn board(&self) -> &Board {
        &self.board
    }

    #[must_use]
    pub const fn get_player(&self, colour: PieceColour) -> &Player {
        match colour {
            PieceColour::Black => &self.players[0],
            PieceColour::White => &self.players[1],
        }
    }
}

impl Game {
    fn get_mut_player(&mut self, colour: PieceColour) -> &mut Player {
        match colour {
            PieceColour::Black => &mut self.players[0],
            PieceColour::White => &mut self.players[1],
        }
    }
}
