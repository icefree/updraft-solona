use anchor_lang::prelude::*;

declare_id!("MmB7bRKF9bjCsZCZ2H33kabeLEsR4fe2xAMLjBgtFqt");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
