//! Record program
#![deny(missing_docs)]

pub mod borsh_utils;
mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

// Export current SDK types for downstream users building with a different SDK version
pub use solana_program;

solana_program::declare_id!("VRPLtk4k31bDL99mn1A5mE96CUUzQ9PnftEwf2LvMiG");
