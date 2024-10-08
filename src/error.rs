use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("NotFound: {reason:?}")]
    NotFound { reason: String },

    #[error("NotAuthorized: {reason:?}")]
    NotAuthorized { reason: String },

    #[error("NameExists: The name {name} is already registered")]
    NameExists { name: String },

    #[error("InsufficientFunds: Expected {exp_amount}")]
    InsufficientFunds { exp_amount: u128 },

    #[error("ValidationError: {reason:?}")]
    ValidationError { reason: String },

    #[error("TooManyRecords: Too many records requested. Maximum Limit is {limit}")]
    TooManyRecords { limit: u8 },
}

impl From<ContractError> for StdError {
    fn from(err: ContractError) -> Self {
        StdError::generic_err(err.to_string())
    }
}
