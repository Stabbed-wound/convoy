use crate::pieces::Piece;

#[derive(Copy, Clone, Debug, Default)]
pub struct Tile {
    has_supplies: bool,
    piece: Option<Piece>,
}

impl Tile {
    #[must_use] pub const fn has_supplies(self) -> bool {
        self.has_supplies
    }
    
    #[must_use] pub const fn piece(&self) -> Option<&Piece> {
        self.piece.as_ref()
    }
}