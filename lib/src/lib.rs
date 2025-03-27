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
    pub const fn player(&self, colour: PieceColour) -> &Player {
        match colour {
            PieceColour::Black => &self.players[0],
            PieceColour::White => &self.players[1],
        }
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
    #[must_use]
    const fn mut_player(&mut self, colour: PieceColour) -> &mut Player {
        match colour {
            PieceColour::Black => &mut self.players[0],
            PieceColour::White => &mut self.players[1],
        }
    }

    #[must_use]
    const fn mut_cur_player(&mut self) -> &mut Player {
        match self.cur_player {
            PieceColour::Black => &mut self.players[0],
            PieceColour::White => &mut self.players[1],
        }
    }

    #[must_use]
    const fn mut_other_player(&mut self) -> &mut Player {
        match self.cur_player {
            PieceColour::Black => &mut self.players[1],
            PieceColour::White => &mut self.players[0],
        }
    }
    
    const fn end_turn(&mut self) {
        self.cur_player = self.cur_player.opposite();
        self.mut_cur_player().money += self.cur_player().income;
    }
}
