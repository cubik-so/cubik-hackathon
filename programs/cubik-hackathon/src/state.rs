use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct  Hackathon {
   pub authority: Pubkey,
   pub counter: String,
   pub bump: u8,
}
#[account]
#[derive(Default)]
pub struct  Participant {
   pub authority: Pubkey,
  pub  is_winner: bool,
  pub  bump: u8,

}