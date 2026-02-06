use anchor_lang::prelude::*;

use crate::state::Oracle;

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, constraint = oracle.owner == owner.key())]
    pub oracle: Account<'info, Oracle>,
}

pub fn update(ctx: Context<Update>, price: u64) -> Result<()> {
    ctx.accounts.oracle.price = price;
    Ok(())
}
