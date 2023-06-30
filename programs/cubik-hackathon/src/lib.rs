use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod cubik_hackathon {
    use super::*;

   pub fn hackathon_init(ctx: Context<HackathonInitContext>,counter:String) -> Result<()> {
        hackathon_init::handler(ctx,counter);

        Ok(())
    }

    pub fn crate_participant_nft(ctx: Context<MintPowNft>,name:String,symbol:String,metadata_url:String,counter: String) -> Result<()> {
        crate_participant_nft::handler(ctx,name,symbol,metadata_url,counter);
         Ok(())
    } 
   
}


