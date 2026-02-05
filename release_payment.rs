use anchor_lang::prelude::*;
use crate::state::Task;

pub fn release_payment(ctx: Context<ReleasePayment>) -> Result<()> {
    let task = &ctx.accounts.task;
    require!(task.completed, ErrorCode::TaskNotCompleted);
    Ok(())
}

#[derive(Accounts)]
pub struct ReleasePayment<'info> {
    pub task: Account<'info, Task>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Task not completed")]
    TaskNotCompleted,
}
