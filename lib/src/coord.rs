use crate::constants::{ BOARD_X, BOARD_Y };

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Coord {
    pub rank: u8,
    pub file: u8,
}

impl Coord {
    #[must_use]
    pub const fn new(rank: u8, file: u8) -> Option<Self> {
        if rank > BOARD_Y || file > BOARD_X {
            return None;
        }

        Some(Self { rank, file })
    }
}
