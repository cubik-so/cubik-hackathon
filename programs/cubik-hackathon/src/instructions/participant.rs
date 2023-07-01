use anchor_lang::prelude::*;
use crate::state::*;
use anchor_lang::solana_program::{self,system_program};

#[derive(Accounts)]
#[instruction(_create_key: Pubkey)]
pub struct TestMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

  
    #[account(
        init,
        payer = authority,
        seeds = [b"hack",authority.key().as_ref(),&_create_key.as_ref()],
        space = 8+1+32+32,
        bump,
    )]
    pub participant_account: Box<Account<'info, Participant>>,

    
    #[account(address = system_program::ID)]
    pub system_program: Program<'info,System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}


pub fn handler(ctx: Context<TestMint>,name:String,symbol:String,metadata_url:String,_counter: u16,_create_key:Pubkey) -> Result<()> {
    
    let participant_account = &mut ctx.accounts.participant_account;
    participant_account.authority = ctx.accounts.authority.key();
    participant_account.is_winner = false;
    participant_account.bump = *ctx.bumps.get("participant_account").unwrap();

    msg!("NFT Minting");

    Ok(())
}