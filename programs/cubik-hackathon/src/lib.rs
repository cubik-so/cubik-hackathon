use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;
declare_id!("DQDrRfiaqSzbSJCL9BMzPd6TfgLmDHxCEQDCrjoK9jCF");

#[program]
pub mod cubik_hackathon {
    use super::*;

   pub fn hackathon_init(ctx: Context<HackathonInitContext>,counter:u16) -> Result<()> {
        hackathon_init::handler(ctx,counter);

        Ok(())
    }

    pub fn create_participant_nft(ctx: Context<MintPowNft>,counter:u16,hackathon_account_authority:Pubkey,name:String,symbol:String,metadata_url:String) -> Result<()> {
        create_participant_nft::handler(ctx,counter,hackathon_account_authority,name,symbol,metadata_url);
         Ok(())
    } 

    pub fn  participant(ctx: Context<TestMint>,name:String,symbol:String,metadata_url:String,counter: u16,create_key:Pubkey) -> Result<()> {
        participant::handler(ctx,name,symbol,metadata_url,counter,create_key);
         Ok(())
    } 

    pub fn create_collection(ctx: Context<CreateCollection>,counter:u16,name:String,symbol:String,metadata_url:String) -> Result<()> {
        create_collection::handler(ctx,counter,name,symbol,metadata_url);
         Ok(())
    }
   
}


