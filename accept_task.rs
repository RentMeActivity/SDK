use anchor_lang::prelude::*;
use crate::state::Task;

pub fn accept_task(ctx: Context<AcceptTask>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    task.executor = ctx.accounts.executor.key();
    Ok(())
}

#[derive(Accounts)]
pub struct AcceptTask<'info> {
    #[account(mut)]
    pub task: Account<'info, Task>,
    pub executor: Signer<'info>,
}
