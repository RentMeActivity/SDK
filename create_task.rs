use anchor_lang::prelude::*;
use crate::state::Task;

pub fn create_task(
    ctx: Context<CreateTask>,
    description: String,
    reward: u64,
) -> Result<()> {
    let task = &mut ctx.accounts.task;
    task.agent = ctx.accounts.agent.key();
    task.executor = Pubkey::default();
    task.reward = reward;
    task.completed = false;
    task.proof_uri = description;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTask<'info> {
    #[account(init, payer = agent, space = 8 + 128)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub agent: Signer<'info>,
    pub system_program: Program<'info, System>,
}
