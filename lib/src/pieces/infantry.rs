use super::{ IsPiece, PieceColour };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Infantry {
    colour: PieceColour,
}

impl IsPiece for Infantry {
    fn get_cost(&self) -> u8 {
        3
    }

    fn get_power(&self) -> u8 {
        2
    }

    fn get_colour(&self) -> PieceColour {
        self.colour
    }
}
