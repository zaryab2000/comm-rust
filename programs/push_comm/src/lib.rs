use anchor_lang::prelude::*;
//import custom files
mod state;


declare_id!("3F2sJzYnEQUt7J3MERtFCHKrz1VDQoFTg1gmZFxk9c86");

#[program]
pub mod push_comm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
