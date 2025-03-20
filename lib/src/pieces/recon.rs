use super::IsPieceVariant;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Recon {}

impl IsPieceVariant for Recon {
    fn cost(&self) -> u8 {
        3
    }

    fn power(&self) -> u8 {
        1
    }
}
