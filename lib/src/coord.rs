use crate::constants::{BOARD_FILES, BOARD_RANKS, RANK_LETTERS};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Coord {
    pub rank: u8,
    pub file: u8,
}

impl Coord {
    #[must_use]
    pub const fn new(rank: u8, file: u8) -> Option<Self> {
        if rank > BOARD_RANKS || file > BOARD_FILES {
            return None;
        }

        Some(Self { rank, file })
    }

    #[must_use]
    pub const fn distance(&self, other: Self) -> u8 {
        self.rank.abs_diff(other.rank) + self.file.abs_diff(other.file)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            RANK_LETTERS
                .chars()
                .nth(self.rank as usize)
                .expect("Is within bounds"),
            self.file
        )
    }
}
