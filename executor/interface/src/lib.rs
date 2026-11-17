#![no_std]

pub mod error;
pub mod instruction;
pub mod pda;

solana_address::declare_id!("ExeXHWU2yrgUrKQg4QcjqnniW6rWYvqBVP36cq5uwTSp");

#[cfg(feature = "codama")]
codama_macros::codama_program!(name = "messageExecutor");
