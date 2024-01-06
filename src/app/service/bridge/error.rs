use thiserror::Error;

pub type BridgeEventResult<T> = Result<T, BridgeEventError>;

#[derive(Error, Debug)]
pub enum BridgeEventError {
    #[error("...")]
    BridgeFailed,
}