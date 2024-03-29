//! Error types

use {
    
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};


#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum VaultError {
   
    #[error("Invalid Authority")]
    InvalidAuthority,


    #[error("Derived key invalid")]
    InvalidID


    #[error("not a owner")]
    NotAOwner



    #[error("blacklisted")]
    BlackListed



    #[error("NotAnerror")]
    NotAnerror
}

impl PrintProgramError for VaultError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<VaultError> for ProgramError {
    fn from(e: VaultError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for VaultError {
    fn type_of() -> &'static str {
        "Vault Error"
    }
}
