use anchor_lang::prelude::*;
use core::mem::size_of;

//import custom files
mod state;
use crate::state::PushCommStorageV3;



declare_id!("3F2sJzYnEQUt7J3MERtFCHKrz1VDQoFTg1gmZFxk9c86");

#[program]
pub mod push_comm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, 
        push_admin: Pubkey, 
        chain_name: String,
        chain_id: u64,
    ) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        storage.governance = push_admin;
        storage.push_channel_admin = push_admin;
        storage.chain_id = chain_id;
        storage.chain_name = chain_name;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(init,
        payer = user,
        space = size_of::<PushCommStorageV3>() + 8,
        seeds = [],
        bump)]
    pub storage: Account<'info, PushCommStorageV3>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

