use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self,system_program,program::{invoke,invoke_signed}, sysvar::rent::Rent,};
use anchor_spl::token::{self,MintTo,Token,Mint,Transfer, TokenAccount,Approve};
use anchor_spl::associated_token::{self,AssociatedToken};
use mpl_token_metadata::state::{Creator, Collection};
use mpl_token_metadata::{instruction as token_instruction, ID as TOKEN_METADATA_ID};
use mpl_token_metadata::{instruction::{create_master_edition_v3,create_metadata_accounts_v3,update_metadata_accounts_v2,freeze_delegated_account}};

use crate::state::Hackathon;

#[derive(Accounts)]
#[instruction(counter:u16)]
pub struct CreateCollection<'info> {
   

    #[account(mut)]
    pub authority: Signer<'info>,


       #[account(
        init,
        payer = authority,
        space = 8 + 1 + 32 + 8,
        seeds=[b"hackathon".as_ref(),authority.key().as_ref(),counter.to_le_bytes().as_ref()],
        bump,
    )]
    pub hackathon_account: Box<Account<'info, Hackathon>>,
 

    #[account(
        init,
        payer = authority,
        seeds = [b"collection".as_ref(),authority.key().as_ref(),counter.to_le_bytes().as_ref()],
        mint::decimals = 0,
        mint::authority = hackathon_account,
        mint::freeze_authority = hackathon_account,
        bump,
    )]
    pub mint:Box<Account<'info, Mint>>,



    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = hackathon_account,
    )]
    pub collection_ata: Box<Account<'info, TokenAccount>>,


    
   /// CHECK: Program ID for CPI No Danger
    #[account(address = mpl_token_metadata::id())]
    pub mpl_program: AccountInfo<'info>,

    /// CHECK: Used in CPI So no Harm
    #[account(mut)]
    pub metadata: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: Used in CPI So no Harm
    pub master_edition: AccountInfo<'info>,


    // Misc Accounts
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info,System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateCollection>,counter:u16,name:String,symbol:String,metadata_url:String) -> Result<()> {
   

      let seller_fee_basis_points:u16= 10000;
        let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info().clone(),
        to: ctx.accounts.collection_ata.to_account_info().clone(), // ata
        authority: ctx.accounts.authority.to_account_info().clone(),
    };
    
   let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, 1)?;

  let creators = vec![Creator {
        address: *ctx.accounts.authority.key,
        verified: false,
        share: 100,
    }];
    let metadata_ix = create_metadata_accounts_v3(
        ctx.accounts.mpl_program.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.authority.key(),
        ctx.accounts.authority.key(),
        ctx.accounts.authority.key(),
        name,
        symbol,
        metadata_url,
        Some(creators),
        seller_fee_basis_points,
        true,
        true,
        None,
        None,
        None
      );

      invoke(
        &metadata_ix,
        &[
            ctx.accounts.metadata.clone(),
            ctx.accounts.mint.to_account_info().clone(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.mpl_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    let ix = create_master_edition_v3(
        *ctx.accounts.mpl_program.key,
        *ctx.accounts.master_edition.key,
        ctx.accounts.mint.key(),
        ctx.accounts.authority.key(),
        ctx.accounts.authority.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.authority.key(),
        Some(0),
    );

    invoke(
        &ix,
        &[
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mpl_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    Ok(())
}