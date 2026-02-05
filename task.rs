use anchor_lang::prelude::*;

#[account]
pub struct Task {
    pub agent: Pubkey,
    pub executor: Pubkey,
    pub reward: u64,
    pub completed: bool,
    pub proof_uri: String,
}
