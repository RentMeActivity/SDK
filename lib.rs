use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("RentMe1111111111111111111111111111111111");

#[program]
pub mod rentme {
    use super::*;

    pub fn create_task(ctx: Context<CreateTask>, description: String, reward: u64) -> Result<()> {
        instructions::create_task(ctx, description, reward)
    }

    pub fn accept_task(ctx: Context<AcceptTask>) -> Result<()> {
        instructions::accept_task(ctx)
    }

    pub fn submit_proof(ctx: Context<SubmitProof>, proof_uri: String) -> Result<()> {
        instructions::submit_proof(ctx, proof_uri)
    }

    pub fn release_payment(ctx: Context<ReleasePayment>) -> Result<()> {
        instructions::release_payment(ctx)
    }
}
