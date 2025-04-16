use thiserror::Error;

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
pub enum CommandError {
    #[error(transparent)]
    Move(#[from]MoveError),
    #[error(transparent)]
    Purchase(#[from]PurchaseError),
    #[error(transparent)]
    Battle(#[from]BattleError),
}

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
#[error("Move Error")]
pub struct MoveError;

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
#[error("Purchase Error")]
pub struct PurchaseError;

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
#[error("Battle Error")]
pub struct BattleError;
