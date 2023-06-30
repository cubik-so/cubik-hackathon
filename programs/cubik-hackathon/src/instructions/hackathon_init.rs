use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, system_program, sysvar::rent::Rent};

use crate::state::Hackathon;

#[derive(Accounts)]
#[instruction(counter: String)]
pub struct HackathonInitContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + 1 + 32 + 4,
        seeds=[b"hackathon".as_ref(),authority.key().as_ref(),counter.as_ref()],
        bump,
    )]
    pub hackathon_account: Account<'info, Hackathon>,

   

    // Misc Accounts
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<HackathonInitContext>,counter:String) -> Result<()> {
let hackathon_account =  &mut ctx.accounts.hackathon_account;
    hackathon_account.authority = *ctx.accounts.authority.key;
    hackathon_account.counter = counter;
    hackathon_account.bump = *ctx.bumps.get("hackathon_account").unwrap();
   
    Ok(())
}