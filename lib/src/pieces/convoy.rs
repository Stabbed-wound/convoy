use super::IsPieceVariant;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Convoy {}

impl IsPieceVariant for Convoy {
    fn cost(&self) -> u8 {
        2
    }

    fn power(&self) -> u8 {
        0
    }
}
