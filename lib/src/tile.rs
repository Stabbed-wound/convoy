use crate::pieces::Piece;
use crate::Player;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum TileType {
    #[default]
    Regular,
    Town,
    Baseline(Player),
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Tile {
    pub piece_option: Option<Piece>,
    pub tile_type: TileType,
}

impl Tile {
    #[must_use]
    pub const fn has_supplies(self, cur_player: Player) -> bool {
        match self.tile_type {
            TileType::Baseline(owner) => matches!(
                (owner, cur_player),
                (Player::P1, Player::P1) | (Player::P2, Player::P2)
            ),
            TileType::Town => true,
            TileType::Regular => false,
        }
    }

    #[must_use]
    pub const fn gives_income(self) -> bool {
        matches!(self.tile_type, TileType::Town)
    }

    #[must_use]
    pub const fn produces_troops(self, cur_player: Player) -> bool {
        match self.tile_type {
            TileType::Baseline(owner) => matches!(
                (owner, cur_player),
                (Player::P1, Player::P1) | (Player::P2, Player::P2)
            ),
            _ => false,
        }
    }
}
