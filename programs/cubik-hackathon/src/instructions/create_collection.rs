use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, system_program, sysvar::rent::Rent};

#[derive(Accounts)]
pub struct CreateCollectionContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

   

    // Misc Accounts
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateCollectionContext>) -> Result<()> {
    
   
    Ok(())
}