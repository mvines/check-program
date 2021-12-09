#![forbid(unsafe_code)]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
pub mod instruction;
pub mod processor;

solana_program::declare_id!("ZkTokenXHhH6t1juaWF74WLcfv4XoNocjXA6sPWHNg1"); // TODO: grind a `ZkToken...` keypair
