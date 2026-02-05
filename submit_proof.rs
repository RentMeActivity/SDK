use anchor_lang::prelude::*;
use crate::state::Task;

pub fn submit_proof(ctx: Context<SubmitProof>, proof_uri: String) -> Result<()> {
    let task = &mut ctx.accounts.task;
    task.proof_uri = proof_uri;
    task.completed = true;
    Ok(())
}

#[derive(Accounts)]
pub struct SubmitProof<'info> {
    #[account(mut)]
    pub task: Account<'info, Task>,
    pub executor: Signer<'info>,
}
