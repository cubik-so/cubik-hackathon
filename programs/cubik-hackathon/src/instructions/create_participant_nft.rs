use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self,system_program,program::{invoke,invoke_signed}, sysvar::rent::Rent,};
use anchor_spl::token::{self,MintTo,Token,Mint,Transfer, TokenAccount,Approve};
use anchor_spl::associated_token::{self,AssociatedToken};
// use mpl_token_metadata::{self};
use crate::state::*;
use mpl_token_metadata::state::Creator;
use mpl_token_metadata::{instruction as token_instruction, ID as TOKEN_METADATA_ID};
use mpl_token_metadata::{instruction::{create_master_edition_v3,create_metadata_accounts_v3,update_metadata_accounts_v2,freeze_delegated_account}};

#[derive(Accounts)]
#[instruction(_counter: u16)]
pub struct MintPowNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer=authority,
        seeds=[b"participant".as_ref(), _counter.to_le_bytes().as_ref(),authority.key().as_ref()],
        bump,
        space=8+1+32+4,
    )]
    pub participant_account: Account<'info, Participant>,

     

    #[account(init, payer = authority, associated_token::mint = mint, associated_token::authority = authority)]
    pub pow_nft_ata: Box<Account<'info, TokenAccount>>,

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


pub fn handler(ctx: Context<MintPowNft>,name:String,symbol:String,metadata_url:String,_counter: u16,) -> Result<()> {
    
    let participant_account = &mut ctx.accounts.participant_account;
    participant_account.authority = ctx.accounts.authority.key();
    participant_account.is_winner = false;
    participant_account.bump = *ctx.bumps.get("participant_account").unwrap();


    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info().clone(),
        to: ctx.accounts.pow_nft_ata.to_account_info().clone(),
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

    let seller_fee_basis_points:u16= 10000;

    
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


    let update_ix = update_metadata_accounts_v2(
        ctx.accounts.mpl_program.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.authority.key(),
        Some(ctx.accounts.participant_account.key()),
        None,
        Some(true),
        Some(true)     
    );

    invoke(
        &update_ix,
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.authority.to_account_info()
        ]
    )?;

     // take back the authority
    let cpi_accounts = Approve {
        to: ctx.accounts.pow_nft_ata.to_account_info(),
        delegate: ctx.accounts.participant_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info()
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    
    token::approve(cpi_context, 1)?;


    let authority_participant = ctx.accounts.authority.key();
    let binding = _counter.to_le_bytes();
    
    let participant_account_seeds = &[
        "participant".as_bytes(),
        binding.as_ref(),
        authority_participant.as_ref(), 
        &[ctx.accounts.participant_account.bump]
    ];

     // freeze Talent ATA
     invoke_signed(
        &freeze_delegated_account(
            *ctx.accounts.mpl_program.key,
            ctx.accounts.participant_account.key(),
            ctx.accounts.pow_nft_ata.key(),
            *ctx.accounts.master_edition.key,
            ctx.accounts.mint.key(),
        ),
        &[
            ctx.accounts.participant_account.to_account_info(),
            ctx.accounts.pow_nft_ata.to_account_info(),
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
        ],
        &[participant_account_seeds],
    )?;


    Ok(())
}