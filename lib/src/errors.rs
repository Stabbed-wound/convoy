use thiserror::Error;

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
pub enum CommandError {
    #[error(transparent)]
    Move(MoveError),
    #[error(transparent)]
    Purchase(PurchaseError),
    #[error(transparent)]
    Battle(BattleError),
}

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
#[error("")]
pub struct MoveError;

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
#[error("")]
pub struct PurchaseError;

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
#[error("")]
pub struct BattleError;
