pub mod board;
pub mod constants;
pub mod coord;
pub mod pieces;
pub mod tile;

use board::Board;

#[derive(Clone, Debug, Default)]
pub struct Game {
    board: Board,
}

impl Game {
    #[must_use]
    pub const fn board(&self) -> &Board {
        &self.board
    }
}
