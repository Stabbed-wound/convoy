use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CommandError {
    #[error(transparent)]
    Move(#[from]MoveError),
    #[error(transparent)]
    Purchase(#[from]PurchaseError),
    #[error(transparent)]
    Battle(#[from]BattleError),
}

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[error("Move Error")]
pub struct MoveError;

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[error("Purchase Error")]
pub struct PurchaseError;

#[derive(Copy, Clone, Debug, Error, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[error("Battle Error")]
pub struct BattleError;
