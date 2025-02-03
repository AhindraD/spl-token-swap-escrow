use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::OfferState;

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program=token_program,
    )]
    pub tokan_mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program=token_program,
    )]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
    mut,
    associated_token::mint=tokan_mint_a,
    associated_token::authority=maker,
    associated_token::token_program=token_program,   
    )]
    pub maker_token_account_a:InterfaceAccount<'info,TokenAccount>,

    pub offer:Account<'info,OfferState>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn send_offered_tokens_to_vault(ctx: Context<MakeOffer>) -> Result<()> {
    Ok(())
}
pub fn save_offer(ctx: Context<MakeOffer>) -> Result<()> {
    Ok(())
}
