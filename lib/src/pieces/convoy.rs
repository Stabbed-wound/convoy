use super::{ IsPiece, PieceColour };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Convoy {
    colour: PieceColour,
}

impl IsPiece for Convoy {
    fn get_cost(&self) -> u8 {
        2
    }

    fn get_power(&self) -> u8 {
        0
    }

    fn get_colour(&self) -> PieceColour {
        self.colour
    }
}
