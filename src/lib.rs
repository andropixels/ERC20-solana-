pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;


//add comments
#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;



pub use solana_program;
