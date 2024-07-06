use anchor_lang::prelude::*;

declare_id!("3eEobEfomwPAfZBtMc7EQ9bMdf9d8kBarmLfuxRkM9Pf");

#[program]
pub mod token_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
