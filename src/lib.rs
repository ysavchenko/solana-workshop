pub mod processor;
pub mod instruction;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

solana_program::declare_id!("F11Gib24ncSwhesgak5S9JnvvCeAxEhbaZTeyjGPUACa");