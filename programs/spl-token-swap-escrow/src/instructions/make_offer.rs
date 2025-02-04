use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use crate::{OfferState, ANCHOR_DISCRIMINATOR};

use super::transfer_token;

#[derive(Accounts)]
#[instruction(id:u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program=token_program,
    )]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program=token_program,
    )]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
    mut,
    associated_token::mint=token_mint_a,
    associated_token::authority=maker,
    associated_token::token_program=token_program,   
    )]
    pub maker_token_account_a:Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        init,
        payer=maker,
        space=ANCHOR_DISCRIMINATOR + OfferState::INIT_SPACE,
        seeds=[
            b"offer",
            maker.key().as_ref(),
            id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub offer:Account<'info,OfferState>,

    #[account(
        init,
        payer=maker,
        associated_token::mint=token_mint_a,
        associated_token::authority=offer,
        associated_token::token_program=token_program,
    )]
    pub vault:InterfaceAccount<'info,TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>,
}

pub fn send_offered_tokens_to_vault(
    ctx: &Context<MakeOffer>,
    token_a_offered_amount:u64,
) -> Result<()> {
    transfer_token(
        &ctx.accounts.maker_token_account_a,
        &ctx.accounts.vault,
        &ctx.accounts.token_mint_a,
        &ctx.accounts.maker,
        &ctx.accounts.token_program,
        &token_a_offered_amount
    )
}
pub fn save_offer(
    ctx: Context<MakeOffer>,
    id:u64,
    token_b_wanted_amount:u64,
) -> Result<()> {
    ctx.accounts.offer.set_inner(OfferState { 
        id: (id), 
        maker: (ctx.accounts.maker.key()), 
        token_mint_a: (ctx.accounts.token_mint_a.key()), 
        token_mint_b: (ctx.accounts.token_mint_a.key()), 
        token_b_wanted_amount: (token_b_wanted_amount), 
        offer_bump: (ctx.bumps.offer) 
    }
    );
    Ok(())
}
