use anchor_lang::prelude::*;
use std::fmt;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum MarketplaceError {
    NameTooLong,
    RecordAlreadyInitialized,
}

impl fmt::Display for MarketplaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarketplaceError::NameTooLong => write!(f, "Name must be between 1 and 36 characters"),
            MarketplaceError::RecordAlreadyInitialized => write!(f, "Record is already initialized"),
        }
    }
}

impl From<MarketplaceError> for anchor_lang::error::Error {
    fn from(e: MarketplaceError) -> Self {
        anchor_lang::error::Error::from(ProgramError::Custom(e as u32))
    }
}