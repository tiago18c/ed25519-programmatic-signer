#![no_std]

pub mod error;
pub mod instruction;
pub mod state;

solana_address::declare_id!("NoncFMHL3z7njAbTPMQfscuyg2efjVADwfrhffgBv2h");

#[cfg(feature = "codama")]
codama_macros::codama_program!(name = "nonce");
