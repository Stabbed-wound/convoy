use super::IsPieceVariant;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Infantry {}

impl IsPieceVariant for Infantry {
    fn cost(&self) -> u8 {
        3
    }

    fn power(&self) -> u8 {
        2
    }
}
