use anchor_lang::prelude::*;

declare_id!("AnTYoq3LfYKz2xUadRL3Y1m2WYmFXyJDYTj73vieue65");

#[program]
pub mod simple_anchor_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
