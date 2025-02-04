use crate::OfferState;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use super::transfer_token;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=token_mint_a,
        associated_token::authority=taker,
        associated_token::token_program=token_program,
    )]
    pub taker_token_account_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint=token_mint_b,
        associated_token::authority=taker,
        associated_token::token_program=token_program,
    )]
    pub taker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer=maker,
        associated_token::mint=token_mint_a,
        associated_token::authority=maker,
        associated_token::token_program=token_program,
    )]
    pub maker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        close=maker,               // Closes `offer` and sends rent-exempt SOL to `maker`
        has_one=maker,              // Ensures `offer.maker == maker.key()`
        has_one=token_mint_a,     // Ensures `offer.token_mint_a == token_mint_a.key()`
        has_one=token_mint_b,      // Ensures `offer.token_mint_b == token_mint_b.key()
        seeds=[b"offer", maker.key.as_ref(),offer.id.to_le_bytes().as_ref()],
        bump=offer.offer_bump
    )]
    offer: Account<'info, OfferState>,

    #[account(
        mut,
        associated_token::mint=token_mint_a,
        associated_token::authority=offer,
        associated_token::token_program=token_program,
    )]
    vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn send_wanted_tokens_to_maker(ctx: &Context<TakeOffer>) -> Result<()> {
    transfer_token(
        &ctx.accounts.taker_token_account_b,
        &ctx.accounts.maker_token_account_b,
        &ctx.accounts.token_mint_b,
        &ctx.accounts.taker,
        &ctx.accounts.token_program,
        &ctx.accounts.offer.token_b_wanted_amount,
    )
}

pub fn withdraw_and_close_vault(ctx: Context<TakeOffer>) -> Result<()> {
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts_options = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.taker_token_account_a.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
    };

    let seeds = &[
        b"offer",
        ctx.accounts.maker.key.as_ref(),
        &ctx.accounts.offer.id.to_le_bytes()[..],
        &[ctx.accounts.offer.offer_bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts_options, signer_seeds);
    transfer_checked(
        cpi_ctx,
        ctx.accounts.vault.amount,
        ctx.accounts.token_mint_a.decimals,
    );

    //CLOSING OFFER
    let close_account_options = CloseAccount {
        account: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
        destination: ctx.accounts.maker.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let close_ctx = CpiContext::new_with_signer(cpi_program, close_account_options, signer_seeds);

    close_account(close_ctx)
}
