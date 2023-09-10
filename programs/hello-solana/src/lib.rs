use anchor_lang::prelude::*;

declare_id!("3XVtSTEXibQ1sEt2BM1VrrLyG1vKbtpwivZkhbUeR2Jk");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn hello(_ctx: Context<Hello>) -> Result<()> {
        msg!("Hello there!");

        msg!("Program ID is: {}", &id());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}
