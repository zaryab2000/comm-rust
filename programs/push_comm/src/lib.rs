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
        chain_id: u64,
    ) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        storage.governance = push_admin;
        storage.push_channel_admin = push_admin;
        storage.chain_id = chain_id;
        Ok(())
    }

/**
 * ADMIN FUNCTIONS
 */
    pub fn set_core_address(ctx: Context<SetCoreAddress>, 
        push_core_address: Pubkey,
        ) -> Result <()> {
            let storage = &mut ctx.accounts.storage;
            storage.push_core_address = push_core_address;
            Ok(())
        }
    
    pub fn set_governance_address(ctx: Context<SetGovernanceAddress>,
        governance: Pubkey,
    ) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        storage.governance = governance;
        Ok(())
    }

    pub fn set_push_token_address(ctx: Context<SetPushTokenAddress>,
        token_address: Pubkey,
    ) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        storage.push_core_address = token_address;
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

#[derive(Accounts)]
pub struct SetCoreAddress <'info> {
    #[account(mut, has_one = push_channel_admin @ PushCommError::Unauthorized)]
    pub storage: Account<'info, PushCommStorageV3>,

    #[account(signer)]
    pub push_channel_admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetGovernanceAddress <'info> {
    #[account(mut, has_one = governance @ PushCommError::Unauthorized)]
    pub storage: Account<'info, PushCommStorageV3>,

    #[account(signer)]
    pub governance: Signer<'info>,

}

#[derive(Accounts)]
pub struct SetPushTokenAddress <'info> {
    #[account(mut, has_one = push_channel_admin @ PushCommError::Unauthorized)]
    pub storage: Account<'info, PushCommStorageV3>,

    #[account(signer)]
    pub push_channel_admin: Signer<'info>,
}

// Error Handling
#[error_code]
pub enum PushCommError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid argument provided")]
    InvalidArgument,
    #[msg("Arithmetic operation failed")]
    ArithmeticError,
    // Add more errors as needed
}

