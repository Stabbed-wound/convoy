use super::{ IsPiece, PieceColour };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Recon {
    colour: PieceColour,
}

impl IsPiece for Recon {
    fn get_cost(&self) -> u8 {
        3
    }

    fn get_power(&self) -> u8 {
        1
    }

    fn get_colour(&self) -> PieceColour {
        self.colour
    }
}
