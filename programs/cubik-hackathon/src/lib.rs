use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;
declare_id!("GCpmUwiZm2rX2X6VWtB1y7w2MMLvR8M2UnNTioYMdTyt");

#[program]
pub mod cubik_hackathon {
    use super::*;

   pub fn hackathon_init(ctx: Context<HackathonInitContext>,counter:u16) -> Result<()> {
        hackathon_init::handler(ctx,counter);

        Ok(())
    }

    pub fn create_participant_nft(ctx: Context<MintPowNft>,name:String,symbol:String,metadata_url:String,counter: u16,_hackathon_owner:Pubkey) -> Result<()> {
        create_participant_nft::handler(ctx,name,symbol,metadata_url,counter);
         Ok(())
    } 
   
}


